use crate::utils::split_as_char_ranges;
use crate::{
    Match,
    TextRange,
    BehaviorForUnmatched,
};
use crate::cedarwood::ForwardDictionary;

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_forward_longest<T: AsRef<str>>(
    text: T,
    dict: &ForwardDictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let text = text
        .as_ref()
    .to_lowercase();

    let mut results: Vec<Match> = vec![];

    let mut unconsumed_word_start_index: Option<usize> = None;
    let mut unconsumed_char_start_index: Option<usize> = None;
    let mut maximum_matched_end_index = 0;
    let mut start_index = 0;
    while start_index < text.len() {
        if text.is_char_boundary(start_index) {
            let mut next_start_index = start_index + 1;

            let mut matched_results: Vec<Match> = vec![];
            // 注意, 虽然不知道这个Option的意义, 但Option是Some不代表matches非空.
            if let Some(matches) = dict.dat.common_prefix_search(
                &text[start_index..]
            ) {
                let longest_match: Option<&(i32, usize)> = matches
                    .iter()
                    .reduce(| longest, current | {
                        let (_, longest_length) = longest;
                        let (_, current_length) = current;
                        if current_length > longest_length {
                            current
                        } else {
                            longest
                        }
                    });

                if let Some((id, length)) = longest_match{
                    let range = TextRange::new(
                        start_index,
                        start_index + length + 1
                    );
                    let value = dict.i32_value_to_f64_value
                        .get(*id as usize)
                        .map(|x| *x);

                    let result = Match::new(range, value);
                    matched_results.push(result);

                    next_start_index = range.end_index();
                    maximum_matched_end_index = range.end_index();
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
                    if matched_results.len() > 0 {
                        // 将之前未消耗的char作为Match提交
                        if let Some(index) = unconsumed_char_start_index {
                            let result = Match::new(
                                TextRange::new(index, start_index),
                                None,
                            );
                            unmatched_results.push(result);
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

            start_index = next_start_index;
        } else {
            start_index += 1;
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
        segment_forward_longest,
        ForwardDictionary,
    };

    #[test]
    fn test_ignore_unmatched() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_forward_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "和服",]
        );
    }

    #[test]
    fn test_keep_unmatched_as_chars() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_forward_longest(
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
                "商品",
                "和服",
                "务",
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
            ],
        );
    }

    #[test]
    fn test_keep_unmatched_as_words() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_forward_longest(
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
                "商品",
                "和服",
                "务, hello world ",
            ],
        );
    }

    #[test]
    fn test_value() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new_with_values(
            vec![
                ("商品", 0f64),
                ("和服", 1f64),
                ("服务", 2f64),
                ("你好世界", 3f64),
            ]
        ).unwrap();

        let result = segment_forward_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.value().unwrap())
                .collect::<Vec<_>>(),
            vec![0f64, 1f64]
        );
    }
}
