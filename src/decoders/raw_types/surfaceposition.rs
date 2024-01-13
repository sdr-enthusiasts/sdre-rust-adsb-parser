// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    cprheaders::CPRFormat, groundspeed::GroundSpeed, statusforgroundtrack::StatusForGroundTrack,
};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct SurfacePosition {
    pub mov: GroundSpeed,
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
    pub fn get_heading(&self) -> Option<f32> {
        match self.s {
            StatusForGroundTrack::Invalid => None,
            StatusForGroundTrack::Valid => {
                // don't divide by zero :((((
                if self.trk == 0 {
                    Some(360.0)
                } else {
                    Some(360.0 * (self.trk as f32 / 128.0))
                }
            }
        }
    }
}
