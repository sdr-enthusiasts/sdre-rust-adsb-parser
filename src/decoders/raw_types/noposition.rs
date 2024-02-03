// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::{
    bitvec::{BitSlice, Msb0},
    prelude::*,
};
use serde::{Deserialize, Serialize};

use super::helper_functions::{decode_id13_field, mode_a_to_mode_c};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct NoPosition {
    #[deku(bits = "3")]
    pub st: u8,
    #[deku(reader = "Self::read(deku::rest)")]
    pub altitude: Option<u16>,
}

impl NoPosition {
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

#[cfg(test)]
mod tests {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;

    #[test]
    fn test_no_position_decoder() {
        "debug".enable_logging();

        let message = "8DADC035002D8000000000B16E64";
        let decoded = message.to_adsb_raw().unwrap();
        let expected = NoPosition {
            st: 0,
            altitude: Some(8000),
        };
        info!("{:?}", decoded);
        match decoded.df {
            crate::decoders::raw_types::df::DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::NoPosition(status) => {
                    assert_eq!(status, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }

    #[test]
    fn test_no_position_alternate() {
        "debug".enable_logging();

        let message = "8EADC035002D800000000059FDEC";
        let decoded = message.to_adsb_raw().unwrap();
        let expected = NoPosition {
            st: 0,
            altitude: Some(8000),
        };
        info!("{:?}", decoded);
        match decoded.df {
            crate::decoders::raw_types::df::DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::NoPosition(status) => {
                    assert_eq!(status, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }

    #[test]
    fn test_no_position_last() {
        "debug".enable_logging();

        let message = "8EADC035002D7000000000B02845";
        let decoded = message.to_adsb_raw().unwrap();
        let expected = NoPosition {
            st: 0,
            altitude: Some(7975),
        };
        info!("{:?}", decoded);
        match decoded.df {
            crate::decoders::raw_types::df::DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::NoPosition(status) => {
                    assert_eq!(status, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }
}
