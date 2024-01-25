// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum Heading {
    HeadingAsInteger(i32),
    HeadingAsFloat(f32),
    HeadingAsFloat64(f64),
    #[default]
    None,
}

impl Serialize for Heading {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Heading::HeadingAsInteger(heading) => serializer.serialize_i32(*heading),
            Heading::HeadingAsFloat(heading) => serializer.serialize_f32(*heading),
            Heading::HeadingAsFloat64(heading) => serializer.serialize_f64(*heading),
            Heading::None => serializer.serialize_none(),
        }
    }
}

impl From<i32> for Heading {
    fn from(heading: i32) -> Self {
        Self::HeadingAsInteger(heading)
    }
}

impl From<f32> for Heading {
    fn from(heading: f32) -> Self {
        Self::HeadingAsFloat(heading)
    }
}

impl From<f64> for Heading {
    fn from(heading: f64) -> Self {
        Self::HeadingAsFloat64(heading)
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Heading::HeadingAsInteger(heading) => write!(f, "{heading} degrees"),
            Heading::HeadingAsFloat(heading) => write!(f, "{} degrees", *heading),
            Heading::HeadingAsFloat64(heading) => write!(f, "{} degrees", *heading),
            Heading::None => write!(f, "None"),
        }
    }
}
