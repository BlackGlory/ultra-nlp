use crate::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    range: TextRange,
    tf_idf: Option<f64>,
}

impl Match {
    pub fn new(range: TextRange, tf_idf: Option<f64>) -> Self {
        Self { range, tf_idf }
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn tf_idf(&self) -> Option<f64> {
        self.tf_idf
    }
}
