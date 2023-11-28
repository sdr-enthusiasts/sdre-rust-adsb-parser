// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "f32")]
pub enum Longitude {
    Longitude(f32),
}

impl From<f32> for Longitude {
    fn from(lat: f32) -> Self {
        Longitude::Longitude(lat)
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Longitude::Longitude(lat) => {
                // format the latitude in DMS
                let lat_deg: f32 = lat.abs().floor();
                let lat_min: f32 = (lat.abs() - lat_deg) * 60.0;
                let lat_sec: f32 = (lat_min - lat_min.floor()) * 60.0;
                let lat_dir: &str = if *lat >= 0.0 { "E" } else { "W" };
                write!(
                    f,
                    "{:.0}° {:.0}' {:.4}\" {}",
                    lat_deg, lat_min, lat_sec, lat_dir
                )
            }
        }
    }
}
