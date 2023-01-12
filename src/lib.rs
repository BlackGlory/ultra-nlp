mod _daachorse;
mod _cedarwood;
mod _hashmap;
mod r#match;
mod text_range;
mod behavior_for_unmatched;
mod utils;
mod error;
mod ngrams;
mod extract_consecutive_chinese_chars;

pub use r#match::*;
pub use text_range::*;
pub use behavior_for_unmatched::*;
pub use error::*;
pub use ngrams::*;
pub use extract_consecutive_chinese_chars::*;

pub mod daachorse {
    pub use crate::_daachorse::*;
}

pub mod cedarwood {
    pub use crate::_cedarwood::*;
}

pub mod hashmap {
    pub use crate::_hashmap::*;
}
