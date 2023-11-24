use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Positive / Negative
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum Sign {
    Positive = 0,
    Negative = 1,
}

impl Sign {
    #[must_use]
    pub fn value(&self) -> i16 {
        match self {
            Self::Positive => 1,
            Self::Negative => -1,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Sign::Positive => write!(f, ""),
            Sign::Negative => write!(f, "-"),
        }
    }
}
