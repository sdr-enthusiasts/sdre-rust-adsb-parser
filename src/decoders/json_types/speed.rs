// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum Speed {
    Knots(f32),
    #[default]
    None,
}

impl From<f32> for Speed {
    fn from(speed: f32) -> Self {
        Self::Knots(speed)
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // cast to u32 to remove the decimal
            Speed::Knots(speed) => write!(f, "{} knots", *speed as u32),
            Speed::None => write!(f, "None"),
        }
    }
}
