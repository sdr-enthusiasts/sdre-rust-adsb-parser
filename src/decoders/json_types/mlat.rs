// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]

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
    NACp,
    Sil,
    SilType,
    #[default]
    None,
}

impl TryFrom<String> for MLATFields {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        match field.as_str() {
            "altitude" => Ok(MLATFields::Altitude),
            "gs" => Ok(MLATFields::GroundSpeed),
            "track" => Ok(MLATFields::Track),
            "baro_rate" => Ok(MLATFields::BaroRate),
            "lat" => Ok(MLATFields::Latitude),
            "lon" => Ok(MLATFields::Longitude),
            "nic" => Ok(MLATFields::NIC),
            "rc" => Ok(MLATFields::RC),
            "nac_v" => Ok(MLATFields::NACv),
            "nac_p" => Ok(MLATFields::NACp),
            "sil" => Ok(MLATFields::Sil),
            "sil_type" => Ok(MLATFields::SilType),
            "none" => Ok(MLATFields::None),
            _ => Err(format!("Invalid MLAT field: {}", field)),
        }
    }
}

// impl From<String> for MLATFields {
//     fn from(field: String) -> Self {
//         match field.as_str() {
//             "altitude" => MLATFields::Altitude,
//             "gs" => MLATFields::GroundSpeed,
//             "track" => MLATFields::Track,
//             "baro_rate" => MLATFields::BaroRate,
//             "lat" => MLATFields::Latitude,
//             "lon" => MLATFields::Longitude,
//             "nic" => MLATFields::NIC,
//             "rc" => MLATFields::RC,
//             "nac_v" => MLATFields::NACv,
//             "nac_p" => MLATFields::NACp,
//             "sil" => MLATFields::Sil,
//             "sil_type" => MLATFields::SilType,
//             "none" => MLATFields::None,
//             _ => panic!("Invalid MLAT field: {}", field),
//         }
//     }
// }

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
            MLATFields::NACp => write!(f, "NACp"),
            MLATFields::Sil => write!(f, "SIL"),
            MLATFields::SilType => write!(f, "SIL Type"),
        }
    }
}
