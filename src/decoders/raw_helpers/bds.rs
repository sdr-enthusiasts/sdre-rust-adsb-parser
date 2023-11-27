// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    datalinkcapability::DataLinkCapability, helper_functions::aircraft_identification_read,
};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
#[deku(type = "u8", bits = "8")]
pub enum BDS {
    /// (1, 0) Table A-2-16
    #[deku(id = "0x00")]
    Empty([u8; 6]),

    /// (1, 0) Table A-2-16
    #[deku(id = "0x10")]
    DataLinkCapability(DataLinkCapability),

    /// (2, 0) Table A-2-32
    #[deku(id = "0x20")]
    AircraftIdentification(#[deku(reader = "aircraft_identification_read(deku::rest)")] String),

    #[deku(id_pat = "_")]
    Unknown([u8; 6]),
}

impl fmt::Display for BDS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(_) => {
                writeln!(f, "Comm-B format: empty response")?;
            }
            Self::AircraftIdentification(s) => {
                writeln!(f, "Comm-B format: BDS2,0 Aircraft identification")?;
                writeln!(f, "  Ident:         {s}")?;
            }
            Self::DataLinkCapability(_) => {
                writeln!(f, "Comm-B format: BDS1,0 Datalink capabilities")?;
            }
            Self::Unknown(_) => {
                writeln!(f, "Comm-B format: unknown format")?;
            }
        }
        Ok(())
    }
}
