use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// SPI Condition
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, Default)]
#[deku(type = "u8", bits = "2")]
pub enum SurveillanceStatus {
    #[default]
    NoCondition = 0,
    PermanentAlert = 1,
    TemporaryAlert = 2,
    SPICondition = 3,
}

impl fmt::Display for SurveillanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SurveillanceStatus::NoCondition => write!(f, "no condition"),
            SurveillanceStatus::PermanentAlert => write!(f, "permanent alert"),
            SurveillanceStatus::TemporaryAlert => write!(f, "temporary alert"),
            SurveillanceStatus::SPICondition => write!(f, "SPI condition"),
        }
    }
}
