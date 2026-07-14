// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{helper_functions::aircraft_identification_read, typecoding::TypeCoding};

/// `tc` (the ADS-B Type Code) is not read from the wire by this struct: the
/// enclosing [`ME`](super::me::ME) enum already has to consume those 5 bits
/// to pick the `AircraftIdentification` variant, so it forwards the
/// already-read value in via `ctx` instead of letting it be read (and thus
/// the bitstream position advanced) a second time here.
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
#[deku(ctx = "tc: u8")]
pub struct Identification {
    #[deku(skip, default = "TypeCoding::from(tc)")]
    pub tc: TypeCoding,

    #[deku(bits = "3")]
    pub ca: u8,

    /// N-Number / Tail Number
    #[deku(reader = "aircraft_identification_read(deku::reader)")]
    pub cn: String,
}

#[cfg(test)]
pub mod test {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::df::DF;
    use crate::decoders::raw_types::identification::Identification;

    #[test]
    fn decode_identification() {
        "debug".enable_logging();

        let message = "8DA69B9C223B5CB5082820C97A87";
        let decoded = message.to_adsb_raw().unwrap();

        info!("{decoded:?}");

        let expected = Identification {
            tc: TypeCoding::A,
            ca: 2,
            cn: "N525BB".to_string(),
        };

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                crate::decoders::raw_types::me::ME::AircraftIdentification(_, id) => {
                    assert_eq!(id, expected);
                }
                _ => panic!("Wrong ME"),
            },
            _ => panic!("Wrong DF"),
        }
    }
}
