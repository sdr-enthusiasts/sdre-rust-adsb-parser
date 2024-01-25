// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum Speed {
    Knots(f32),
    #[default]
    None,
}

impl Speed {
    #[must_use]
    pub fn get_speed(&self) -> f64 {
        match self {
            Self::Knots(speed) => f64::from(*speed),
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
            Speed::Knots(speed) => serializer.serialize_f32(speed),
            Speed::None => serializer.serialize_none(),
        }
    }
}

impl From<f64> for Speed {
    fn from(speed: f64) -> Self {
        Self::Knots(speed as f32)
    }
}

impl From<f32> for Speed {
    fn from(speed: f32) -> Self {
        Self::Knots(speed)
    }
}

impl Speed {
    #[must_use]
    pub fn as_meters(&self) -> f32 {
        match self {
            Speed::Knots(speed) => *speed * 0.514_444,
            Speed::None => 0.0,
        }
    }

    #[must_use]
    pub fn as_knots(&self) -> f32 {
        match self {
            Speed::Knots(speed) => *speed,
            Speed::None => 0.0,
        }
    }

    #[must_use]
    pub fn display_as_knots(&self) -> String {
        match self {
            Speed::Knots(speed) => format!("{speed} knots"),
            Speed::None => "None".to_string(),
        }
    }

    #[must_use]
    pub fn display_as_meters(&self) -> String {
        match self {
            Speed::Knots(speed) => format!("{} m/min", speed * 0.514_444),
            Speed::None => "None".to_string(),
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Speed::Knots(speed) => write!(f, "{} knots", *speed),
            Speed::None => write!(f, "None"),
        }
    }
}
