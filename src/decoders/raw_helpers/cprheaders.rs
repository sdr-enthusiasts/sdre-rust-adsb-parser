use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Even / Odd
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, Default)]
#[deku(type = "u8", bits = "1")]
pub enum CPRFormat {
    #[default]
    Even = 0,
    Odd = 1,
}

impl fmt::Display for CPRFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CPRFormat::Even => write!(f, "even"),
            CPRFormat::Odd => write!(f, "odd"),
        }
    }
}
