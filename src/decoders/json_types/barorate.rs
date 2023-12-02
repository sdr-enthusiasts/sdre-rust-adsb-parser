// Copyright 2023 Frederick Clausen II

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

impl BaroRate {
    pub fn as_meters(&self) -> f32 {
        self.baro_rate as f32 * 0.00508
    }

    pub fn as_feet(&self) -> f32 {
        self.baro_rate as f32
    }

    pub fn display_as_feet(&self) -> String {
        format!("{} ft/min", self.baro_rate)
    }

    pub fn display_as_meters(&self) -> String {
        format!("{} m/min", self.baro_rate as f32 * 0.00508)
    }
}

impl fmt::Display for BaroRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ft/min", self.baro_rate)
    }
}
