// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Altitude {
    U16(u16),
    U32(u32),
    String(String),
}

impl From<u16> for Altitude {
    fn from(altitude: u16) -> Self {
        Altitude::U16(altitude)
    }
}

impl From<&str> for Altitude {
    fn from(altitude: &str) -> Self {
        Altitude::String(altitude.to_string())
    }
}

impl From<u32> for Altitude {
    fn from(altitude: u32) -> Self {
        Altitude::U32(altitude)
    }
}

impl Serialize for Altitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Altitude::U16(altitude) => serializer.serialize_u16(*altitude),
            Altitude::U32(altitude) => serializer.serialize_u32(*altitude),
            Altitude::String(altitude) => serializer.serialize_str(altitude),
        }
    }
}

impl Default for Altitude {
    fn default() -> Self {
        Self::U16(0)
    }
}

impl fmt::Display for Altitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Altitude::U16(altitude) => write!(f, "{} ft", altitude),
            Altitude::U32(altitude) => write!(f, "{} ft", altitude),
            Altitude::String(_) => write!(f, "On Ground"),
        }
    }
}
