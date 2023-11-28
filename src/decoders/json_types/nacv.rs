// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "u8")]
pub enum NavigationAccuracyVelocity {
    #[default]
    Category0,
    Category1,
    Category2,
    Category3,
    Category4,
}

impl From<u8> for NavigationAccuracyVelocity {
    fn from(nacv: u8) -> Self {
        match nacv {
            0 => NavigationAccuracyVelocity::Category0,
            1 => NavigationAccuracyVelocity::Category1,
            2 => NavigationAccuracyVelocity::Category2,
            3 => NavigationAccuracyVelocity::Category3,
            4 => NavigationAccuracyVelocity::Category4,
            _ => NavigationAccuracyVelocity::Category0,
        }
    }
}

impl fmt::Display for NavigationAccuracyVelocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NavigationAccuracyVelocity::Category0 => write!(f, "Category 0 or unknown"),
            NavigationAccuracyVelocity::Category1 => write!(f, "Category 1: < 10 m/s"),
            NavigationAccuracyVelocity::Category2 => write!(f, "Category 2: < 3 m/s"),
            NavigationAccuracyVelocity::Category3 => write!(f, "Category 3: < 1 m/s"),
            NavigationAccuracyVelocity::Category4 => write!(f, "Category 4: < 0.3 m/s"),
        }
    }
}
