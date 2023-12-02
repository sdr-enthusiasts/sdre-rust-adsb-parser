// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default, Debug)]
#[serde(from = "u8")]
pub enum SourceIntegrityLevel {
    #[default]
    Level0,
    Level1,
    Level2,
    Level3,
}

impl From<u8> for SourceIntegrityLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => Self::Level0,
            1 => Self::Level1,
            2 => Self::Level2,
            3 => Self::Level3,
            _ => Self::Level0,
        }
    }
}

impl fmt::Display for SourceIntegrityLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SourceIntegrityLevel::Level0 => write!(f, "SIL Level 0"),
            SourceIntegrityLevel::Level1 => write!(f, "SIL Level 1"),
            SourceIntegrityLevel::Level2 => write!(f, "SIL Level 2"),
            SourceIntegrityLevel::Level3 => write!(f, "SIL Level 3"),
        }
    }
}
