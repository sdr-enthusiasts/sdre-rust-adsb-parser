// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
// #[serde(untagged)]
pub enum Emergency {
    #[default]
    None,
    General,
    Lifeguard,
    Minfuel,
    Nordo,
    Unlawful,
    Downed,
    Reserved,
}

impl Serialize for Emergency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Emergency::None => serializer.serialize_str("none"),
            Emergency::General => serializer.serialize_str("general"),
            Emergency::Lifeguard => serializer.serialize_str("lifeguard"),
            Emergency::Minfuel => serializer.serialize_str("minfuel"),
            Emergency::Nordo => serializer.serialize_str("nordo"),
            Emergency::Unlawful => serializer.serialize_str("unlawful"),
            Emergency::Downed => serializer.serialize_str("downed"),
            Emergency::Reserved => serializer.serialize_str("reserved"),
        }
    }
}

impl TryFrom<String> for Emergency {
    type Error = String;

    fn try_from(emergency: String) -> Result<Self, Self::Error> {
        match emergency.as_str() {
            "none" => Ok(Emergency::None),
            "general" => Ok(Emergency::General),
            "lifeguard" => Ok(Emergency::Lifeguard),
            "minfuel" => Ok(Emergency::Minfuel),
            "nordo" => Ok(Emergency::Nordo),
            "unlawful" => Ok(Emergency::Unlawful),
            "downed" => Ok(Emergency::Downed),
            "reserved" => Ok(Emergency::Reserved),
            _ => Err(format!("Invalid emergency: {emergency}")),
        }
    }
}

impl fmt::Display for Emergency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Emergency::None => write!(f, "No emergency"),
            Emergency::General => write!(f, "General emergency"),
            Emergency::Lifeguard => write!(f, "Lifeguard"),
            Emergency::Minfuel => write!(f, "Minimum fuel"),
            Emergency::Nordo => write!(f, "No radio"),
            Emergency::Unlawful => write!(f, "Unlawful interference"),
            Emergency::Downed => write!(f, "Downed aircraft"),
            Emergency::Reserved => write!(f, "Reserved"),
        }
    }
}
