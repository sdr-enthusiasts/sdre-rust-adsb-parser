// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub enum SecondsAgo {
    SecondsAsF32(f32),
    #[default]
    None,
}

impl Serialize for SecondsAgo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SecondsAgo::SecondsAsF32(seconds) => serializer.serialize_f32(seconds),
            SecondsAgo::None => serializer.serialize_none(),
        }
    }
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
