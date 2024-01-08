// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Clone, PartialEq, PartialOrd, Default, Debug)]
#[serde(try_from = "u8")]
pub enum SourceIntegrityLevel {
    #[default]
    Level0,
    Level1,
    Level2,
    Level3,
}

impl Serialize for SourceIntegrityLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SourceIntegrityLevel::Level0 => serializer.serialize_u8(0),
            SourceIntegrityLevel::Level1 => serializer.serialize_u8(1),
            SourceIntegrityLevel::Level2 => serializer.serialize_u8(2),
            SourceIntegrityLevel::Level3 => serializer.serialize_u8(3),
        }
    }
}

impl TryFrom<u8> for SourceIntegrityLevel {
    type Error = String;

    fn try_from(level: u8) -> Result<Self, Self::Error> {
        match level {
            0 => Ok(Self::Level0),
            1 => Ok(Self::Level1),
            2 => Ok(Self::Level2),
            3 => Ok(Self::Level3),
            _ => Err(format!(
                "SIL should be a value between 0 and 3, inclusive. Found {}",
                level
            )),
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
