// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// [`ME::AirborneVelocity`] && [`AirborneVelocitySubType::AirspeedDecoding`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AirspeedDecoding {
    #[deku(bits = "1")]
    pub status_heading: u8,
    #[deku(endian = "big", bits = "10")]
    pub mag_heading: u16,
    #[deku(bits = "1")]
    pub airspeed_type: u8,
    #[deku(
        endian = "big",
        bits = "10",
        map = "|airspeed: u16| -> Result<_, DekuError> {Ok(if airspeed > 0 { airspeed - 1 } else { 0 })}"
    )]
    pub airspeed: u16,
}

impl fmt::Display for AirspeedDecoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.airspeed_type == 0 {
            write!(f, "  IAS:           {} kt", self.airspeed)?;
        } else {
            write!(f, "  TAS:           {} kt", self.airspeed)?;
        }
        Ok(())
    }
}
