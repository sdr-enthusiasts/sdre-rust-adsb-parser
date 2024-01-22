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
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<Latitude>,
    #[serde(rename = "lon", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<Longitude>,
    #[serde(rename = "nic", skip_serializing_if = "Option::is_none")]
    pub naviation_integrity_category: Option<NavigationIntegrityCategory>,
    #[serde(rename = "rc", skip_serializing_if = "Option::is_none")]
    pub radius_of_containment: Option<Meters>,
    #[serde(rename = "seen_pos")]
    pub last_time_seen: SecondsAgo,
}

impl fmt::Display for LastKnownPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Last Known Position:")?;
        if let Some(latitude) = &self.latitude {
            write!(f, "\tLatitude: {}", latitude)?;
        }
        if let Some(longitude) = &self.longitude {
            write!(f, "\tLongitude: {}", longitude)?;
        }
        if let Some(naviation_integrity_category) = &self.naviation_integrity_category {
            write!(f, "\tNIC: {}", naviation_integrity_category)?;
        }
        if let Some(radius_of_containment) = &self.radius_of_containment {
            write!(f, "\tRadius of Containment: {}", radius_of_containment)?;
        }
        write!(f, "\tLast Seen: {}", self.last_time_seen)
    }
}
