// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "f64")]
pub struct Latitude {
    pub latitude: f64,
}

impl From<f64> for Latitude {
    fn from(lat: f64) -> Self {
        Latitude { latitude: lat }
    }
}

impl Serialize for Latitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.latitude)
    }
}

impl Default for Latitude {
    fn default() -> Self {
        Latitude { latitude: 0.0 }
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the latitude in DMS
        let lat_deg: f64 = self.latitude.abs().floor();
        let lat_min: f64 = (self.latitude.abs() - lat_deg) * 60.0;
        let lat_sec: f64 = (lat_min - lat_min.floor()) * 60.0;
        let lat_dir: &str = if self.latitude >= 0.0 { "N" } else { "S" };
        write!(
            f,
            "{:.0}° {:.0}' {:.4}\" {}",
            lat_deg, lat_min, lat_sec, lat_dir
        )
    }
}
