pub struct NgramsIterator<'a> {
    n: usize,
    text: &'a str,
    index: usize,
}

pub fn ngrams(text: &str, n: usize) -> impl Iterator<Item = &str> {
    NgramsIterator::new(text, n)
}

impl<'a> NgramsIterator<'a> {
    fn new(text: &str, n: usize) -> NgramsIterator {
        NgramsIterator {
            text,
            n,
            index: 0,
        }
    }
}

impl<'a> Iterator for NgramsIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let start_index: usize = self.index;

        if start_index < self.text.len() {
            let mut end_index: usize = start_index + 1;
            let mut first_char_end_index: Option<usize> = None;
            let mut chars: usize = 0;

            while end_index < self.text.len() {
                if self.text.is_char_boundary(end_index) {
                    chars += 1;

                    if first_char_end_index.is_none() {
                        first_char_end_index = Some(end_index);
                    }
                }

                if chars == self.n {
                    self.index = first_char_end_index.unwrap();

                    return Some(&self.text[start_index..end_index])
                } else {
                    end_index += 1;
                }
            }

            self.index = end_index;

            Some(&self.text[start_index..end_index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    mod ngrams {
        use crate::ngrams::ngrams;

        #[test]
        fn test_english_chars_1_gram() {
            let text = "foo";
            let size = 1;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["f", "o", "o"]
            );
        }

        #[test]
        fn test_english_chars_2_gram() {
            let text = "foo";
            let size = 2;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["fo", "oo"]
            );
        }

        #[test]
        fn test_chinese_chars_1_gram() {
            let text = "你好世界";
            let size = 1;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["你", "好", "世", "界"]
            );
        }

        #[test]
        fn test_chineses_chars_2_gram() {
            let text = "你好世界";
            let size = 2;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["你好", "好世", "世界"]
            );
        }

        #[test]
        fn test_emoji_chars_1_gram() {
            let text = "🌱🌿🌲🌳";
            let size = 1;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["🌱", "🌿", "🌲", "🌳"]
            );
        }

        #[test]
        fn test_emoji_chars_2_gram() {
            let text = "🌱🌿🌲🌳";
            let size = 2;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["🌱🌿", "🌿🌲", "🌲🌳"]
            );
        }

        #[test]
        fn test_mix_chars_1_gram() {
            let text = "f🌱你o";
            let size = 1;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["f", "🌱", "你", "o"]
            );
        }

        #[test]
        fn test_mix_chars_2_gram() {
            let text = "f🌱你o";
            let size = 2;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<&str>>(),
                vec!["f🌱", "🌱你", "你o"]
            );
        }
    }
}
