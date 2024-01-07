// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{airspeeddecoding::AirspeedDecoding, groundspeeddecoding::GroundSpeedDecoding};

/// Airborne Velocity Message “Subtype” Code Field Encoding
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
#[deku(ctx = "st: u8", id = "st")]
pub enum AirborneVelocitySubType {
    #[deku(id = "0")]
    Reserved0(#[deku(bits = "22")] u32),

    #[deku(id_pat = "1..=2")]
    GroundSpeedDecoding(GroundSpeedDecoding),

    #[deku(id_pat = "3..=4")]
    AirspeedDecoding(AirspeedDecoding),

    #[deku(id_pat = "5..=7")]
    Reserved1(#[deku(bits = "22")] u32),
}
