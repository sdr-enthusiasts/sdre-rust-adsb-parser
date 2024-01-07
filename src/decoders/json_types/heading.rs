// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(untagged)]
pub enum Heading {
    HeadingAsInteger(i32),
    HeadingAsFloat(f32),
    #[default]
    None,
}

impl From<i32> for Heading {
    fn from(speed: i32) -> Self {
        Self::HeadingAsInteger(speed)
    }
}

impl From<f32> for Heading {
    fn from(speed: f32) -> Self {
        Self::HeadingAsFloat(speed)
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Heading::HeadingAsInteger(speed) => write!(f, "{} degrees", speed),
            Heading::HeadingAsFloat(speed) => write!(f, "{} degrees", *speed as u32),
            Heading::None => write!(f, "None"),
        }
    }
}
