use crate::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    range: TextRange,
    index_of_patterns: Option<u32>,
}

impl Match {
    pub fn new(range: TextRange, index_of_patterns: Option<u32>) -> Self {
        Self { range, index_of_patterns }
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn index_of_patterns(&self) -> Option<u32> {
        self.index_of_patterns
    }
}
