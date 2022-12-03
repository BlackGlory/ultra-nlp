use crate::utils::split_as_char_ranges;
use crate::{
    Match,
    TextRange,
    BehaviorForUnmatched, UltraNLPError,
};
use crate::cedarwood::ForwardDictionary;

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_fully<T: AsRef<str>>(
    text: T,
    dict: &ForwardDictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let text = text.as_ref().to_lowercase();
    let mut results: Vec<Match> = vec![];

    let mut unconsumed_word_start_index: Option<usize> = None;
    let mut unconsumed_char_start_index: Option<usize> = None;
    let mut maximum_matched_end_index = 0;
    for start_index in 0..text.len() {
        if text.is_char_boundary(start_index) {
            let mut matched_results: Vec<Match> = vec![];
            // 注意, 虽然不知道这个Option的意义, 但Option是Some不代表matches非空.
            if let Some(matches) = dict.dat.common_prefix_search(
                &text[start_index..]
            ) {
                for (id, length) in matches {
                    let range = TextRange::new(
                        start_index,
                        start_index + length + 1
                    );
                    let value = u32::try_from(id)
                        .map_err(|err| UltraNLPError::new(err.to_string()))
                        // 没有使用负数值, 且u32的最大值大于i32, 转换应当总是能成功
                        .unwrap();

                    let result = Match::new(range, Some(value));
                    matched_results.push(result);

                    if range.end_index() > maximum_matched_end_index {
                        maximum_matched_end_index = range.end_index();
                    }
                }
            }

            let mut unmatched_results: Vec<Match> = vec![];
            match behavior_for_unmatched {
                BehaviorForUnmatched::KeepAsWords => {
                    if matched_results.len() > 0 {
                        // 将之前未消耗的word作为Match提交
                        if let Some(index) = unconsumed_word_start_index {
                            let result = Match::new(
                                TextRange::new(index, start_index),
                                None,
                            );
                            unmatched_results.push(result);
                            unconsumed_word_start_index = None;
                        }
                    } else {
                        if start_index >= maximum_matched_end_index {
                            if let None = unconsumed_word_start_index {
                                unconsumed_word_start_index = Some(start_index);
                            }
                        }
                    }
                },
                BehaviorForUnmatched::KeepAsChars => {
                    if matched_results.len() > 0{
                        // 将之前未消耗的char作为Match提交
                        if let Some(index) = unconsumed_char_start_index {
                            let result = Match::new(
                                TextRange::new(index, start_index),
                                None,
                            );
                            results.push(result);
                            unconsumed_char_start_index = None;
                        }
                    } else {
                        if start_index >= maximum_matched_end_index {
                            if let None = unconsumed_char_start_index {
                                unconsumed_char_start_index = Some(start_index);
                            }
                        }
                    }
                },
                BehaviorForUnmatched::Ignore => (),
            }

            results.append(&mut unmatched_results);
            results.append(&mut matched_results);
        }
    }
    if maximum_matched_end_index < text.len() {
        // 处理text剩余的文本
        match behavior_for_unmatched {
            BehaviorForUnmatched::KeepAsWords => {
                results.push(Match::new(
                    TextRange::new(maximum_matched_end_index, text.len()),
                    None
                ))
            },
            BehaviorForUnmatched::KeepAsChars => {
                for range in split_as_char_ranges(&text[maximum_matched_end_index..]) {
                    results.push(Match::new(
                        TextRange::new(
                            maximum_matched_end_index + range.start_index(),
                            maximum_matched_end_index + range.end_index(),
                        ),
                        None
                    ))
                }
            }
            BehaviorForUnmatched::Ignore => (),
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::BehaviorForUnmatched;
    use crate::cedarwood::{
        segment_fully,
        ForwardDictionary,
    };

    #[test]
    fn test_ignore_unmatched() {
        let text = " 南京市长江大桥, hello world ";
        let dict = ForwardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        ).unwrap();

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["南京", "南京市", "市长", "长江", "大桥"]
        );
    }

    #[test]
    fn test_keep_unmatched_as_chars() {
        let text = " 南京市长江大桥, hello world ";
        let dict = ForwardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        ).unwrap();

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsChars
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ",
                "南京",
                "南京市",
                "市长",
                "长江",
                "大桥",
                ",",
                " ",
                "h",
                "e",
                "l",
                "l",
                "o",
                " ",
                "w",
                "o",
                "r",
                "l",
                "d",
                " ",
            ]
        );
    }

    #[test]
    fn test_keep_unmatched_as_words() {
        let text = " 南京市长江大桥, hello world ";
        let dict = ForwardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        ).unwrap();

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsWords
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ",
                "南京",
                "南京市",
                "市长",
                "长江",
                "大桥",
                ", hello world ",
            ]
        );
    }

    #[test]
    fn test_value() {
        let text = " 南京市长江大桥, hello world ";
        let dict = ForwardDictionary::new(
            vec![
                "南京",
                "南京市",
                "市长",
                "长江",
                "大桥",
                "你好世界",
            ]
        ).unwrap();

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.index_of_patterns().unwrap())
                .collect::<Vec<_>>(),
            vec![
                0,
                1,
                2,
                3,
                4,
            ]
        );
    }

    #[test]
    fn test_chars_on_edge() {
        let text = "你好世界";
        let dict = ForwardDictionary::new(
            vec!["你好", "世界"]
        ).unwrap();

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["你好", "世界"]
        );
    }
}
