// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]

pub enum MLATFields {
    Altitude,
    GroundSpeed,
    Track,
    BaroRate,
    Latitude,
    Longitude,
    NIC,
    RC, // TODO: rename this field
    NACv,
    #[default]
    None,
}

impl From<String> for MLATFields {
    fn from(field: String) -> Self {
        match field.as_str() {
            "altitude" => MLATFields::Altitude,
            "gs" => MLATFields::GroundSpeed,
            "track" => MLATFields::Track,
            "baro_rate" => MLATFields::BaroRate,
            "lat" => MLATFields::Latitude,
            "lon" => MLATFields::Longitude,
            "nic" => MLATFields::NIC,
            "rc" => MLATFields::RC,
            "nac_v" => MLATFields::NACv,
            //_ => panic!("Unknown MLAT field: {}", field),
            _ => panic!("Unknown MLAT field: {}", field),
        }
    }
}

impl fmt::Display for MLATFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MLATFields::Altitude => write!(f, "Altitude"),
            MLATFields::GroundSpeed => write!(f, "Ground Speed"),
            MLATFields::Track => write!(f, "Track"),
            MLATFields::BaroRate => write!(f, "Barometric Rate"),
            MLATFields::Latitude => write!(f, "Latitude"),
            MLATFields::Longitude => write!(f, "Longitude"),
            MLATFields::NIC => write!(f, "NIC"),
            MLATFields::RC => write!(f, "RC"),
            MLATFields::NACv => write!(f, "NACv"),
            MLATFields::None => write!(f, "None"),
        }
    }
}
