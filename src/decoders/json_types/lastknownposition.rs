// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

use super::{
    latitude::Latitude, longitude::Longitude, meters::Meters, nacp::NavigationIntegrityCategory,
    secondsago::SecondsAgo,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(deny_unknown_fields)]
pub struct LastKnownPosition {
    // lat, lon, nic, rc, seen_pos
    #[serde(rename = "lat")]
    latitude: Latitude,
    #[serde(rename = "lon")]
    longitude: Longitude,
    #[serde(rename = "nic")]
    naviation_integrity_category: NavigationIntegrityCategory,
    #[serde(rename = "rc")]
    radius_of_containment: Meters,
    #[serde(rename = "seen_pos")]
    last_time_seen: SecondsAgo,
}

impl fmt::Display for LastKnownPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Last Known Position:")?;
        write!(f, "\tLatitude: {}", self.latitude)?;
        write!(f, "\tLongitude: {}", self.longitude)?;
        write!(f, "\tNIC: {}", self.naviation_integrity_category)?;
        write!(f, "\tRadius of Containment: {}", self.radius_of_containment)?;
        write!(f, "\tLast Seen: {}", self.last_time_seen)
    }
}
