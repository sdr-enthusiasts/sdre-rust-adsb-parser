// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use super::helper_functions::{decode_id13_field, mode_a_to_mode_c};
use deku::ctx::{BitSize, Endian};
use deku::no_std_io::{Read, Seek};
use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// 13 bit encoded altitude
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AC13Field(#[deku(reader = "Self::read(deku::reader)")] pub u16);

impl AC13Field {
    // TODO Add unit
    fn read<R: Read + Seek>(reader: &mut Reader<R>) -> Result<u16, DekuError> {
        let num = u16::from_reader_with_ctx(reader, (Endian::Big, BitSize(13)))?;

        // Handle invalid or special codes
        if num == 0 || num == 0b1_1111_1111_1111 {
            return Ok(0);
        }

        let m_bit = num & 0x0040;
        let q_bit = num & 0x0010;

        if m_bit != 0 {
            // TODO: read altitude when meter is selected
            Ok(0)
        } else if q_bit != 0 {
            let n = ((num & 0x1f80) >> 2) | ((num & 0x0020) >> 1) | (num & 0x000f);
            let n = n * 25;
            if n > 1000 {
                Ok(n - 1000)
            } else {
                // TODO: add error
                Ok(0)
            }
        } else {
            // TODO 11 bit gillham coded altitude
            if let Ok(n) = mode_a_to_mode_c(decode_id13_field(u32::from(num))) {
                #[allow(clippy::cast_possible_truncation)]
                Ok((100 * n) as u16)
            } else {
                Ok(0)
            }
        }
    }
}
