// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "i32")]
pub struct BaroRate {
    baro_rate: i32,
}

impl Serialize for BaroRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.baro_rate)
    }
}

impl From<i16> for BaroRate {
    fn from(baro_rate: i16) -> Self {
        Self {
            baro_rate: i32::from(baro_rate),
        }
    }
}

impl From<i32> for BaroRate {
    fn from(baro_rate: i32) -> Self {
        Self { baro_rate }
    }
}

impl fmt::Display for BaroRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ft/min", self.baro_rate)
    }
}
