// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    cprheaders::CPRFormat, groundspeed::GroundSpeed, statusforgroundtrack::StatusForGroundTrack,
};

/// `type_code` (the ADS-B Type Code) is not read from the wire by this
/// struct: the enclosing [`ME`](super::me::ME) enum already has to consume
/// those 5 bits to pick the `SurfacePosition` variant, so it forwards the
/// already-read value in via `ctx` instead of letting it be read (and thus
/// the bitstream position advanced) a second time here.
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
#[deku(ctx = "type_code: u8")]
pub struct SurfacePosition {
    #[deku(skip, default = "type_code")]
    pub type_code: u8,
    #[deku(bits = "7")]
    pub mov: u8,
    pub s: StatusForGroundTrack,
    #[deku(bits = "7")]
    pub trk: u8,
    #[deku(bits = "1")]
    pub t: bool,
    pub f: CPRFormat,
    #[deku(bits = "17", endian = "big")]
    pub lat_cpr: u32,
    #[deku(bits = "17", endian = "big")]
    pub lon_cpr: u32,
}

impl SurfacePosition {
    #[must_use]
    pub fn get_heading(&self) -> Option<f32> {
        match self.s {
            StatusForGroundTrack::Invalid => None,
            StatusForGroundTrack::Valid => {
                // don't divide by zero :((((
                if self.trk == 0 {
                    Some(360.0)
                } else {
                    Some((360.0 * f32::from(self.trk)) / 128.0)
                }
            }
        }
    }

    #[must_use]
    pub fn get_ground_speed(&self) -> Option<GroundSpeed> {
        match self.s {
            StatusForGroundTrack::Invalid => None,
            StatusForGroundTrack::Valid => Some(GroundSpeed::from(self.mov)),
        }
    }
}

// We would do tests here but we're doing that in the cpr module, where we also test decoding the position
