// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "3")]
pub enum AircraftStatusType {
    #[deku(id = "0")]
    NoInformation,
    #[deku(id = "1")]
    EmergencyPriorityStatus,
    #[deku(id = "2")]
    ACASRaBroadcast,
    #[deku(id_pat = "_")]
    Reserved,
}

impl fmt::Display for AircraftStatusType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoInformation => write!(f, "no information"),
            Self::EmergencyPriorityStatus => write!(f, "emergency/priority status"),
            Self::ACASRaBroadcast => write!(f, "ACAS RA broadcast"),
            Self::Reserved => write!(f, "reserved"),
        }
    }
}
