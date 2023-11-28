// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum BaroRate {
    I32(i32),
}

impl Default for BaroRate {
    fn default() -> Self {
        Self::I32(0)
    }
}

impl fmt::Display for BaroRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaroRate::I32(altitude) => write!(f, "{} fpm", altitude),
        }
    }
}
