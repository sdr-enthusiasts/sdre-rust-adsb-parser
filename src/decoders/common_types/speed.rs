// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum Speed {
    KnotsAsF32(f32),
    KnotsAsF64(f64),
    #[default]
    None,
}

impl Speed {
    #[must_use]
    pub fn get_speed(&self) -> f64 {
        match self {
            Self::KnotsAsF32(speed) => f64::from(*speed),
            Self::KnotsAsF64(speed) => *speed,
            Self::None => 0.0,
        }
    }
}

impl Serialize for Speed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Speed::KnotsAsF32(speed) => serializer.serialize_f32(speed),
            Speed::KnotsAsF64(speed) => serializer.serialize_f64(speed),
            Speed::None => serializer.serialize_none(),
        }
    }
}

impl From<f64> for Speed {
    fn from(speed: f64) -> Self {
        Self::KnotsAsF64(speed)
    }
}

impl From<f32> for Speed {
    fn from(speed: f32) -> Self {
        Self::KnotsAsF32(speed)
    }
}

impl Speed {
    #[must_use]
    pub fn as_meters(&self) -> f64 {
        match self {
            Speed::KnotsAsF32(speed) => f64::from(*speed) * 0.514_444,
            Speed::KnotsAsF64(speed) => *speed * 0.514_444,
            Speed::None => 0.0,
        }
    }

    #[must_use]
    pub fn as_knots(&self) -> f64 {
        match self {
            Speed::KnotsAsF32(speed) => f64::from(*speed),
            Speed::KnotsAsF64(speed) => *speed,
            Speed::None => 0.0,
        }
    }

    #[must_use]
    pub fn display_as_knots(&self) -> String {
        match self {
            Speed::KnotsAsF32(speed) => format!("{speed} knots"),
            Speed::KnotsAsF64(speed) => format!("{speed} knots"),
            Speed::None => "None".to_string(),
        }
    }

    #[must_use]
    pub fn display_as_meters(&self) -> String {
        match self {
            Speed::KnotsAsF32(speed) => format!("{} m/min", speed * 0.514_444),
            Speed::KnotsAsF64(speed) => format!("{} m/min", speed * 0.514_444),
            Speed::None => "None".to_string(),
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Speed::KnotsAsF32(speed) => write!(f, "{} knots", *speed),
            Speed::KnotsAsF64(speed) => write!(f, "{} knots", *speed),
            Speed::None => write!(f, "None"),
        }
    }
}
