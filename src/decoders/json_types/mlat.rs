// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum MLATFields {
    Altitude,
    GroundSpeed,
    Track,
    BaroRate,
    Latitude,
    Longitude,
    NIC,
    RC,
    NACv,
    NACp,
    Sil,
    SilType,
    #[default]
    None,
}

impl Serialize for MLATFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            MLATFields::Altitude => serializer.serialize_str("altitude"),
            MLATFields::GroundSpeed => serializer.serialize_str("gs"),
            MLATFields::Track => serializer.serialize_str("track"),
            MLATFields::BaroRate => serializer.serialize_str("baro_rate"),
            MLATFields::Latitude => serializer.serialize_str("lat"),
            MLATFields::Longitude => serializer.serialize_str("lon"),
            MLATFields::NIC => serializer.serialize_str("nic"),
            MLATFields::RC => serializer.serialize_str("rc"),
            MLATFields::NACv => serializer.serialize_str("nac_v"),
            MLATFields::NACp => serializer.serialize_str("nac_p"),
            MLATFields::Sil => serializer.serialize_str("sil"),
            MLATFields::SilType => serializer.serialize_str("sil_type"),
            MLATFields::None => serializer.serialize_str("none"),
        }
    }
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
            _ => Err(format!("Invalid MLAT field: {field}")),
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
            MLATFields::NACp => write!(f, "NACp"),
            MLATFields::Sil => write!(f, "SIL"),
            MLATFields::SilType => write!(f, "SIL Type"),
        }
    }
}
