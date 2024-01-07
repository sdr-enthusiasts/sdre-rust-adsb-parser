// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "3")]
pub enum EmergencyState {
    None = 0,
    General = 1,
    Lifeguard = 2,
    MinimumFuel = 3,
    NoCommunication = 4,
    UnlawfulInterference = 5,
    DownedAircraft = 6,
    Reserved2 = 7,
}

impl fmt::Display for EmergencyState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "no emergency"),
            Self::General => write!(f, "general"),
            Self::Lifeguard => write!(f, "lifeguard"),
            Self::MinimumFuel => write!(f, "minimum fuel"),
            Self::NoCommunication => write!(f, "no communication"),
            Self::UnlawfulInterference => write!(f, "unlawful interference"),
            Self::DownedAircraft => write!(f, "downed aircraft"),
            Self::Reserved2 => write!(f, "reserved2"),
        }
    }
}
