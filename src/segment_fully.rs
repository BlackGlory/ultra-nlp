use super::{
    StandardDictionary,
    TextRange,
};

pub fn segment_fully(
    text: &str,
    dict: &StandardDictionary
) -> Vec<TextRange> {
    let text = text.to_lowercase();

    dict.acdat
        .find_overlapping_iter(text)
        .map(|mat| TextRange::new(mat.start(), mat.end()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{segment_fully, StandardDictionary};

    #[test]
    fn test() {
        let text = " 南京市长江大桥, hello world ";
        let dict = StandardDictionary::new(
            vec!["南京", "南京市", "市长", "长江", "大桥", "你好世界"]
        );

        let result = segment_fully(text, &dict);

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
}
