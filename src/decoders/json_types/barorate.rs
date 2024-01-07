// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "i32")]
pub struct BaroRate {
    baro_rate: i32,
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
