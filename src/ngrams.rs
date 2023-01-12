pub struct NgramsIterator {
    n: usize,
    chars: Vec<char>,
    char_index: usize,
}

pub fn ngrams(text: &str, n: usize) -> impl Iterator<Item = String> {
    NgramsIterator::new(text, n)
}

impl NgramsIterator {
    fn new(text: &str, n: usize) -> NgramsIterator {
        NgramsIterator {
            n,
            chars: text.chars().collect::<Vec<char>>(),
            char_index: 0,
        }
    }
}

impl Iterator for NgramsIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.char_index + self.n <= self.chars.len() {
            let result = self
                .chars[self.char_index..self.char_index + self.n]
                .iter()
                .collect::<String>();

            self.char_index += 1;

            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    mod ngram {
        use crate::ngrams::ngrams;

        #[test]
        fn test_1_gram() {
            let text = "你好世界";
            let size = 1;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<String>>(),
                vec!["你", "好", "世", "界"]
            );
        }

        #[test]
        fn test_2_gram() {
            let text = "你好世界";
            let size = 2;

            let result = ngrams(text, size);

            assert_eq!(
                result
                    .into_iter()
                    .collect::<Vec<String>>(),
                vec!["你好", "好世", "世界"]
            );
        }
    }
}
