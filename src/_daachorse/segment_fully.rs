use crate::{
    Match,
    TextRange,
    BehaviorForUnmatched,
    utils::split_as_char_ranges,
};
use crate::daachorse::StandardDictionary;

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_fully<T: AsRef<str>>(
    text: T,
    dict: &StandardDictionary,
    behavior_for_unmatched: BehaviorForUnmatched,
) -> Vec<Match> {
    let text = text.as_ref().to_lowercase();

    match behavior_for_unmatched {
        BehaviorForUnmatched::Ignore => {
            dict.acdat
                .find_overlapping_iter(&text)
                .map(|mat| {
                    Match::new(
                        TextRange::new(mat.start(), mat.end()),
                        Some(mat.value())
                    )
                })
                .collect()
        },
        | BehaviorForUnmatched::KeepAsChars
        | BehaviorForUnmatched::KeepAsWords => {
            let mut results: Vec<Match> = vec![];

            let mut maximum_matched_end_index = 0;
            for mat in dict.acdat.find_overlapping_iter(&text) {
                if mat.start() > maximum_matched_end_index {
                    // 处理匹配结果之前的文本
                    match behavior_for_unmatched {
                        BehaviorForUnmatched::Ignore => panic!("Rust is stupid."),
                        BehaviorForUnmatched::KeepAsWords => {
                            results.push(
                                Match::new(
                                    TextRange::new(maximum_matched_end_index, mat.start()),
                                    None
                                )
                            );
                        },
                        BehaviorForUnmatched::KeepAsChars => {
                            for range in split_as_char_ranges(
                                &text[maximum_matched_end_index..mat.start()]
                            ) {
                                results.push(Match::new(range, None));
                            }
                        },
                    }

                    // mat.end() > last_match_end_index
                    maximum_matched_end_index = mat.end();
                } else {
                    if mat.end() > maximum_matched_end_index {
                        maximum_matched_end_index = mat.end();
                    }
                }

                let result = Match::new(
                    TextRange::new(mat.start(), mat.end()),
                    Some(mat.value())
                );
                results.push(result);
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
                        for range in split_as_char_ranges(
                            &text[maximum_matched_end_index..]
                        ) {
                            let result = Match::new(
                                TextRange::new(
                                    maximum_matched_end_index + range.start_index(),
                                    maximum_matched_end_index + range.end_index(),
                                ),
                                None,
                            );
                            results.push(result);
                        }
                    },
                    BehaviorForUnmatched::Ignore => panic!("Rust is stupid."),
                }
            }

            results
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BehaviorForUnmatched;
    use crate::daachorse::{
        segment_fully,
        StandardDictionary,
    };

    #[test]
    fn test_ignore_unmatched() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new(
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
        let dict = StandardDictionary::new(
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
        let dict = StandardDictionary::new(
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
        let dict = StandardDictionary::new(
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
        let dict = StandardDictionary::new(
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
