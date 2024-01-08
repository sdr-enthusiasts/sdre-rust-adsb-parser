// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum TiSB {
    BaroRate,
    Callsign,
    Altitude,
    AltGeom,
    GroundSpeed,
    Track,
    GeomRate,
    Latitude,
    Longitude,
    NIC,
    RC, // TODO: rename these
    NACp,
    NACv,
    SIL,
    SILType,
    #[default]
    None,
}

impl Serialize for TiSB {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            TiSB::BaroRate => serializer.serialize_str("baro_rate"),
            TiSB::Callsign => serializer.serialize_str("callsign"),
            TiSB::Altitude => serializer.serialize_str("altitude"),
            TiSB::AltGeom => serializer.serialize_str("alt_geom"),
            TiSB::GroundSpeed => serializer.serialize_str("gs"),
            TiSB::Track => serializer.serialize_str("track"),
            TiSB::GeomRate => serializer.serialize_str("geom_rate"),
            TiSB::Latitude => serializer.serialize_str("lat"),
            TiSB::Longitude => serializer.serialize_str("lon"),
            TiSB::NIC => serializer.serialize_str("nic"),
            TiSB::RC => serializer.serialize_str("rc"),
            TiSB::NACp => serializer.serialize_str("nac_p"),
            TiSB::NACv => serializer.serialize_str("nac_v"),
            TiSB::SIL => serializer.serialize_str("sil"),
            TiSB::SILType => serializer.serialize_str("sil_type"),
            TiSB::None => serializer.serialize_str("None"),
        }
    }
}

impl fmt::Display for TiSB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TiSB::BaroRate => write!(f, "Baro Rate"),
            TiSB::Callsign => write!(f, "Callsign"),
            TiSB::Altitude => write!(f, "Altitude"),
            TiSB::AltGeom => write!(f, "Altitude Geometric"),
            TiSB::GroundSpeed => write!(f, "Ground Speed"),
            TiSB::Track => write!(f, "Track"),
            TiSB::GeomRate => write!(f, "Geom Rate"),
            TiSB::Latitude => write!(f, "Latitude"),
            TiSB::Longitude => write!(f, "Longitude"),
            TiSB::NIC => write!(f, "NIC"),
            TiSB::RC => write!(f, "RC"),
            TiSB::NACp => write!(f, "NACp"),
            TiSB::NACv => write!(f, "NACv"),
            TiSB::SIL => write!(f, "SIL"),
            TiSB::SILType => write!(f, "SIL Type"),
            TiSB::None => write!(f, "None"),
        }
    }
}

impl TryFrom<String> for TiSB {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        match field.as_str() {
            "None" => Ok(TiSB::None),
            "baro_rate" => Ok(TiSB::BaroRate),
            "callsign" => Ok(TiSB::Callsign),
            "altitude" => Ok(TiSB::Altitude),
            "alt_geom" => Ok(TiSB::AltGeom),
            "gs" => Ok(TiSB::GroundSpeed),
            "track" => Ok(TiSB::Track),
            "geom_rate" => Ok(TiSB::GeomRate),
            "lat" => Ok(TiSB::Latitude),
            "lon" => Ok(TiSB::Longitude),
            "nic" => Ok(TiSB::NIC),
            "rc" => Ok(TiSB::RC),
            "nac_p" => Ok(TiSB::NACp),
            "nac_v" => Ok(TiSB::NACv),
            "sil" => Ok(TiSB::SIL),
            "sil_type" => Ok(TiSB::SILType),
            _ => Err(format!("Invalid TiSB field: {}", field)),
        }
    }
}
