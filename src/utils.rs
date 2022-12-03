use crate::TextRange;
use rayon::prelude::*;

pub fn split_as_char_ranges(
    text: &str
) -> Vec<TextRange> {
    text
        .par_char_indices()
        .map(|(start_index, char)| {
            let end_index = start_index + char.len_utf8();

            TextRange::new(start_index, end_index)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::split_as_char_ranges;

    #[test]
    fn test_split_as_char_ranges() {
        let text = " 你好世界, hello world ";

        let result = split_as_char_ranges(&text);

        assert_eq!(
            result.iter()
                .map(|x| x.extract(text))
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
