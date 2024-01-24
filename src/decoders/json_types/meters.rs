// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum Meters {
    MetersAsInteger(i32),
    MetersAsFloat(f32),
    #[default]
    None,
}

impl Serialize for Meters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Meters::MetersAsInteger(meters) => serializer.serialize_i32(*meters),
            Meters::MetersAsFloat(meters) => serializer.serialize_f32(*meters),
            Meters::None => serializer.serialize_none(),
        }
    }
}

impl From<i32> for Meters {
    fn from(meters: i32) -> Self {
        Self::MetersAsInteger(meters)
    }
}

impl From<f32> for Meters {
    fn from(meters: f32) -> Self {
        Self::MetersAsFloat(meters)
    }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Meters::MetersAsInteger(meters) => write!(f, "{meters} meters"),
            Meters::MetersAsFloat(meters) => write!(f, "{meters} meters"),
            Meters::None => write!(f, "None"),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum NauticalMiles {
    NauticalMilesAsInteger(i32),
    NauticalMilesAsFloat(f32),
    NauticalMilesAsFloat64(f64),
    #[default]
    None,
}

impl Serialize for NauticalMiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NauticalMiles::NauticalMilesAsInteger(miles) => serializer.serialize_i32(*miles),
            NauticalMiles::NauticalMilesAsFloat(miles) => serializer.serialize_f32(*miles),
            NauticalMiles::NauticalMilesAsFloat64(miles) => serializer.serialize_f64(*miles),
            NauticalMiles::None => serializer.serialize_none(),
        }
    }
}

impl From<i32> for NauticalMiles {
    fn from(miles: i32) -> Self {
        Self::NauticalMilesAsInteger(miles)
    }
}

impl From<f32> for NauticalMiles {
    fn from(miles: f32) -> Self {
        Self::NauticalMilesAsFloat(miles)
    }
}

impl From<f64> for NauticalMiles {
    fn from(miles: f64) -> Self {
        Self::NauticalMilesAsFloat64(miles)
    }
}

impl fmt::Display for NauticalMiles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            NauticalMiles::NauticalMilesAsInteger(miles) => write!(f, "{miles} nm"),
            NauticalMiles::NauticalMilesAsFloat(miles) => write!(f, "{miles} nm"),
            NauticalMiles::NauticalMilesAsFloat64(miles) => write!(f, "{miles} nm"),
            NauticalMiles::None => write!(f, "None"),
        }
    }
}
