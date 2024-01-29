// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f64")]
pub struct Altimeter {
    /// Default units are in QNH
    altimeter: f64,
}

impl Serialize for Altimeter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.altimeter)
    }
}

impl From<f64> for Altimeter {
    fn from(altimeter: f64) -> Self {
        Self { altimeter }
    }
}

impl fmt::Display for Altimeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} hPa", self.altimeter)
    }
}
