use crate::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    range: TextRange,
    value: Option<f64>,
}

impl Match {
    pub fn new(range: TextRange, value: Option<f64>) -> Self {
        Self { range, value }
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }
}
