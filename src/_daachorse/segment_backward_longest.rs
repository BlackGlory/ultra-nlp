use crate::{
    Match,
    TextRange,
    BehaviorForUnmatched,
    utils::split_as_char_ranges,
};
use crate::daachorse::BackwardDictionary;

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_backward_longest<T: AsRef<str>>(
    text: T,
    dict: &BackwardDictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let text = text
        .as_ref()
        .to_lowercase()
        .chars()
        .rev()
        .collect::<String>();

    let mut results: Vec<Match> = vec![];

    let mut start_index = 0;
    while start_index < text.len() {
        if text.is_char_boundary(start_index) {
            let mut iter = dict.acdat.leftmost_find_iter(&text[start_index..]);

            match iter.next() {
                Some(mat) => {
                    let real_mat_start_index = start_index + mat.start();
                    let real_mat_end_index = start_index + mat.end();
                    let result = Match::new(
                        TextRange::new(
                            text.len() - real_mat_end_index,
                            text.len() - real_mat_start_index,
                        ),
                        Some(mat.value())
                    );

                    if mat.start() > 0 {
                        // 处理匹配结果之前的文本
                        match behavior_for_unmatched {
                            BehaviorForUnmatched::Ignore => {},
                            BehaviorForUnmatched::KeepAsWords => {
                                let result = Match::new(
                                    TextRange::new(
                                        text.len() - (start_index + mat.start()),
                                        text.len() - start_index,
                                    ),
                                    None,
                                );
                                results.push(result);
                            },
                            BehaviorForUnmatched::KeepAsChars => {
                                for range in split_as_char_ranges(
                                    &text[start_index..start_index + mat.start()]
                                ) {
                                    let result = Match::new(
                                        TextRange::new(
                                            text.len() - (start_index + range.end_index()),
                                            text.len() - (start_index + range.start_index()),
                                        ),
                                        None,
                                    );
                                    results.push(result);
                                }
                            },
                        }
                    }

                    start_index = real_mat_end_index;

                    results.push(result);
                }
                None => {
                    // 处理text剩余的文本
                    match behavior_for_unmatched {
                        BehaviorForUnmatched::Ignore => {},
                        BehaviorForUnmatched::KeepAsWords => {
                            results.push(
                                Match::new(
                                    TextRange::new(
                                        0,
                                        text.len() - start_index
                                    ),
                                    None,
                                )
                            );
                        },
                        BehaviorForUnmatched::KeepAsChars => {
                            for range in split_as_char_ranges(
                                &text[start_index..]
                            ) {
                                let result = Match::new(
                                    TextRange::new(
                                        text.len() - (start_index + range.end_index()),
                                        text.len() - (start_index + range.start_index()),
                                    ),
                                    None,
                                );
                                results.push(result);
                            }
                        },
                    }

                    start_index += 1;
                }
            }
        } else {
            start_index += 1;
        }
    }

    results.reverse();

    results
}

#[cfg(test)]
mod tests {
    use crate::BehaviorForUnmatched;
    use crate::daachorse::{
        segment_backward_longest,
        BackwardDictionary,
    };

    #[test]
    fn test_ignore_unmatched() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_backward_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["商品", "服务",]
        );
    }

    #[test]
    fn test_keep_unmatched_as_chars() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_backward_longest(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsChars
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ",
                "商品",
                "和",
                "服务",
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
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        ).unwrap();

        let result = segment_backward_longest(
            text,
            &dict,
            BehaviorForUnmatched::KeepAsWords
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec![
                " ",
                "商品",
                "和",
                "服务",
                ", hello world ",
            ]
        );
    }

    #[test]
    fn test_value() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec![
                "商品",
                "和服",
                "服务",
                "你好世界",
            ]
        ).unwrap();

        let result = segment_backward_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.index_of_patterns().unwrap())
                .collect::<Vec<_>>(),
            vec![0, 2]
        );
    }

    #[test]
    fn test_chars_on_edge() {
        let text = "你好世界";
        let dict = BackwardDictionary::new(
            vec!["你好", "世界"]
        ).unwrap();

        let result = segment_backward_longest(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.range().extract(text))
                .collect::<Vec<_>>(),
            vec!["你好", "世界"]
        );
    }
}
