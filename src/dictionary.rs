use daachorse::charwise::{
    CharwiseDoubleArrayAhoCorasick as DoubleArrayAhoCorasick,
    CharwiseDoubleArrayAhoCorasickBuilder as DoubleArrayAhoCorasickBuilder,
};
use daachorse::MatchKind;

pub struct StandardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
}

pub struct ForwardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
}

pub struct BackwardDictionary {
    pub acdat: DoubleArrayAhoCorasick,
}

impl StandardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns = patterns
            .iter()
            .map(|x| x.to_lowercase());

        let acdat = DoubleArrayAhoCorasickBuilder::new()
            .match_kind(MatchKind::Standard)
            .build(patterns)
            .unwrap();

        Self { acdat }
    }
}

impl ForwardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns = patterns
            .iter()
            .map(|x| x.to_lowercase());

        let acdat = DoubleArrayAhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(patterns)
            .unwrap();

        Self { acdat }
    }
}


impl BackwardDictionary {
    pub fn new(patterns: Vec<&str>) -> Self {
        let reversed_patterns = patterns
            .iter()
            .map(|x| x.to_lowercase())
            .map(|x| x.chars().rev().collect::<String>());

        let acdat = DoubleArrayAhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(reversed_patterns)
            .unwrap();

        Self { acdat }
    }
}
