use crate::{
    Match,
    StandardDictionary,
    TextRange,
    BehaviorForUnmatched,
    utils::split_as_char_ranges,
};

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
                        dict.value_to_tf_idf
                            .get(mat.value())
                            .map(|x| *x),
                    )
                })
                .collect()
        },
        | BehaviorForUnmatched::KeepAsChars
        | BehaviorForUnmatched::KeepAsWords => {
            let mut results: Vec<Match> = vec![];

            let mut last_match_end_index = 0;
            for mat in dict.acdat.find_overlapping_iter(&text) {
                if mat.start() > last_match_end_index {
                    // 处理匹配结果之前的文本
                    match behavior_for_unmatched {
                        BehaviorForUnmatched::Ignore => panic!("Rust is stupid."),
                        BehaviorForUnmatched::KeepAsWords => {
                            results.push(
                                Match::new(
                                    TextRange::new(last_match_end_index, mat.start()),
                                    None
                                )
                            );
                        },
                        BehaviorForUnmatched::KeepAsChars => {
                            for range in split_as_char_ranges(
                                &text[last_match_end_index..mat.start()]
                            ) {
                                results.push(Match::new(range, None));
                            }
                        },
                    }

                    last_match_end_index = mat.end();
                } else {
                    if mat.end() > last_match_end_index {
                        last_match_end_index = mat.end();
                    }
                }

                let result = Match::new(
                    TextRange::new(mat.start(), mat.end()),
                    dict.value_to_tf_idf
                        .get(mat.value())
                        .map(|x| *x),
                );
                results.push(result);
            }
            if last_match_end_index < text.len() {
                // 处理text剩余的文本
                match behavior_for_unmatched {
                    BehaviorForUnmatched::KeepAsWords => {
                        results.push(Match::new(
                            TextRange::new(last_match_end_index, text.len()),
                            None
                        ))
                    },
                    BehaviorForUnmatched::KeepAsChars => {
                        for range in split_as_char_ranges(
                            &text[last_match_end_index..]
                        ) {
                            let result = Match::new(
                                TextRange::new(
                                    last_match_end_index + range.start_index(),
                                    last_match_end_index + range.end_index(),
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
    use crate::{
        segment_fully,
        StandardDictionary,
        BehaviorForUnmatched,
    };

    #[test]
    fn test_ignore_unmatched() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        );

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
        );

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
        );

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
    fn test_tf_idf() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new_with_tf_idf(
            vec![
                ("南京", 0f64),
                ("南京市", 1f64),
                ("市长", 2f64),
                ("长江", 3f64),
                ("大桥", 4f64),
                ("你好世界", 5f64),
            ]
        );

        let result = segment_fully(
            text,
            &dict,
            BehaviorForUnmatched::Ignore
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.tf_idf().unwrap())
                .collect::<Vec<_>>(),
            vec![
                0f64,
                1f64,
                2f64,
                3f64,
                4f64
            ]
        );
    }
}
