// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[serde(try_from = "u8")]
pub enum GeometricVerticalAccuracy {
    UnknownOrGreaterThan150m,
    LessThanEqual150m,
    LessThanEqual45m,
    LessThanEqual10m,
}

impl Serialize for GeometricVerticalAccuracy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            GeometricVerticalAccuracy::UnknownOrGreaterThan150m => serializer.serialize_u8(0),
            GeometricVerticalAccuracy::LessThanEqual150m => serializer.serialize_u8(1),
            GeometricVerticalAccuracy::LessThanEqual45m => serializer.serialize_u8(2),
            GeometricVerticalAccuracy::LessThanEqual10m => serializer.serialize_u8(3),
        }
    }
}

impl From<u8> for GeometricVerticalAccuracy {
    fn from(value: u8) -> Self {
        match value {
            1 => GeometricVerticalAccuracy::LessThanEqual150m,
            2 => GeometricVerticalAccuracy::LessThanEqual45m,
            3 => GeometricVerticalAccuracy::LessThanEqual10m,
            _ => GeometricVerticalAccuracy::UnknownOrGreaterThan150m,
        }
    }
}

impl fmt::Display for GeometricVerticalAccuracy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometricVerticalAccuracy::UnknownOrGreaterThan150m => {
                write!(f, "Unknown or greater than 150m")
            }
            GeometricVerticalAccuracy::LessThanEqual150m => write!(f, "Less than or equal to 150m"),
            GeometricVerticalAccuracy::LessThanEqual45m => write!(f, "Less than or equal to 45m"),
            GeometricVerticalAccuracy::LessThanEqual10m => write!(f, "Less than or equal to 10m"),
        }
    }
}
