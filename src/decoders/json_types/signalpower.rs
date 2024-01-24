// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum SignalPower {
    Decibels(f32),
    #[default]
    None,
}

impl Serialize for SignalPower {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SignalPower::Decibels(rssi) => serializer.serialize_f32(rssi),
            SignalPower::None => serializer.serialize_none(),
        }
    }
}

impl From<f32> for SignalPower {
    fn from(rssi: f32) -> Self {
        Self::Decibels(rssi)
    }
}

impl fmt::Display for SignalPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            SignalPower::Decibels(rssi) => write!(f, "{rssi:.1} dB"),
            SignalPower::None => write!(f, "None"),
        }
    }
}
