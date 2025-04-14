// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum AutopilotEngaged {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for AutopilotEngaged {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "Autppilot Engaged"),
            Self::Disengaged => write!(f, "Autopilot Disengaged"),
        }
    }
}

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum VNAVEngaged {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for VNAVEngaged {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "VNAV Engaged"),
            Self::Disengaged => write!(f, "VNAV Disenaged"),
        }
    }
}

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum AltitudeHold {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for AltitudeHold {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "Altitude Hold Engaged"),
            Self::Disengaged => write!(f, "Altitude Hold Disengaged"),
        }
    }
}

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum ApproachMode {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for ApproachMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "Approach Mode Engaged"),
            Self::Disengaged => write!(f, "Approach Mode Disengaged"),
        }
    }
}

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum TCAS {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for TCAS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "TCAS Engaged"),
            Self::Disengaged => write!(f, "TCAS Disengaged"),
        }
    }
}

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum LNAV {
    #[deku(id = "1")]
    Engaged,
    #[deku(id = "0")]
    Disengaged,
}

impl fmt::Display for LNAV {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "LNAV Engaged"),
            Self::Disengaged => write!(f, "LNAV Disengaged"),
        }
    }
}
