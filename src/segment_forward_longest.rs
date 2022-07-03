use super::{
    ForwardDictionary,
    TextRange,
    utils::split_as_char_ranges,
};

pub fn segment_forward_longest(
    text: &str,
    dict: &ForwardDictionary,
    ignore_unmatched_chars: bool,
) -> Vec<TextRange> {
    let text = text.to_lowercase();
    let mut results: Vec<TextRange> = vec![];

    let mut start_index = 0;
    while start_index < text.len() {
        if text.is_char_boundary(start_index) {
            let mut iter =
                dict.acdat.leftmost_find_iter(&text[start_index..]);

            match iter.next() {
                Some(mat) => {
                    let word = TextRange::new(
                        start_index + mat.start(),
                        start_index + mat.end()
                    );

                    if !ignore_unmatched_chars {
                        for range in split_as_char_ranges(
                            &text[start_index..word.start_index]
                        ) {
                            let char = TextRange::new(
                                start_index + range.start_index,
                                start_index + range.end_index,
                            );
                            results.push(char);
                        }
                    }

                    start_index = word.end_index;

                    results.push(word);
                },
                None => {
                    if !ignore_unmatched_chars {
                        for range in split_as_char_ranges(
                            &text[start_index..]
                        ) {
                            let char = TextRange::new(
                                start_index + range.start_index,
                                start_index + range.end_index,
                            );
                            results.push(char);
                        }
                    }

                    break;
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
    use super::{segment_forward_longest, ForwardDictionary};

    #[test]
    fn test_ignore_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_forward_longest(
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
                "和服",
            ]
        );
    }

    #[test]
    fn test_keep_unmatched_chars() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_forward_longest(
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
}
