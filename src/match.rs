use super::TextRange;

#[derive(Clone, Debug, PartialEq)]
pub struct Match {
    pub range: TextRange,
    pub tf_idf: Option<f64>,
}

impl Match {
    pub fn new(range: TextRange, tf_idf: Option<f64>) -> Self {
        Self { range, tf_idf }
    }
}
