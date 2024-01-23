// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// [`ME::AircraftOperationStatus`]
#[derive(Deserialize, DekuRead, Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[serde(try_from = "u8")]
#[deku(type = "u8", bits = "2")]
pub enum SystemDesignAssurance {
    #[default]
    #[deku(id = "0")]
    UnknownOrNoSafetyEffect,
    #[deku(id = "1")]
    Minor,
    #[deku(id = "2")]
    Major,
    #[deku(id = "3")]
    Hazardous,
}

impl TryFrom<u8> for SystemDesignAssurance {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UnknownOrNoSafetyEffect),
            1 => Ok(Self::Minor),
            2 => Ok(Self::Major),
            3 => Ok(Self::Hazardous),
            // We should probably catch all in to the unknown variant
            // but technically the bit field in the raw message should only
            // ever be 2 bits so the range of values is 0-3
            _ => Err(format!("Invalid SystemDesignAssurance: {}", value)),
        }
    }
}

impl Serialize for SystemDesignAssurance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            SystemDesignAssurance::UnknownOrNoSafetyEffect => serializer.serialize_u8(0),
            SystemDesignAssurance::Minor => serializer.serialize_u8(1),
            SystemDesignAssurance::Major => serializer.serialize_u8(2),
            SystemDesignAssurance::Hazardous => serializer.serialize_u8(3),
        }
    }
}

impl fmt::Display for SystemDesignAssurance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            SystemDesignAssurance::UnknownOrNoSafetyEffect => write!(
                f,
                "Unknown or No Safety Effect (> 1x10-3 per flight hour
                or Unknown)"
            ),
            SystemDesignAssurance::Minor => write!(f, "Minor (≤ 1x10-3 per flight hour)"),
            SystemDesignAssurance::Major => write!(f, "Major (≤ 1x10-5 per flight hour)"),
            SystemDesignAssurance::Hazardous => write!(f, "Hazardous (≤ 1x10 per flight hour)"),
        }
    }
}
