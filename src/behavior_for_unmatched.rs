#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BehaviorForUnmatched {
    Ignore,
    KeepAsChars,
    KeepAsWords,
}
