// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::helper_functions::{decode_id13_field, mode_a_to_mode_c};
use super::{cprheaders::CPRFormat, surveillancestatus::SurveillanceStatus};

/// Latitude, Longitude and Altitude information
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Altitude {
    #[deku(bits = "5")]
    pub tc: u8,
    pub ss: SurveillanceStatus,
    #[deku(bits = "1")]
    pub saf_or_imf: u8,
    #[deku(reader = "Self::read(deku::rest)")]
    pub alt: Option<u16>,
    /// UTC sync or not
    #[deku(bits = "1")]
    pub t: bool,
    /// Odd or even
    pub odd_flag: CPRFormat,
    #[deku(bits = "17", endian = "big")]
    pub lat_cpr: u32,
    #[deku(bits = "17", endian = "big")]
    pub lon_cpr: u32,
}

impl fmt::Display for Altitude {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let altitude: String = self.alt.map_or_else(
            || "None".to_string(),
            |altitude| format!("{altitude} ft barometric"),
        );
        writeln!(f, "  Altitude:      {altitude}")?;
        writeln!(f, "  CPR type:      Airborne")?;
        writeln!(f, "  CPR odd flag:  {}", self.odd_flag)?;
        writeln!(f, "  CPR latitude:  ({})", self.lat_cpr)?;
        writeln!(f, "  CPR longitude: ({})", self.lon_cpr)?;
        Ok(())
    }
}

impl Altitude {
    /// `decodeAC12Field`
    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, Option<u16>), DekuError> {
        let (rest, num) = u32::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(12)))?;

        let q: u32 = num & 0x10;

        if q > 0 {
            let n: u32 = ((num & 0x0fe0) >> 1) | (num & 0x000f);
            let n: u32 = n * 25;
            if n > 1000 {
                // TODO: maybe replace with Result->Option
                Ok((rest, u16::try_from(n - 1000).ok()))
            } else {
                Ok((rest, None))
            }
        } else {
            let mut n: u32 = ((num & 0x0fc0) << 1) | (num & 0x003f);
            n = decode_id13_field(n);
            if let Ok(n) = mode_a_to_mode_c(n) {
                Ok((rest, u16::try_from(n * 100).ok()))
            } else {
                Ok((rest, None))
            }
        }
    }
}
