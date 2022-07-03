use super::{
    StandardDictionary,
    TextRange,
    utils::split_as_char_ranges,
};

pub fn segment_fully(
    text: &str,
    dict: &StandardDictionary,
    ignore_unmatched_chars: bool,
) -> Vec<TextRange> {
    let text = text.to_lowercase();

    if ignore_unmatched_chars {
        dict.acdat
            .find_overlapping_iter(&text)
            .map(|mat| TextRange::new(mat.start(), mat.end()))
            .collect()
    } else {
        let mut results: Vec<TextRange> = vec![];

        let mut last_match_end_index = 0;
        for mat in dict.acdat.find_overlapping_iter(&text) {
            if mat.start() > last_match_end_index {
                for range in split_as_char_ranges(
                    &text[last_match_end_index..mat.start()]
                ) { 
                    results.push(range);
                }
                last_match_end_index = mat.end();
            } else {
                if mat.end() > last_match_end_index {
                    last_match_end_index = mat.end();
                }
            }

            let range = TextRange::new(mat.start(), mat.end());
            results.push(range);
        }
        if last_match_end_index < text.len() {
            for range in split_as_char_ranges(
                &text[last_match_end_index..]
            ) {
                let char = TextRange::new(
                    last_match_end_index + range.start_index,
                    last_match_end_index + range.end_index,
                );
                results.push(char);
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
        let dict = StandardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        );

        let result = segment_fully(text, &dict, true);

        assert_eq!(
            result
                .iter()
                .map(|x| x.get(text))
                .collect::<Vec<_>>(),
            vec![
                "南京",
                "南京市",
                "市长",
                "长江",
                "大桥"
            ]
        );
    }

    #[test]
    fn test_keep_unmatched_chars() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        );

        let result = segment_fully(text, &dict, false);

        assert_eq!(
            result
                .iter()
                .map(|x| x.get(text))
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
}
