// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{helper_functions::aircraft_identification_read, typecoding::TypeCoding};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
pub struct Identification {
    pub tc: TypeCoding,

    #[deku(bits = "3")]
    pub ca: u8,

    /// N-Number / Tail Number
    #[deku(reader = "aircraft_identification_read(deku::rest)")]
    pub cn: String,
}

#[cfg(test)]

pub mod test {
    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::df::DF;
    use crate::decoders::raw_types::identification::Identification;

    #[test]
    fn decode_identification() {
        let message = "8DA69B9C223B5CB5082820C97A87";
        let decoded = message.to_adsb_raw().unwrap();

        println!("{:?}", decoded);

        let expected = Identification {
            tc: TypeCoding::A,
            ca: 2,
            cn: "N525BB".to_string(),
        };

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::AircraftIdentification(id) => {
                    assert_eq!(id, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }
}
