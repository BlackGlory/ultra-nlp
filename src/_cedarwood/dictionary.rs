use cedarwood::Cedar;
use crate::{
    UltraNLPResult,
    UltraNLPError,
};

#[derive(Clone)]
pub struct ForwardDictionary {
    pub(crate) dat: Cedar,
    pub(crate) value_to_tf_idf: Vec<f64>,
}

#[derive(Clone)]
pub struct BackwardDictionary {
    pub(crate) dat: Cedar,
    pub(crate) value_to_tf_idf: Vec<f64>,
}

impl ForwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns = prepare_patterns_for_dictionary(patterns);

        let dat = create_dat(patterns)?;

        Ok(Self {
            dat,
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
        ) = prepare_patterns_with_tf_idf_for_dictionary(patterns_with_tf_idf)?;

        let dat = create_dat_with_values(patterns_with_values);

        Ok(Self {
            dat,
            value_to_tf_idf,
        })
    }
}

impl BackwardDictionary {
    pub fn new<T: AsRef<str>, I: IntoIterator<Item = T>>(
        patterns: I
    ) -> UltraNLPResult<Self> {
        let patterns = prepare_patterns_for_dictionary(patterns)
            .into_iter()
            .map(|x| x
                .chars()
                .rev()
                .collect::<String>()
            )
            .collect::<Vec<_>>();

        let dat = create_dat(patterns)?;

        Ok(Self {
            dat,
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
        ) = prepare_patterns_with_tf_idf_for_dictionary(patterns_with_tf_idf)?;

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

        Ok(Self {
            dat,
            value_to_tf_idf,
        })
    }
}

fn create_dat<
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
>(patterns: I) -> UltraNLPResult<Cedar> {
    let key_values: Vec<(String, i32)> = patterns
        .into_iter()
        .enumerate()
        .map(|(i, key)| -> Result<_, _> {
            let key = key.as_ref().to_owned();
            let value = i32::try_from(i)
                .map_err(|err| UltraNLPError::new(err.to_string()))?;

            Ok((key, value))
        })
        .collect::<Result<_, _>>()?;
    let key_values: Vec<(&str, i32)> = key_values
        .iter()
        .map(|(key, value)| (key.as_str(), *value))
        .collect::<Vec<_>>();

    let mut dat = Cedar::new();
    dat.build(&key_values);

    Ok(dat)
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
>(patterns: I) -> Vec<String> {
    let patterns = patterns
        .into_iter()
        .map(|x| x.as_ref().to_lowercase())
        .collect();
    
    patterns
}

fn prepare_patterns_with_tf_idf_for_dictionary<
    T: AsRef<str>,
    I: IntoIterator<Item = (T, f64)>
>(
    patterns_with_tf_idf: I,
) -> UltraNLPResult<(Vec<(String, i32)>, Vec<f64>)> {
    let mut value_to_tf_idf: Vec<f64> = vec![];
    let patterns = patterns_with_tf_idf
        .into_iter()
        .map(|(pattern, tf_idf)| -> Result<(String, i32), _>{
            let pattern = pattern.as_ref().to_lowercase();

            value_to_tf_idf.push(tf_idf);
            let value = i32::try_from(value_to_tf_idf.len() - 1)
                .map_err(|err| UltraNLPError::new(err.to_string()))?;

            Ok((pattern, value))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((patterns, value_to_tf_idf))
}

#[cfg(test)]
mod tests {
    mod forward_dictionary {
        use crate::cedarwood::ForwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            ForwardDictionary::new(patterns).unwrap();
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            ForwardDictionary::new(patterns).unwrap();
        }
    }

    mod backward_dictionary {
        use crate::cedarwood::BackwardDictionary;

        #[test]
        fn test_empty_patterns() {
            let patterns: Vec<&str> = vec![];

            BackwardDictionary::new(patterns).unwrap();
        }

        #[test]
        fn test_patterns() {
            let patterns: Vec<&str> = vec!["foo", "bar"];

            BackwardDictionary::new(patterns).unwrap();
        }
    }
}
