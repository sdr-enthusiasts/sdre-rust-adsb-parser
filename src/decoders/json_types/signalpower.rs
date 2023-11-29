// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum SignalPower {
    Decibels(f32),
    #[default]
    None,
}

impl From<f32> for SignalPower {
    fn from(speed: f32) -> Self {
        Self::Decibels(speed)
    }
}

impl fmt::Display for SignalPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            SignalPower::Decibels(speed) => write!(f, "{} dB", *speed as u32),
            SignalPower::None => write!(f, "None"),
        }
    }
}
