// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// 13 bit identity code
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct IdentityCode(#[deku(reader = "Self::read(deku::rest)")] pub u16);

impl IdentityCode {
    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, u16), DekuError> {
        let (rest, num) = u32::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(13)))?;

        let c1: u32 = (num & 0b1_0000_0000_0000) >> 12;
        let a1: u32 = (num & 0b0_1000_0000_0000) >> 11;
        let c2: u32 = (num & 0b0_0100_0000_0000) >> 10;
        let a2: u32 = (num & 0b0_0010_0000_0000) >> 9;
        let c4: u32 = (num & 0b0_0001_0000_0000) >> 8;
        let a4: u32 = (num & 0b0_0000_1000_0000) >> 7;
        let b1: u32 = (num & 0b0_0000_0010_0000) >> 5;
        let d1: u32 = (num & 0b0_0000_0001_0000) >> 4;
        let b2: u32 = (num & 0b0_0000_0000_1000) >> 3;
        let d2: u32 = (num & 0b0_0000_0000_0100) >> 2;
        let b4: u32 = (num & 0b0_0000_0000_0010) >> 1;
        let d4: u32 = num & 0b0_0000_0000_0001;

        let a: u32 = a4 << 2 | a2 << 1 | a1;
        let b: u32 = b4 << 2 | b2 << 1 | b1;
        let c: u32 = c4 << 2 | c2 << 1 | c1;
        let d: u32 = d4 << 2 | d2 << 1 | d1;

        let num: u16 = (a << 12 | b << 8 | c << 4 | d) as u16;
        Ok((rest, num))
    }
}

impl fmt::Display for IdentityCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "  Ident:         {}", self.0)?;
        Ok(())
    }
}
