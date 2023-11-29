// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum Meters {
    MetersAsInteger(i32),
    MetersAsFloat(f32),
    #[default]
    None,
}

impl From<i32> for Meters {
    fn from(speed: i32) -> Self {
        Self::MetersAsInteger(speed)
    }
}

impl From<f32> for Meters {
    fn from(speed: f32) -> Self {
        Self::MetersAsFloat(speed)
    }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Meters::MetersAsInteger(speed) => write!(f, "{} meters", speed),
            Meters::MetersAsFloat(speed) => write!(f, "{} meters", speed),
            Meters::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum NauticalMiles {
    NauticalMilesAsInteger(i32),
    NauticalMilesAsFloat(f32),
    #[default]
    None,
}

impl From<i32> for NauticalMiles {
    fn from(speed: i32) -> Self {
        Self::NauticalMilesAsInteger(speed)
    }
}

impl From<f32> for NauticalMiles {
    fn from(speed: f32) -> Self {
        Self::NauticalMilesAsFloat(speed)
    }
}

impl fmt::Display for NauticalMiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            NauticalMiles::NauticalMilesAsInteger(speed) => write!(f, "{} nm", speed),
            NauticalMiles::NauticalMilesAsFloat(speed) => write!(f, "{} nm", speed),
            NauticalMiles::None => write!(f, "None"),
        }
    }
}
