// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "f32")]
pub struct Longitude {
    longitude: f32,
}

impl From<f32> for Longitude {
    fn from(lat: f32) -> Self {
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
        serializer.serialize_f32(self.longitude)
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the longitude in DMS
        let lon_deg: f32 = self.longitude.abs().floor();
        let lon_min: f32 = (self.longitude.abs() - lon_deg) * 60.0;
        let lon_sec: f32 = (lon_min - lon_min.floor()) * 60.0;
        let lon_dir: &str = if self.longitude >= 0.0 { "E" } else { "W" };
        write!(
            f,
            "{:.0}° {:.0}' {:.4}\" {}",
            lon_deg, lon_min, lon_sec, lon_dir
        )
    }
}
