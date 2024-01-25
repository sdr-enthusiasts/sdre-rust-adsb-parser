// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::decoders::helpers::time::get_time_as_f64;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f64")]
pub enum SecondsAgo {
    TimeStamp(f64),
    #[default]
    None,
}

impl SecondsAgo {
    #[must_use]
    pub fn now() -> Self {
        // get the current unix timestamp
        let seconds = get_time_as_f64() as f64;
        Self::TimeStamp(seconds)
    }
}

impl Serialize for SecondsAgo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SecondsAgo::TimeStamp(seconds) => {
                let seconds = get_time_as_f64() - seconds;
                serializer.serialize_f64(seconds)
            }
            SecondsAgo::None => serializer.serialize_none(),
        }
    }
}

impl From<f64> for SecondsAgo {
    fn from(seconds: f64) -> Self {
        // get the current unix timestamp

        let seconds = get_time_as_f64() - seconds;
        Self::TimeStamp(seconds)
    }
}

impl fmt::Display for SecondsAgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimeStamp(seconds) => {
                let seconds = get_time_as_f64() - seconds;
                write!(f, "{seconds}")
            }
            Self::None => write!(f, "None"),
        }
    }
}
