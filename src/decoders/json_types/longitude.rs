// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "f64")]
pub struct Longitude {
    pub longitude: f64,
}

impl From<f64> for Longitude {
    fn from(lat: f64) -> Self {
        Longitude { longitude: lat }
    }
}

impl Default for Longitude {
    fn default() -> Self {
        Longitude { longitude: 0.0 }
    }
}

impl Serialize for Longitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.longitude)
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the longitude in DMS
        let lon_deg: f64 = self.longitude.abs().floor();
        let lon_min: f64 = (self.longitude.abs() - lon_deg) * 60.0;
        let lon_sec: f64 = (lon_min - lon_min.floor()) * 60.0;
        let lon_dir: &str = if self.longitude >= 0.0 { "E" } else { "W" };
        write!(f, "{lon_deg:.0}° {lon_min:.0}' {lon_sec:.4}\" {lon_dir}")
    }
}
