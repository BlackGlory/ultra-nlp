#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextRange {
    pub start_index: usize,
    pub end_index: usize,
}

impl TextRange {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }

    pub fn len(&self) -> usize {
        self.end_index - self.start_index
    }

    pub fn extract<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start_index..self.end_index]
    }
}

#[cfg(test)]
mod tests {
    use super::TextRange;

    #[test]
    fn test_len() {
        let range = TextRange::new(1, 2);

        let result = range.len();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_extract_ascii() {
        let text = "hello world";
        let range = TextRange::new(1, 2);

        let result = range.extract(text);

        assert_eq!(result, "e");
    }

    #[test]
    fn test_extract_unicode() {
        let text = "你好世界";
        let range = TextRange::new(3, 6);

        let result = range.extract(text);

        assert_eq!(result, "好");
    }
}
