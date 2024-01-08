// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "f32")]
pub struct Altimeter {
    /// Default units are in QNH
    altimeter: f32,
}

impl Serialize for Altimeter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f32(self.altimeter)
    }
}

impl From<f32> for Altimeter {
    fn from(altimeter: f32) -> Self {
        Self { altimeter }
    }
}

impl fmt::Display for Altimeter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} hPa", self.altimeter)
    }
}
