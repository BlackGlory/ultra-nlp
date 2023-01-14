use lazy_static::lazy_static;
use regex::Regex;

pub fn extract_consecutive_chinese_chars(text: &str) -> impl Iterator<Item = &str>{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\p{Script=Han}]+").unwrap();
    }

    let result = RE
        .split(text)
        .filter(|x| !x.is_empty());

    result
}

#[cfg(test)]
mod tests {
    mod extract_consecutive_chinese_chars {
        use crate::extract_consecutive_chinese_chars::extract_consecutive_chinese_chars;

        #[test]
        fn test_extract_chinese_chars() {
            let text = "foo中文bar字符baz";

            let result = extract_consecutive_chinese_chars(text);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["中文", "字符"]
            );
        }
    }
}
