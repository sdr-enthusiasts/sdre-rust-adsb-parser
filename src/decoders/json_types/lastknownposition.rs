// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct LastKnownPosition {
    // lat, lon, nic, rc, seen_pos
    #[serde(rename = "lat")]
    latitude: f32,
    #[serde(rename = "lon")]
    longitude: f32,
    #[serde(rename = "nic")]
    naviation_integrity_category: i32,
    #[serde(rename = "rc")]
    radius_of_containment: i32,
    #[serde(rename = "seen_pos")]
    last_time_seen: f32,
}

impl Default for LastKnownPosition {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            naviation_integrity_category: 0,
            radius_of_containment: 0,
            last_time_seen: 0.0,
        }
    }
}
