// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::sign::Sign;

/// [`ME::AirborneVelocity`] && [`AirborneVelocitySubType::GroundSpeedDecoding`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GroundSpeedDecoding {
    pub ew_sign: Sign,
    #[deku(endian = "big", bits = "10")]
    pub ew_vel: u16,
    pub ns_sign: Sign,
    #[deku(endian = "big", bits = "10")]
    pub ns_vel: u16,
}
