use daachorse::charwise::{
    CharwiseDoubleArrayAhoCorasick as DoubleArrayAhoCorasick,
    CharwiseDoubleArrayAhoCorasickBuilder as DoubleArrayAhoCorasickBuilder,
};
use daachorse::MatchKind;
use crate::{
    UltraNLPResult,
    UltraNLPError
};

#[derive(Clone)]
pub struct StandardDictionary {
    pub(crate) acdat: DoubleArrayAhoCorasick<u32>,
}

#[derive(Clone)]
pub struct ForwardDictionary {
    pub(crate) acdat: DoubleArrayAhoCorasick<u32>,
}

#[derive(Clone)]
pub struct BackwardDictionary {
    pub(crate) acdat: DoubleArrayAhoCorasick<u32>,
}

impl StandardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;

        let acdat = create_acdat_with_values(
            patterns_with_values,
            MatchKind::Standard
        )?;

        Ok(Self { acdat, })
    }
}

impl ForwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;

        let acdat = create_acdat_with_values(
            patterns_with_values,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self { acdat })
    }
}

impl BackwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns_with_values = prepare_patterns_for_dictionary(patterns)?;

        let reversed_patterns_with_values = patterns_with_values
            .into_iter()
            .map(|(pattern, value)| {
                let pattern = pattern
                    .chars()
                    .rev()
                    .collect::<String>();

                (pattern, value)
            })
            .collect::<Vec<_>>();

        let acdat = create_acdat_with_values(
            reversed_patterns_with_values,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self { acdat })
    }
}

fn create_acdat_with_values<
    T: AsRef<str>,
    I: IntoIterator<Item = (T, u32)>
>(
    patterns_with_values: I,
    match_kind: MatchKind,
) -> UltraNLPResult<DoubleArrayAhoCorasick<u32>> {
    let acdat = DoubleArrayAhoCorasickBuilder::new()
        .match_kind(match_kind)
        .build_with_values(patterns_with_values);

    acdat.map_err(|err| UltraNLPError::new(err.to_string()))
}

fn prepare_patterns_for_dictionary<
    T: AsRef<str>,
    I: IntoIterator<Item = T>
>(
    patterns: I,
) -> UltraNLPResult<Vec<(String, u32)>> {
    let patterns_with_values = patterns
        .into_iter()
        .enumerate()
        .map(|(i, pattern)| -> Result<(String, u32), _>{
            let pattern = pattern
                .as_ref()
                .to_lowercase();

            let value = u32::try_from(i)
                .map_err(|err| UltraNLPError::new(err.to_string()))?;

            Ok((pattern, value))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(patterns_with_values)
}

#[cfg(test)]
mod tests {
    mod standard_dictionary {
        use crate::daachorse::StandardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert!(StandardDictionary::new(patterns).is_err());
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            StandardDictionary::new(patterns).unwrap();
        }

        #[test]
        fn test_same_patterns() {
            let patterns: Vec<&str> = vec!["foo", "FOO"];

            assert!(StandardDictionary::new(patterns).is_err());
        }
    }

    mod forward_dictionary {
        use crate::daachorse::ForwardDictionary;

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
        use crate::daachorse::BackwardDictionary;

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
