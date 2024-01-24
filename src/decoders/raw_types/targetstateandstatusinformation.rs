// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// Target State and Status (§2.2.3.2.7.1)
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, PartialEq)]
pub struct TargetStateAndStatusInformation {
    // TODO Support Target State and Status defined in DO-260A, ADS-B Version=1
    // TODO Support reserved 2..=3
    #[deku(bits = "2", assert_eq = "1")]
    pub subtype: u8,
    #[deku(bits = "1")]
    pub is_fms: bool,
    #[deku(
        bits = "12",
        endian = "big",
        map = "|altitude: u32| -> Result<_, DekuError> {Ok(if altitude > 1 {(altitude - 1) * 32} else {0} )}"
    )]
    pub altitude: u32,
    #[deku(
        bits = "9",
        endian = "big",
        map = "|qnh: u32| -> Result<_, DekuError> {if qnh == 0 { Ok(0.0) } else { Ok(800.0 + ((qnh - 1) as f32) * 0.8)}}"
    )]
    pub qnh: f32,
    #[deku(bits = "1")]
    pub is_heading: bool,
    #[deku(
        bits = "9",
        endian = "big",
        map = "|heading: u16| -> Result<_, DekuError> {Ok(heading as f32 * 180.0 / 256.0)}"
    )]
    pub heading: f32,
    #[deku(bits = "4")]
    pub nacp: u8,
    #[deku(bits = "1")]
    pub nicbaro: u8,
    #[deku(bits = "2")]
    pub sil: u8,
    #[deku(bits = "1")]
    pub mode_validity: bool,
    #[deku(bits = "1")]
    pub autopilot: bool,
    #[deku(bits = "1")]
    pub vnac: bool,
    #[deku(bits = "1")]
    pub alt_hold: bool,
    #[deku(bits = "1")]
    pub reserved0: u8,
    #[deku(bits = "1")]
    pub approach: bool,
    #[deku(bits = "1")]
    pub tcas: bool,
    #[deku(bits = "1")]
    pub lnav: bool,
    #[deku(bits = "2")]
    pub reserved1: u8,
}

impl TargetStateAndStatusInformation {
    #[must_use] pub const fn is_reserved_zero(&self) -> bool {
        self.reserved0 == 0 && self.reserved1 == 0
    }
}

#[cfg(test)]
mod test {
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::df::DF;
    use crate::decoders::raw_types::me::ME;

    use super::*;

    #[test]
    fn test_status_information() {
        let message = "8DABEBE0EA36C866DD5C082732C5";
        let decoded = message.to_adsb_raw().unwrap();
        println!("Decoded {:?}", decoded);

        let expected = TargetStateAndStatusInformation {
            subtype: 1,
            is_fms: false,
            altitude: 28000,
            qnh: 1013.6,
            is_heading: true,
            heading: 257.34375,
            nacp: 10,
            nicbaro: 1,
            sil: 3,
            mode_validity: false,
            autopilot: false,
            vnac: false,
            reserved0: 0,
            alt_hold: false,
            approach: false,
            tcas: true,
            lnav: false,
            reserved1: 0,
        };

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                ME::TargetStateAndStatusInformation(state) => {
                    assert_eq!(state, expected);
                }
                _ => panic!("ME is not Target and state information"),
            },
            _ => panic!("DF is not ADSB"),
        }
    }
}
