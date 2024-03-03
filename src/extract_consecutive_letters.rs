use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_consecutive_letters(text: &str) -> impl Iterator<Item = &str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\p{General_Category=Letter}+").unwrap();
    }

    let result = RE.find_iter(text)
      .map(|x| x.as_str());

    result
}

#[cfg(test)]
mod tests {
    mod extract_consecutive_letters {
        use crate::extract_consecutive_letters::extract_consecutive_letters;

        #[test]
        fn test_extract_chinese_chars() {
            let text = "foo中文，bar,字符baz";

            let result = extract_consecutive_letters(text);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["foo中文", "bar", "字符baz"]
            );
        }
    }
}
