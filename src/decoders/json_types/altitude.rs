// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Altitude {
    I32(i32),
    String(String),
}

impl Altitude {
    pub fn as_meters(&self) -> f32 {
        match self {
            Altitude::I32(altitude) => *altitude as f32 * 0.3048,
            Altitude::String(_) => 0.0,
        }
    }

    pub fn as_feet(&self) -> f32 {
        match self {
            Altitude::I32(altitude) => *altitude as f32,
            Altitude::String(_) => 0.0,
        }
    }

    pub fn display_as_feet(&self) -> String {
        match self {
            Altitude::I32(altitude) => format!("{} ft", altitude),
            Altitude::String(_) => "On Ground".to_string(),
        }
    }

    pub fn display_as_meters(&self) -> String {
        match self {
            Altitude::I32(altitude) => format!("{} m", *altitude as f32 * 0.3048),
            Altitude::String(_) => "On Ground".to_string(),
        }
    }
}

impl Default for Altitude {
    fn default() -> Self {
        Self::I32(0)
    }
}

impl fmt::Display for Altitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Altitude::I32(altitude) => write!(f, "{} ft", altitude),
            Altitude::String(_) => write!(f, "On Ground"),
        }
    }
}
