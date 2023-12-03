// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// With MASSIVE thanks to https://github.com/rsadsb/adsb_deku

use crate::MessageResult;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::raw_types::{df::DF, helper_functions::modes_checksum};

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `ADSBMessage`.
pub trait NewAdsbRawMessage {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage>;
}

/// Implementing `.to_adsb_raw()` for the type `String`.
///
/// This does not consume the `String`.
/// The expected input is a hexadecimal string.
///
/// Additionally, the input should not contain the adsb raw control characters `*` or `;` or `\n`
/// This is handled by the helpers::encode_adsb_raw_input::format_* functions
impl NewAdsbRawMessage for String {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        let bytes = hex::decode(self)?;
        match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `str`.
///
/// This does not consume the `str`.
/// The expected input is a hexadecimal string.
/// ///
/// Additionally, the input should not contain the adsb raw control characters `*` or `;` or `\n`
/// This is handled by the helpers::encode_adsb_raw_input::format_* functions
impl NewAdsbRawMessage for str {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        let bytes = hex::decode(self)?;
        match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `Vec<u8>`.
/// The expected input is a a Vec<u8> of *bytes*.
///
/// Additionally, the input should not contain the adsb raw control characters `*` or `;` or `\n`
/// This is handled by the helpers::encode_adsb_raw_input::format_* functions
impl NewAdsbRawMessage for &Vec<u8> {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        match AdsbRawMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `[u8]`.
/// The expected input is a a [u8] of *bytes*.
///
/// Additionally, the input should not contain the adsb raw control characters `*` or `;` or `\n`
/// This is handled by the helpers::encode_adsb_raw_input::format_* functions
impl NewAdsbRawMessage for &[u8] {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        match AdsbRawMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Downlink ADS-B Packet
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
pub struct AdsbRawMessage {
    /// Starting with 5 bit identifier, decode packet
    pub df: DF,
    /// Calculated from all bits, used as ICAO for Response packets
    #[deku(reader = "Self::read_crc(df, deku::input_bits)")]
    pub crc: u32,
}

impl fmt::Display for AdsbRawMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.to_string()
                .unwrap_or("Failed to convert to string".to_string())
        )
    }
}

/// Struct for holding a raw ADS-B message
/// This is the raw message that is received from the SDR.
///
/// The input used for deserializing in to this struct should not contain the adsb raw control characters `*` or `;` or `\n`
/// This is handled by the helpers::encode_adsb_raw_input::format_* functions
impl AdsbRawMessage {
    /// Read rest as CRC bits
    fn read_crc<'b>(
        df: &DF,
        rest: &'b BitSlice<u8, Msb0>,
    ) -> Result<(&'b BitSlice<u8, Msb0>, u32), DekuError> {
        const MODES_LONG_MSG_BYTES: usize = 14;
        const MODES_SHORT_MSG_BYTES: usize = 7;

        let bit_len = if let Ok(id) = df.deku_id() {
            if id & 0x10 != 0 {
                MODES_LONG_MSG_BYTES * 8
            } else {
                MODES_SHORT_MSG_BYTES * 8
            }
        } else {
            // In this case, it's the DF::CommD, which has multiple ids
            MODES_LONG_MSG_BYTES * 8
        };

        let (_, remaining_bytes, _) = rest.domain().region().unwrap();
        let crc = modes_checksum(remaining_bytes, bit_len)?;
        Ok((rest, crc))
    }

    pub fn pretty_print(&self) -> String {
        let mut output: String = String::new();
        // output.push_str(&format!("DF: {}\n", self.df));
        output.push_str(&format!("CRC: {}\n", self.crc));
        output
    }

    pub fn pretty_print_united_states(&self) -> String {
        unimplemented!()
    }

    pub fn pretty_print_metric(&self) -> String {
        unimplemented!()
    }

    /// Converts `AdsbRawMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `AdsbRawMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `ADSBRawMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `ADSBRawMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    pub fn get_time(&self) -> Option<f64> {
        Some(0.0)
    }
}
