// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(try_from = "u8")]
pub enum ADSBVersion {
    Version0,
    Version1,
    Version2,
    Version3,
    Version4,
    Version5,
    Version6,
    Version7,
    Unknown,
}

impl Serialize for ADSBVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ADSBVersion::Version0 => serializer.serialize_u8(0),
            ADSBVersion::Version1 => serializer.serialize_u8(1),
            ADSBVersion::Version2 => serializer.serialize_u8(2),
            ADSBVersion::Version3 => serializer.serialize_u8(3),
            ADSBVersion::Version4 => serializer.serialize_u8(4),
            ADSBVersion::Version5 => serializer.serialize_u8(5),
            ADSBVersion::Version6 => serializer.serialize_u8(6),
            ADSBVersion::Version7 => serializer.serialize_u8(7),
            ADSBVersion::Unknown => serializer.serialize_u8(8),
        }
    }
}

impl TryFrom<u8> for ADSBVersion {
    type Error = String;

    fn try_from(field: u8) -> Result<Self, Self::Error> {
        match field {
            0 => Ok(ADSBVersion::Version0),
            1 => Ok(ADSBVersion::Version1),
            2 => Ok(ADSBVersion::Version2),
            3 => Ok(ADSBVersion::Version3),
            4 => Ok(ADSBVersion::Version4),
            5 => Ok(ADSBVersion::Version5),
            6 => Ok(ADSBVersion::Version6),
            7 => Ok(ADSBVersion::Version7),
            8 => Ok(ADSBVersion::Unknown),
            _ => Err(format!("Invalid ADSBVersion field: {}", field)),
        }
    }
}

impl fmt::Display for ADSBVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ADSBVersion::Version0 => write!(f, "ADSB Version 0"),
            ADSBVersion::Version1 => write!(f, "ADSB Version 1"),
            ADSBVersion::Version2 => write!(f, "ADSB Version 2"),
            ADSBVersion::Version3 => write!(f, "ADSB Version 3"),
            ADSBVersion::Version4 => write!(f, "ADSB Version 4"),
            ADSBVersion::Version5 => write!(f, "ADSB Version 5"),
            ADSBVersion::Version6 => write!(f, "ADSB Version 6"),
            ADSBVersion::Version7 => write!(f, "ADSB Version 7"),
            ADSBVersion::Unknown => write!(f, "ADSB Version Unknown"),
        }
    }
}
