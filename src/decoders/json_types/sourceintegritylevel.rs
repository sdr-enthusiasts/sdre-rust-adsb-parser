// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum SourceIntegrityLevelType {
    #[default]
    Unknown,
    PerSample,
    PerHour,
}

impl Serialize for SourceIntegrityLevelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SourceIntegrityLevelType::PerSample => serializer.serialize_str("persample"),
            SourceIntegrityLevelType::PerHour => serializer.serialize_str("perhour"),
            SourceIntegrityLevelType::Unknown => serializer.serialize_str("unknown"),
        }
    }
}

impl TryFrom<String> for SourceIntegrityLevelType {
    type Error = String;

    fn try_from(source_integrity_level: String) -> Result<Self, Self::Error> {
        match source_integrity_level.as_str() {
            "persample" => Ok(SourceIntegrityLevelType::PerSample),
            "perhour" => Ok(SourceIntegrityLevelType::PerHour),
            "unknown" => Ok(SourceIntegrityLevelType::Unknown),
            _ => Err(format!(
                "SIL should be unknown, persample, perhour, inclusive. Found {}",
                source_integrity_level
            )),
        }
    }
}

impl fmt::Display for SourceIntegrityLevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SourceIntegrityLevelType::PerSample => write!(f, "Per Sample"),
            SourceIntegrityLevelType::PerHour => write!(f, "Per Hour"),
            SourceIntegrityLevelType::Unknown => write!(f, "Unknown"),
        }
    }
}
