// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "u8")]
pub enum NavigationAccuracyVelocity {
    #[default]
    Category0,
    Category1,
    Category2,
    Category3,
    Category4,
}

impl TryFrom<u8> for NavigationAccuracyVelocity {
    type Error = String;

    fn try_from(nacv: u8) -> Result<Self, Self::Error> {
        match nacv {
            0 => Ok(NavigationAccuracyVelocity::Category0),
            1 => Ok(NavigationAccuracyVelocity::Category1),
            2 => Ok(NavigationAccuracyVelocity::Category2),
            3 => Ok(NavigationAccuracyVelocity::Category3),
            4 => Ok(NavigationAccuracyVelocity::Category4),
            _ => Err(format!(
                "NACv should be a value between 0 and 4, inclusive. Found {}",
                nacv
            )),
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
