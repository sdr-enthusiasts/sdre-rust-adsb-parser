// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum SecondsAgo {
    SecondsAsF32(f32),
    #[default]
    None,
}

impl From<f32> for SecondsAgo {
    fn from(seconds: f32) -> Self {
        Self::SecondsAsF32(seconds)
    }
}

impl fmt::Display for SecondsAgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SecondsAsF32(seconds) => write!(f, "{} seconds ago", seconds),
            Self::None => write!(f, "None"),
        }
    }
}
