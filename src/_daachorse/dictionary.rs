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
    pub(crate) value_to_tf_idf: Vec<f64>,
}

#[derive(Clone)]
pub struct ForwardDictionary {
    pub(crate) acdat: DoubleArrayAhoCorasick<u32>,
    pub(crate) value_to_tf_idf: Vec<f64>,
}

#[derive(Clone)]
pub struct BackwardDictionary {
    pub(crate) acdat: DoubleArrayAhoCorasick<u32>,
    pub(crate) value_to_tf_idf: Vec<f64>,
}

impl StandardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns = process_patterns(patterns);

        let acdat = create_acdat(
            patterns,
            MatchKind::Standard
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf: vec![],
        })
    }

    pub fn new_with_tf_idf<
        T: AsRef<str>,
        I: IntoIterator<Item = (T, f64)>
    >(patterns_with_tf_idf: I) -> UltraNLPResult<Self> {
        let (
            patterns_with_values,
            value_to_tf_idf
        ) = process_patterns_with_tf_idf(patterns_with_tf_idf);

        let acdat = create_acdat_with_values(
            patterns_with_values,
            MatchKind::Standard
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf,
        })
    }
}

impl ForwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns = process_patterns(patterns);

        let acdat = create_acdat(
            patterns,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf: vec![],
        })
    }

    pub fn new_with_tf_idf<
        T: AsRef<str>,
        I: IntoIterator<Item = (T, f64)>
    >(patterns_with_tf_idf: I) -> UltraNLPResult<Self> {
        let (
            patterns_with_values,
            value_to_tf_idf
        ) = process_patterns_with_tf_idf(patterns_with_tf_idf);

        let acdat = create_acdat_with_values(
            patterns_with_values,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf,
        })
    }
}

impl BackwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let reversed_patterns = process_patterns(patterns)
            .into_iter()
            .map(|x| x
                .chars()
                .rev()
                .collect::<String>()
            )
            .collect::<Vec<_>>();

        let acdat = create_acdat(
            reversed_patterns,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf: vec![],
        })
    }

    pub fn new_with_tf_idf<
        T: AsRef<str>,
        I: IntoIterator<Item = (T, f64)>
    >(patterns_with_tf_idf: I) -> UltraNLPResult<Self> {
        let (
            patterns_with_values,
            value_to_tf_idf
        ) = process_patterns_with_tf_idf(patterns_with_tf_idf);

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

        let acdat = create_acdat_with_values(
            patterns_with_values,
            MatchKind::LeftmostLongest
        )?;

        Ok(Self {
            acdat,
            value_to_tf_idf,
        })
    }
}

fn create_acdat<T: AsRef<str>, I: IntoIterator<Item = T>>(
    patterns: I,
    match_kind: MatchKind,
) -> UltraNLPResult<DoubleArrayAhoCorasick<u32>> {
    let acdat = DoubleArrayAhoCorasickBuilder::new()
        .match_kind(match_kind)
        .build(patterns);

    acdat.map_err(|err| UltraNLPError::new(err.to_string()))
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

fn process_patterns<
    T: AsRef<str>,
    I: IntoIterator<Item = T>
>(patterns: I) -> Vec<String> {
    let patterns = patterns
        .into_iter()
        .map(|x| x.as_ref().to_lowercase())
        .collect();
    
    patterns
}

fn process_patterns_with_tf_idf<
    T: AsRef<str>,
    I: IntoIterator<Item = (T, f64)>
>(
    patterns_with_tf_idf: I,
) -> (Vec<(String, u32)>, Vec<f64>) {
    let mut value_to_tf_idf: Vec<f64> = vec![];
    let patterns = patterns_with_tf_idf
        .into_iter()
        .map(|(word, tf_idf)| {
            let word = word.as_ref().to_lowercase();

            value_to_tf_idf.push(tf_idf);
            // 待TryFrom的实现稳定, 改为使用try_from, 以便在转换失败时panic.
            let value = (value_to_tf_idf.len() - 1) as u32;

            (word, value)
        })
        .collect();

    (patterns, value_to_tf_idf)
}

#[cfg(test)]
mod tests {
    mod standard_dictionary {
        use crate::daachorse::StandardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert_eq!(
                StandardDictionary::new(patterns).is_err(),
                true
            );
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            StandardDictionary::new(patterns).unwrap();
        }
    }

    mod forward_dictionary {
        use crate::daachorse::ForwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert_eq!(
                ForwardDictionary::new(patterns).is_err(),
                true
            );
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            ForwardDictionary::new(patterns).unwrap();
        }
    }

    mod backward_dictionary {
        use crate::daachorse::BackwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            assert_eq!(
                BackwardDictionary::new(patterns).is_err(),
                true
            );
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            BackwardDictionary::new(patterns).unwrap();
        }
    }
}
