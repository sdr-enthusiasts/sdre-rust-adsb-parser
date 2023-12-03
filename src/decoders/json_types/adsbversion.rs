// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "u8")]
pub enum ADSBVersion {
    Version0,
    Version1,
    Version2,
    Version3,
    Version4,
    Version5,
    Version6,
    Version7,
}

impl From<u8> for ADSBVersion {
    fn from(version: u8) -> Self {
        match version {
            0 => ADSBVersion::Version0,
            1 => ADSBVersion::Version1,
            2 => ADSBVersion::Version2,
            3 => ADSBVersion::Version3,
            4 => ADSBVersion::Version4,
            5 => ADSBVersion::Version5,
            6 => ADSBVersion::Version6,
            7 => ADSBVersion::Version7,
            _ => panic!(
                "Invalid ADSB Version. Should be a value between 0 - 7, inclusive. Found {}",
                version
            ), // TODO: propagate error
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
        }
    }
}
