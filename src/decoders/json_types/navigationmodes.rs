// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]
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

impl From<String> for NavigationModes {
    fn from(navigation_modes: String) -> Self {
        match navigation_modes.as_str() {
            "autopilot" => NavigationModes::Autopilot,
            "vnav" => NavigationModes::VNAV,
            "althold" => NavigationModes::AltHold,
            "approach" => NavigationModes::Approach,
            "lnav" => NavigationModes::LNAV,
            "tcas" => NavigationModes::TCAS,
            _ => NavigationModes::None,
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
