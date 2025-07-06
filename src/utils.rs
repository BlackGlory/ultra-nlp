use crate::TextRange;

pub fn split_as_char_ranges<'a>(
    text: &'a str
) -> impl Iterator<Item = TextRange> + 'a {
    text
        .char_indices()
        .map(|(start_index, char)| {
            let end_index = start_index + char.len_utf8();

            TextRange::new(start_index, end_index)
        })
}

#[cfg(test)]
mod tests {
    use crate::utils::split_as_char_ranges;

    #[test]
    fn test_split_as_char_ranges() {
        let text = " 你好世界, hello world ";

        let result = split_as_char_ranges(&text).collect::<Vec<_>>();

        assert_eq!(
            result
                .into_iter()
                .map(|x| x.extract(text).unwrap())
                .collect::<Vec<_>>(),
            vec![
                " ",
                "你",
                "好",
                "世",
                "界",
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
