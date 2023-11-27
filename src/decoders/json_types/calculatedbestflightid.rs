// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum CalculatedBestFlightID {
    String(String),
}

impl Default for CalculatedBestFlightID {
    fn default() -> Self {
        Self::String("".to_string())
    }
}

impl fmt::Display for CalculatedBestFlightID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatedBestFlightID::String(flight_id) => write!(f, "{}", flight_id.trim()),
        }
    }
}

impl fmt::Debug for CalculatedBestFlightID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatedBestFlightID::String(flight_id) => fmt::Display::fmt(&flight_id.trim(), f),
        }
    }
}
