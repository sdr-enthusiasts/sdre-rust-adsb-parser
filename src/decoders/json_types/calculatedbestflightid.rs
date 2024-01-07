// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
#[serde(from = "String")]
pub struct CalculatedBestFlightID {
    flight_id: String,
}

impl Default for CalculatedBestFlightID {
    fn default() -> Self {
        Self {
            flight_id: "".to_string(),
        }
    }
}

impl From<String> for CalculatedBestFlightID {
    fn from(flight_id: String) -> Self {
        Self { flight_id }
    }
}

impl fmt::Display for CalculatedBestFlightID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.flight_id)
    }
}
