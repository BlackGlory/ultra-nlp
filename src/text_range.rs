#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextRange {
    start_index: usize,
    end_index: usize,
}

impl TextRange {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
        }
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn end_index(&self) -> usize {
        self.end_index
    }

    pub fn len(&self) -> usize {
        self.end_index - self.start_index
    }

    pub fn extract<'a>(&self, text: &'a str) -> Option<&'a str> {
        text.get(self.start_index..self.end_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::TextRange;

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

        assert_eq!(result, Some("e"));
    }

    #[test]
    fn test_extract_unicode() {
        let text = "你好世界";
        let range = TextRange::new(3, 6);

        let result = range.extract(text);

        assert_eq!(result, Some("好"));
    }

    #[test]
    fn test_extract_invalid_text() {
        let text = "你好世界";
        let range = TextRange::new(0, 1);

        let result = range.extract(text);

        assert_eq!(result, None);
    }
}
