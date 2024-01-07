// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::airbornevelocitytype::AirborneVelocityType;
use super::direction_nsew::{DirectionEW, DirectionNS};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(ctx = "t: AirborneVelocityType")]
pub struct AirborneVelocitySubFields {
    pub dew: DirectionEW,
    #[deku(reader = "Self::read_v(deku::rest, t)")]
    pub vew: u16,
    pub dns: DirectionNS,
    #[deku(reader = "Self::read_v(deku::rest, t)")]
    pub vns: u16,
}

impl AirborneVelocitySubFields {
    fn read_v(
        rest: &BitSlice<u8, Msb0>,
        t: AirborneVelocityType,
    ) -> Result<(&BitSlice<u8, Msb0>, u16), DekuError> {
        match t {
            AirborneVelocityType::Subsonic => {
                u16::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(10)))
                    .map(|(rest, value)| (rest, value - 1))
            }
            AirborneVelocityType::Supersonic => {
                u16::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(10)))
                    .map(|(rest, value)| (rest, 4 * (value - 1)))
            }
        }
    }
}
