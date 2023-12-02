// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]
pub enum SourceIntegrityLevelType {
    #[default]
    Unknown,
    PerSample,
    PerHour,
}

impl From<String> for SourceIntegrityLevelType {
    fn from(source_integrity_level: String) -> Self {
        match source_integrity_level.as_str() {
            "persample" => SourceIntegrityLevelType::PerSample,
            "perhour" => SourceIntegrityLevelType::PerHour,
            "unknown" => SourceIntegrityLevelType::Unknown,
            _ => panic!(
                "SIL should be unknown, persample, perhour, inclusive. Found {}",
                source_integrity_level
            ), // TODO: propagate this error
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
