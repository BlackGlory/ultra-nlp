use std::collections::HashMap;
use crate::{
    UltraNLPResult,
    UltraNLPError,
};

#[derive(Clone)]
pub struct Dictionary {
    pub(crate) map: HashMap<String, usize>,
}

impl Dictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;
        if patterns_with_values.len() == 0 {
            return Err(UltraNLPError::new("The patterns cannot be empty"));
        }

        let mut map: HashMap<String, usize> = HashMap::new();
        patterns_with_values
            .into_iter()
            .try_for_each(|(pattern, value)| {
                let result = map.insert(pattern, value);

                if let Some(_) = result {
                    Err(UltraNLPError::new("The patterns are not unique"))
                } else {
                    Ok(())
                }
            })?;

        Ok(Self { map })
    }
}

fn prepare_patterns_for_dictionary<
    T: AsRef<str>,
    I: IntoIterator<Item = T>
>(
    patterns: I,
) -> UltraNLPResult<Vec<(String, usize)>> {
    let patterns_with_values = patterns
        .into_iter()
        .enumerate()
        .map(|(index, pattern)| -> Result<(String, usize), _>{
            let pattern = pattern
                .as_ref()
                .to_lowercase();

            let value = usize::try_from(index)
                .map_err(|err| UltraNLPError::new(err.to_string()))?;

            Ok((pattern, value))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(patterns_with_values)
}

#[cfg(test)]
mod tests {
    use crate::hashmap::Dictionary;

    #[test]
    fn test_empty_patterns() {
        let patterns: Vec<&str> = vec![];

        assert!(Dictionary::new(patterns).is_err());
    }

    #[test]
    fn test_patterns() {
        let patterns: Vec<&str> = vec!["foo", "bar"];

        Dictionary::new(patterns)
            .unwrap();
    }

    #[test]
    fn test_same_patterns() {
        let patterns: Vec<&str> = vec!["foo", "FOO"];

        assert!(Dictionary::new(patterns).is_err());
    }
}
