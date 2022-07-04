use daachorse::charwise::{
    CharwiseDoubleArrayAhoCorasick as DoubleArrayAhoCorasick,
    CharwiseDoubleArrayAhoCorasickBuilder as DoubleArrayAhoCorasickBuilder,
};
use daachorse::MatchKind;

pub struct StandardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
    pub value_to_tf_idf: Vec<f64>,
}

pub struct ForwardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
    pub value_to_tf_idf: Vec<f64>,
}

pub struct BackwardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
    pub value_to_tf_idf: Vec<f64>,
}

impl StandardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns = process_patterns(patterns);

        let acdat = create_acdat(patterns, MatchKind::Standard);

        Self {
            acdat,
            value_to_tf_idf: vec![],
        }
    }

    pub fn new_with_tf_idf(patterns_with_tf_idf: Vec<(&str, f64)>) -> Self {
        let (patterns_with_values, value_to_tf_idf) =
            process_patterns_with_tf_idf(patterns_with_tf_idf);

        let acdat = create_acdat_with_values(patterns_with_values, MatchKind::Standard);

        Self {
            acdat,
            value_to_tf_idf,
        }
    }
}

impl ForwardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns = process_patterns(patterns);

        let acdat = create_acdat(patterns, MatchKind::LeftmostLongest);

        Self {
            acdat,
            value_to_tf_idf: vec![],
        }
    }

    pub fn new_with_tf_idf(patterns_with_tf_idf: Vec<(&str, f64)>) -> Self {
        let (patterns_with_values, value_to_tf_idf) =
            process_patterns_with_tf_idf(patterns_with_tf_idf);

        let acdat = create_acdat_with_values(patterns_with_values, MatchKind::LeftmostLongest);

        Self {
            acdat,
            value_to_tf_idf,
        }
    }
}

impl BackwardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let reversed_patterns = process_patterns(patterns)
            .into_iter()
            .map(|x| x.chars().rev().collect::<String>())
            .collect::<Vec<_>>();

        let acdat = create_acdat(reversed_patterns, MatchKind::LeftmostLongest);

        Self {
            acdat,
            value_to_tf_idf: vec![],
        }
    }

    pub fn new_with_tf_idf(patterns_with_tf_idf: Vec<(&str, f64)>) -> Self {
        let (patterns_with_values, value_to_tf_idf) =
            process_patterns_with_tf_idf(patterns_with_tf_idf);

        let patterns_with_values = patterns_with_values
            .into_iter()
            .map(|(pattern, value)| (pattern.chars().rev().collect::<String>(), value))
            .collect::<Vec<_>>();

        let acdat = create_acdat_with_values(patterns_with_values, MatchKind::LeftmostLongest);

        Self {
            acdat,
            value_to_tf_idf,
        }
    }
}

fn create_acdat(patterns: Vec<String>, match_kind: MatchKind) -> DoubleArrayAhoCorasick {
    let acdat = DoubleArrayAhoCorasickBuilder::new()
        .match_kind(match_kind)
        .build(patterns)
        .unwrap();
    acdat
}

fn create_acdat_with_values(
    patterns_with_values: Vec<(String, u32)>,
    match_kind: MatchKind,
) -> DoubleArrayAhoCorasick {
    let acdat = DoubleArrayAhoCorasickBuilder::new()
        .match_kind(match_kind)
        .build_with_values(patterns_with_values)
        .unwrap();
    acdat
}

fn process_patterns(patterns: Vec<&str>) -> Vec<String> {
    let patterns = patterns.into_iter().map(|x| x.to_lowercase()).collect();
    patterns
}

fn process_patterns_with_tf_idf(
    patterns_with_tf_idf: Vec<(&str, f64)>,
) -> (Vec<(String, u32)>, Vec<f64>) {
    let mut value_to_tf_idf: Vec<f64> = vec![];
    let patterns = patterns_with_tf_idf
        .into_iter()
        .map(|(word, tf_idf)| {
            value_to_tf_idf.push(tf_idf);
            (
                word.to_lowercase(),
                // 待TryFrom的实现稳定, 改为使用try_from, 以便在转换失败时panic.
                (value_to_tf_idf.len() - 1) as u32,
            )
        })
        .collect();

    (patterns, value_to_tf_idf)
}
