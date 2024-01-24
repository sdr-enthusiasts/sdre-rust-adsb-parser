// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f64")]
pub enum TimeStamp {
    TimeStampAsF64(f64),
    #[default]
    None,
}

impl TimeStamp {
    #[must_use] pub fn add_time(&self, seconds: f64) -> f64 {
        match self {
            Self::TimeStampAsF64(current_time) => *current_time + seconds,
            Self::None => 0.0,
        }
    }

    #[must_use] pub fn get_time(&self) -> f64 {
        match self {
            Self::TimeStampAsF64(current_time) => *current_time,
            Self::None => 0.0,
        }
    }
}

impl Serialize for TimeStamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::TimeStampAsF64(seconds) => serializer.serialize_f64(*seconds),
            Self::None => serializer.serialize_none(),
        }
    }
}

impl From<f64> for TimeStamp {
    fn from(seconds: f64) -> Self {
        Self::TimeStampAsF64(seconds)
    }
}

impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeStampAsF64(seconds) => {
                // Create a human readable timestamp in current timezone
                let timestamp = chrono::NaiveDateTime::from_timestamp_opt(*seconds as i64, 0);
                match timestamp {
                    None => write!(f, "Invalid timestamp"),
                    Some(timestamp) => write!(f, "{}", timestamp.format("%Y-%m-%d %H:%M:%S")),
                }
            }
            Self::None => write!(f, "None"),
        }
    }
}
