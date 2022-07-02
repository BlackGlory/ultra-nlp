use super::{
    BackwardDictionary,
    TextRange,
};

pub fn segment_backward_longest(
    text: &str,
    dict: &BackwardDictionary
) -> Vec<TextRange> {
    let text = text.chars().rev().collect::<String>();

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

                    start_index = start_index + mat.end();

                    results.insert(0, word);
                },
                None => {
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
    fn test() {
        let text = " 商品和服务, hello world ";
        let dict = BackwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_backward_longest(text, &dict);

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
}
