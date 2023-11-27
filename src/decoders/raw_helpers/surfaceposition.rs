// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{cprheaders::CPRFormat, statusforgroundtrack::StatusForGroundTrack};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct SurfacePosition {
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

impl fmt::Display for SurfacePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let lat: f64 = decode_cpr_latitude(self.lat_cpr, self.f);
        // let lon: f64 = decode_cpr_longitude(self.lon_cpr, self.f, lat);
        writeln!(f, "  Latitude:      {}", self.lat_cpr)?;
        writeln!(f, "  Longitude:     {}", self.lon_cpr)?;
        writeln!(f, "  CPR type:      Surface")?;
        writeln!(f, "  CPR odd flag:  {}", self.f)?;
        writeln!(f, "  Ground track:  {}", self.trk)?;
        writeln!(f, "  Ground speed:  {}", self.mov)?;
        writeln!(f, "  UTC sync:      {}", self.t)?;
        writeln!(f, "  Status:        {}", self.s)?;
        Ok(())
    }
}
