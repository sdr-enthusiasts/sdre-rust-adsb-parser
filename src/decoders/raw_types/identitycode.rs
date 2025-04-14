// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::ctx::{BitSize, Endian};
use deku::no_std_io::{Read, Seek};
use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// 13 bit identity code
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct IdentityCode(#[deku(reader = "Self::read(deku::reader)")] pub u16);

impl IdentityCode {
    fn read<R: Read + Seek>(reader: &mut Reader<R>) -> Result<u16, DekuError> {
        let num = u32::from_reader_with_ctx(reader, (Endian::Big, BitSize(13)))?;

        let c1 = (num & 0b1_0000_0000_0000) >> 12;
        let a1 = (num & 0b0_1000_0000_0000) >> 11;
        let c2 = (num & 0b0_0100_0000_0000) >> 10;
        let a2 = (num & 0b0_0010_0000_0000) >> 9;
        let c4 = (num & 0b0_0001_0000_0000) >> 8;
        let a4 = (num & 0b0_0000_1000_0000) >> 7;
        let b1 = (num & 0b0_0000_0010_0000) >> 5;
        let d1 = (num & 0b0_0000_0001_0000) >> 4;
        let b2 = (num & 0b0_0000_0000_1000) >> 3;
        let d2 = (num & 0b0_0000_0000_0100) >> 2;
        let b4 = (num & 0b0_0000_0000_0010) >> 1;
        let d4 = num & 0b0_0000_0000_0001;

        let a = (a4 << 2) | (a2 << 1) | a1;
        let b = (b4 << 2) | (b2 << 1) | b1;
        let c = (c4 << 2) | (c2 << 1) | c1;
        let d = (d4 << 2) | (d2 << 1) | d1;

        #[allow(clippy::cast_possible_truncation)]
        let num: u16 = ((a << 12) | (b << 8) | (c << 4) | d) as u16;
        Ok(num)
    }
}
