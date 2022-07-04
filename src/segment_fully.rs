use super::{utils::split_as_char_ranges, Match, StandardDictionary, TextRange};

// 待generator稳定, 改为generator, 以便返回Iterator.
pub fn segment_fully(
    text: &str,
    dict: &StandardDictionary,
    ignore_unmatched_chars: bool,
) -> Vec<Match> {
    let text = text.to_lowercase();

    if ignore_unmatched_chars {
        dict.acdat
            .find_overlapping_iter(&text)
            .map(|mat| {
                Match::new(
                    TextRange::new(mat.start(), mat.end()),
                    dict.value_to_tf_idf.get(mat.value()).map(|x| *x),
                )
            })
            .collect()
    } else {
        let mut results: Vec<Match> = vec![];

        let mut last_match_end_index = 0;
        for mat in dict.acdat.find_overlapping_iter(&text) {
            if mat.start() > last_match_end_index {
                for range in split_as_char_ranges(&text[last_match_end_index..mat.start()]) {
                    results.push(Match::new(range, None));
                }
                last_match_end_index = mat.end();
            } else {
                if mat.end() > last_match_end_index {
                    last_match_end_index = mat.end();
                }
            }

            let result = Match::new(
                TextRange::new(mat.start(), mat.end()),
                dict.value_to_tf_idf.get(mat.value()).map(|x| *x),
            );
            results.push(result);
        }
        if last_match_end_index < text.len() {
            for range in split_as_char_ranges(&text[last_match_end_index..]) {
                let result = Match::new(
                    TextRange::new(
                        last_match_end_index + range.start_index,
                        last_match_end_index + range.end_index,
                    ),
                    None,
                );
                results.push(result);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::{segment_fully, StandardDictionary};

    #[test]
    fn test_ignore_unmatched_chars() {
        let text = " 南京市长江大桥, hello world ";
        let dict =
            StandardDictionary::new(vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]);

        let result = segment_fully(text, &dict, true);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
                .collect::<Vec<_>>(),
            vec!["南京", "南京市", "市长", "长江", "大桥"]
        );
    }

    #[test]
    fn test_keep_unmatched_chars() {
        let text = " 南京市长江大桥, hello world ";
        let dict =
            StandardDictionary::new(vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]);

        let result = segment_fully(text, &dict, false);

        assert_eq!(
            result
                .iter()
                .map(|x| x.range.extract(text))
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
    fn test_tf_idf() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new_with_tf_idf(vec![
            ("南京", 0f64),
            ("南京市", 1f64),
            ("市长", 2f64),
            ("长江", 3f64),
            ("大桥", 4f64),
            ("你好世界", 5f64),
        ]);

        let result = segment_fully(text, &dict, true);

        assert_eq!(
            result.iter().map(|x| x.tf_idf.unwrap()).collect::<Vec<_>>(),
            vec![0f64, 1f64, 2f64, 3f64, 4f64]
        );
    }
}
