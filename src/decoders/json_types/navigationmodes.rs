// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum NavigationModes {
    Autopilot,
    VNAV,
    AltHold,
    Approach,
    LNAV,
    TCAS,
    #[default]
    None,
}

impl Serialize for NavigationModes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            NavigationModes::Autopilot => serializer.serialize_str("autopilot"),
            NavigationModes::VNAV => serializer.serialize_str("vnav"),
            NavigationModes::AltHold => serializer.serialize_str("althold"),
            NavigationModes::Approach => serializer.serialize_str("approach"),
            NavigationModes::LNAV => serializer.serialize_str("lnav"),
            NavigationModes::TCAS => serializer.serialize_str("tcas"),
            NavigationModes::None => serializer.serialize_str("none"),
        }
    }
}

impl TryFrom<String> for NavigationModes {
    type Error = String;

    fn try_from(navigation_modes: String) -> Result<Self, Self::Error> {
        match navigation_modes.as_str() {
            "autopilot" => Ok(NavigationModes::Autopilot),
            "vnav" => Ok(NavigationModes::VNAV),
            "althold" => Ok(NavigationModes::AltHold),
            "approach" => Ok(NavigationModes::Approach),
            "lnav" => Ok(NavigationModes::LNAV),
            "tcas" => Ok(NavigationModes::TCAS),
            _ => Err(format!("Invalid navigation mode: {navigation_modes}")),
        }
    }
}

impl fmt::Display for NavigationModes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NavigationModes::Autopilot => write!(f, "Autopilot"),
            NavigationModes::VNAV => write!(f, "Vertical Navigation"),
            NavigationModes::AltHold => write!(f, "Altitude Hold"),
            NavigationModes::Approach => write!(f, "Approach"),
            NavigationModes::LNAV => write!(f, "Lateral Navigation"),
            NavigationModes::TCAS => write!(f, "Traffic Collision Avoidance System"),
            NavigationModes::None => write!(f, "None"),
        }
    }
}
