use std::collections::HashSet;
use cedarwood::Cedar;
use crate::{
    UltraNLPResult,
    UltraNLPError,
};

#[derive(Clone)]
pub struct ForwardDictionary {
    pub(crate) dat: Cedar,
}

#[derive(Clone)]
pub struct BackwardDictionary {
    pub(crate) dat: Cedar,
}

impl ForwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T> + Clone>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;
        if patterns_with_values.len() == 0 {
            return Err(UltraNLPError::new("The patterns cannot be empty"));
        }

        let patterns = patterns_with_values
            .clone()
            .into_iter()
            .map(|(x, _)| x);
        if !is_unique(patterns) {
            return Err(UltraNLPError::new("The patterns are not unique"));
        }


        let dat = create_dat_with_values(patterns_with_values);

        Ok(Self { dat })
    }
}

impl BackwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T> + Clone>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;
        if patterns_with_values.len() == 0 {
            return Err(UltraNLPError::new("The patterns cannot be empty"));
        }

        let patterns = patterns_with_values
            .clone()
            .into_iter()
            .map(|(x, _)| x);
        if !is_unique(patterns) {
            return Err(UltraNLPError::new("The patterns are not unique"));
        }

        let patterns_with_values = patterns_with_values
            .into_iter()
            .map(|(pattern, value)| {
                let pattern = pattern
                    .chars()
                    .rev()
                    .collect::<String>();

                (pattern, value)
            })
            .collect::<Vec<_>>();

        let dat = create_dat_with_values(patterns_with_values);

        Ok(Self { dat })
    }
}

fn create_dat_with_values<
    T: AsRef<str>,
    I: IntoIterator<Item = (T, i32)>,
>(patterns_with_values: I) -> Cedar {
    let key_values: Vec<(String, i32)> = patterns_with_values.into_iter()
        .map(|(key, value)| {
            let key = key.as_ref().to_owned();
            let value = value;

            (key, value)
        })
        .collect::<Vec<_>>();
    let key_values: Vec<(&str, i32)> = key_values
        .iter()
        .map(|(key, value)| (key.as_str(), *value))
        .collect::<Vec<_>>();

    let mut dat = Cedar::new();
    dat.build(&key_values);

    dat
}

fn prepare_patterns_for_dictionary<
    T: AsRef<str>,
    I: IntoIterator<Item = T>
>(
    patterns: I,
) -> UltraNLPResult<Vec<(String, i32)>> {
    let patterns_with_values = patterns
        .into_iter()
        .enumerate()
        .map(|(index, pattern)| -> Result<(String, i32), _>{
            let pattern = pattern.as_ref().to_lowercase();

            let value = i32::try_from(index)
                .map_err(|err| UltraNLPError::new(err.to_string()))?;

            Ok((pattern, value))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(patterns_with_values)
}

fn is_unique<T: AsRef<str>, I: IntoIterator<Item = T>>(
    collection: I
) -> bool {
    let mut set = HashSet::new();
    collection
        .into_iter()
        .all(|x| set.insert(x.as_ref().to_owned()))
}

#[cfg(test)]
mod tests {
    mod forward_dictionary {
        use crate::cedarwood::ForwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert!(ForwardDictionary::new(patterns).is_err());
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            ForwardDictionary::new(patterns).unwrap();
        }

        #[test]
        fn test_same_patterns() {
            let patterns: Vec<&str> = vec!["foo", "FOO"];

            assert!(ForwardDictionary::new(patterns).is_err());
        }
    }

    mod backward_dictionary {
        use crate::cedarwood::BackwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert!(BackwardDictionary::new(patterns).is_err());
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            BackwardDictionary::new(patterns).unwrap();
        }

        #[test]
        fn test_same_patterns() {
            let patterns: Vec<&str> = vec!["foo", "FOO"];

            assert!(BackwardDictionary::new(patterns).is_err());
        }
    }
}
