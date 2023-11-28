// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]
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

impl From<String> for Emergency {
    fn from(emergency: String) -> Self {
        match emergency.as_str() {
            "none" => Emergency::None,
            "general" => Emergency::General,
            "lifeguard" => Emergency::Lifeguard,
            "minfuel" => Emergency::Minfuel,
            "nordo" => Emergency::Nordo,
            "unlawful" => Emergency::Unlawful,
            "downed" => Emergency::Downed,
            "reserved" => Emergency::Reserved,
            _ => Emergency::None,
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
