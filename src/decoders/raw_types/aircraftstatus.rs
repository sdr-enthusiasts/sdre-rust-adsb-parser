// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use radix_fmt::radix;
use serde::{Deserialize, Serialize};

use super::{
    aircraftstatustype::AircraftStatusType, emergencystate::EmergencyState,
    helper_functions::decode_id13_field,
};

// FIXME: there appear to be 4 different variants of this message type.

/// Table: A-2-97
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AircraftStatus {
    pub sub_type: AircraftStatusType,
    pub emergency_state: EmergencyState,
    #[deku(
        bits = "13",
        endian = "big",
        map = "|squawk: u32| -> Result<_, DekuError> {Ok(decode_id13_field(squawk))}"
    )]
    pub squawk: u32,
    #[deku(bits = "32")]
    pub reserved: u32,
}

impl AircraftStatus {
    #[must_use]
    pub const fn is_reserved_zero(&self) -> bool {
        self.reserved == 0
    }

    #[must_use]
    pub fn get_squawk_as_octal_string(&self) -> String {
        format!("{:04}", radix(self.squawk, 16))
    }
}

#[cfg(test)]

pub mod test {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::aircraftstatus::AircraftStatus;
    use crate::decoders::raw_types::df::DF;

    #[test]
    fn decode_aircraftstatus() {
        "debug".enable_logging();

        let message = "8DAB44A7E10289000000008922C1";
        let decoded = message.to_adsb_raw().unwrap();

        info!("{:?}", decoded);

        let expected = AircraftStatus {
            sub_type: AircraftStatusType::EmergencyPriorityStatus,
            emergency_state: EmergencyState::None,
            squawk: 25092,
            reserved: 0,
        };

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::AircraftStatus(status) => {
                    assert_eq!(status, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }
}
