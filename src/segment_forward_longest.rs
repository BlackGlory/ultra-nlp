use super::{
    ForwardDictionary,
    TextRange
};

pub fn segment_forward_longest(
    text: &str,
    dict: &ForwardDictionary
) -> Vec<TextRange> {
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

                    start_index = word.end_index;

                    results.push(word);
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
    use super::{segment_forward_longest, ForwardDictionary};

    #[test]
    fn test() {
        let text = " 商品和服务, hello world ";
        let dict = ForwardDictionary::new(
            vec!["商品", "和服", "服务", "你好世界"]
        );

        let result = segment_forward_longest(text, &dict);

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
}
