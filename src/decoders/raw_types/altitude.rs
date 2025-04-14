// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::decoders::common_types::surveillancestatus::SurveillanceStatus;
use deku::ctx::{BitSize, Endian};
use deku::no_std_io::{Read, Seek};
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::cprheaders::CPRFormat;
use super::helper_functions::{decode_id13_field, mode_a_to_mode_c};

/// Latitude, Longitude and Altitude information
#[derive(
    Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Default,
)]
pub struct Altitude {
    #[deku(bits = "5")]
    pub tc: u8,
    pub ss: SurveillanceStatus,
    #[deku(bits = "1")]
    /// nic supplement b
    pub saf_or_imf: u8,
    #[deku(reader = "Self::read(deku::reader)")]
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
    fn read<R: Read + Seek>(reader: &mut Reader<R>) -> Result<Option<u16>, DekuError> {
        let num = u32::from_reader_with_ctx(reader, (Endian::Big, BitSize(12)))?;
        let q = num & 0x10;

        if q > 0 {
            let n = ((num & 0x0fe0) >> 1) | (num & 0x000f);
            let n = n * 25;
            if n > 1000 {
                // TODO: maybe replace with Result->Option
                Ok(u16::try_from(n - 1000).ok())
            } else {
                Ok(None)
            }
        } else {
            let mut n = ((num & 0x0fc0) << 1) | (num & 0x003f);
            n = decode_id13_field(n);
            if let Ok(n) = mode_a_to_mode_c(n) {
                Ok(u16::try_from(n * 100).ok())
            } else {
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::df::DF;

    #[test]
    fn decode_altitude() {
        "debug".enable_logging();

        let message = "8FA4955D597D8288F8C756559A37";
        let decoded = message.to_adsb_raw().unwrap();

        let expected = Altitude {
            tc: 11,
            ss: SurveillanceStatus::NoCondition,
            saf_or_imf: 1,
            alt: Some(24000),
            t: false,
            odd_flag: CPRFormat::Even,
            lat_cpr: 83068,
            lon_cpr: 51030,
        };

        info!("Decoded Message: {:?}", &decoded);

        if let DF::ADSB(adsb) = decoded.df {
            match adsb.me {
                crate::decoders::raw_types::me::ME::AirbornePositionBaroAltitude(altitude) => {
                    assert_eq!(altitude, expected);
                }
                _ => panic!("Wrong ME type"),
            }
        }
    }
}
