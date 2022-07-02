pub struct TextRange {
  pub start_index: usize,
  pub end_index: usize
}

impl TextRange {
  pub fn new(start_index: usize, end_index: usize) -> TextRange {
    TextRange {
      start_index,
      end_index,
    }
  }

  pub fn len(&self) -> usize {
    self.end_index - self.start_index
  }

  pub fn get<'a>(&self, text: &'a str) -> &'a str {
    &text[self.start_index..self.end_index]
  }
}
