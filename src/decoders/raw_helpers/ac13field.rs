// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use super::helper_functions::{decode_id13_field, mode_a_to_mode_c};
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// 13 bit encoded altitude
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AC13Field(#[deku(reader = "Self::read(deku::rest)")] pub u16);

impl fmt::Display for AC13Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  AC13:          {}", self.0)?;
        Ok(())
    }
}

impl AC13Field {
    // TODO Add unit
    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, u16), DekuError> {
        let (rest, num) = u32::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(13)))?;

        let m_bit: u32 = num & 0x0040;
        let q_bit: u32 = num & 0x0010;

        if m_bit != 0 {
            // TODO: this might be wrong?
            Ok((rest, 0))
        } else if q_bit != 0 {
            let n: u32 = ((num & 0x1f80) >> 2) | ((num & 0x0020) >> 1) | (num & 0x000f);
            let n: u32 = n * 25;
            if n > 1000 {
                Ok((rest, (n - 1000) as u16))
            } else {
                // TODO: add error
                Ok((rest, 0))
            }
        } else {
            // TODO 11 bit gillham coded altitude
            if let Ok(n) = mode_a_to_mode_c(decode_id13_field(num)) {
                Ok((rest, (100 * n) as u16))
            } else {
                Ok((rest, 0))
            }
        }
    }
}
