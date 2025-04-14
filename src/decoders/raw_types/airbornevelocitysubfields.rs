// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::no_std_io::{Read, Seek};
use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::airbornevelocitytype::AirborneVelocityType;
use super::direction_nsew::{DirectionEW, DirectionNS};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(ctx = "t: AirborneVelocityType")]
pub struct AirborneVelocitySubFields {
    pub dew: DirectionEW,
    #[deku(reader = "Self::read_v(deku::reader, t)")]
    pub vew: u16,
    pub dns: DirectionNS,
    #[deku(reader = "Self::read_v(deku::reader, t)")]
    pub vns: u16,
}

impl AirborneVelocitySubFields {
    fn read_v<R: Read + Seek>(
        reader: &mut Reader<R>,
        t: AirborneVelocityType,
    ) -> Result<u16, DekuError> {
        match t {
            AirborneVelocityType::Subsonic => {
                u16::from_reader_with_ctx(reader, (deku::ctx::Endian::Big, deku::ctx::BitSize(10)))
                    .map(|value| value - 1)
            }
            AirborneVelocityType::Supersonic => {
                u16::from_reader_with_ctx(reader, (deku::ctx::Endian::Big, deku::ctx::BitSize(10)))
                    .map(|value| 4 * (value - 1))
            }
        }
    }
}
