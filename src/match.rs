use crate::TextRange;

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    range: TextRange,

    /**
     * 在保留未匹配内容的情况下, 匹配到的内容有可能不在字典里.
     */
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

    /**
     * A shortcut to get value from map by the index of patterns
     */
    pub fn value_from<T: Copy>(&self, map: Vec<T>) -> Option<T> {
        match self.index_of_patterns {
            Some(index) => {
                match map.get(index as usize) {
                    Some(result) => Some(*result),
                    None => None
                }
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    mod value {
        use crate::{Match, TextRange};

        #[test]
        fn test_none() {
            let mat = Match::new(TextRange::new(0, 1), None);
            let map = vec![1, 2];

            let result = mat.value_from(map);

            assert!(result.is_none());
        }

        #[test]
        fn test_some() {
            let mat = Match::new(TextRange::new(0, 1), Some(1));
            let map = vec!["0", "1"];

            let result = mat.value_from(map).unwrap();

            assert_eq!(result, "1");
        }

        #[test]
        fn test_out_of_bounds() {
            let mat = Match::new(TextRange::new(0, 1), Some(2));
            let map = vec!["0", "1"];

            let result = mat.value_from(map);

            assert!(result.is_none());
        }
    }
}
