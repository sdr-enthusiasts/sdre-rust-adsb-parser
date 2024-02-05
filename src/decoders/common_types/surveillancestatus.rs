// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// SPI Condition
#[derive(
    Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Default,
)]
#[serde(from = "u8")]
#[deku(type = "u8", bits = "2")]
pub enum SurveillanceStatus {
    #[default]
    NoCondition = 0,
    PermanentAlert = 1,
    TemporaryAlert = 2,
    SPICondition = 3,
}

impl From<u8> for SurveillanceStatus {
    fn from(v: u8) -> Self {
        match v {
            1 => SurveillanceStatus::PermanentAlert,
            2 => SurveillanceStatus::TemporaryAlert,
            3 => SurveillanceStatus::SPICondition,
            _ => SurveillanceStatus::NoCondition,
        }
    }
}

impl fmt::Display for SurveillanceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SurveillanceStatus::NoCondition => write!(f, "No condition"),
            SurveillanceStatus::PermanentAlert => write!(f, "Permanent alert"),
            SurveillanceStatus::TemporaryAlert => write!(f, "Temporary alert"),
            SurveillanceStatus::SPICondition => write!(f, "SPI condition"),
        }
    }
}
