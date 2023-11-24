use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum SourceBitVerticalRate {
    GNSS = 0,
    Barometer = 1,
}

impl fmt::Display for SourceBitVerticalRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SourceBitVerticalRate::GNSS => write!(f, "GNSS"),
            SourceBitVerticalRate::Barometer => write!(f, "barometer"),
        }
    }
}
