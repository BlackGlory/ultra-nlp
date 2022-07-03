use super::{
    BackwardDictionary,
    TextRange,
    utils::split_as_char_ranges,
};

pub fn segment_backward_longest(
    text: &str,
    dict: &BackwardDictionary,
    ignore_unmatched_chars: bool,
) -> Vec<TextRange> {
    let text = text
        .to_lowercase()
        .chars()
        .rev()
        .collect::<String>();

    let mut results: Vec<TextRange> = vec![];

    let mut start_index = 0;
    while start_index < text.len() {
        if text.is_char_boundary(start_index) {
            let mut iter =
                dict.acdat.leftmost_find_iter(&text[start_index..]);

            match iter.next() {
                Some(mat) => {
                    let word = TextRange::new(
                        text.len() - (start_index + mat.end()),
                        text.len() - (start_index + mat.start())
                    );

                    if !ignore_unmatched_chars {
                        for range in split_as_char_ranges(
                            &text[start_index..start_index + mat.start()]
                        ) {
                            let char = TextRange::new(
                                text.len() - (start_index + range.end_index),
                                text.len() - (start_index + range.start_index),
                            );
                            results.insert(0, char);
                        }
                    }

                    start_index = start_index + mat.end();

                    results.insert(0, word);
                },
                None => {
                    if !ignore_unmatched_chars {
                        for range in split_as_char_ranges(
                            &text[start_index..]
                        ) {
                            let char = TextRange::new(
                                text.len() - (start_index + range.end_index),
                                text.len() - (start_index + range.start_index),
                            );
                            results.insert(0, char);
                        }
                    }

                    start_index += 1;
                }
            }
        } else {
            start_index += 1;
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::{segment_backward_longest, BackwardDictionary};

    #[test]
    fn test_ignore_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_backward_longest(
            text,
            &dict,
            true,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.get(text))
                .collect::<Vec<_>>(),
            vec![
                "商品",
                "服务",
            ]
        );
    }

    #[test]
    fn test_keep_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_backward_longest(
            text,
            &dict,
            false,
        );

        assert_eq!(
            result
                .iter()
                .map(|x| x.get(text))
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
}
