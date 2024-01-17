// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// With MASSIVE thanks to https://github.com/rsadsb/adsb_deku

use crate::MessageResult;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self};

use super::helpers::prettyprint::{pretty_print_field, pretty_print_label};
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crc = self.crc;
        match &self.df {
            DF::ShortAirAirSurveillance { altitude, .. } => {
                writeln!(f, " Short Air-Air Surveillance")?;
                writeln!(f, "  ICAO Address:  {crc:06x} (Mode S / ADS-B)")?;
                if altitude.0 > 0 {
                    let altitude = altitude.0;
                    writeln!(f, "  Air/Ground:    airborne?")?;
                    writeln!(f, "  Altitude:      {altitude} ft barometric")?;
                } else {
                    writeln!(f, "  Air/Ground:    ground")?;
                }
            }
            DF::SurveillanceAltitudeReply { fs, ac, .. } => {
                writeln!(f, " Surveillance, Altitude Reply")?;
                writeln!(f, "  ICAO Address:  {crc:06x} (Mode S / ADS-B)")?;
                writeln!(f, "  Air/Ground:    {fs}")?;
                if ac.0 > 0 {
                    let altitude = ac.0;
                    writeln!(f, "  Altitude:      {altitude} ft barometric")?;
                }
            }
            DF::SurveillanceIdentityReply { fs, id, .. } => {
                let identity = id.0;
                writeln!(f, " Surveillance, Identity Reply")?;
                writeln!(f, "  ICAO Address:  {crc:06x} (Mode S / ADS-B)")?;
                writeln!(f, "  Air/Ground:    {fs}")?;
                writeln!(f, "  Identity:      {identity:04x}")?;
            }
            DF::AllCallReply {
                capability, icao, ..
            } => {
                writeln!(f, " All Call Reply")?;
                writeln!(f, "  ICAO Address:  {icao} (Mode S / ADS-B)")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
            }
            DF::LongAirAir { altitude, .. } => {
                writeln!(f, " Long Air-Air ACAS")?;
                writeln!(f, "  ICAO Address:  {crc:06x} (Mode S / ADS-B)")?;
                // TODO the airborne? shouldn't be static
                if altitude.0 > 0 {
                    let altitude = altitude.0;
                    writeln!(f, "  Air/Ground:    airborne?")?;
                    writeln!(f, "  Baro altitude: {altitude} ft")?;
                } else {
                    writeln!(f, "  Air/Ground:    ground")?;
                }
            }
            DF::ADSB(adsb) => {
                write!(f, "{}", adsb.to_string("(Mode S / ADS-B)"))?;
            }
            DF::TisB { cf, .. } => {
                write!(f, "{cf}")?;
            }
            // TODO
            DF::ExtendedQuitterMilitaryApplication { .. } => {}
            DF::CommBAltitudeReply { bds, alt, .. } => {
                writeln!(f, " Comm-B, Altitude Reply")?;
                writeln!(f, "  ICAO Address:  {crc:02X?} (Mode S / ADS-B)")?;
                let altitude = alt.0;
                writeln!(f, "  Altitude:      {altitude} ft")?;
                write!(f, "  {bds}")?;
            }
            DF::CommBIdentityReply { id, bds, .. } => {
                writeln!(f, " Comm-B, Identity Reply")?;
                writeln!(f, "    ICAO Address:  {crc:02X?} (Mode S / ADS-B)")?;
                writeln!(f, "    Squawk:        {id:02X?}")?;
                write!(f, "    {bds}")?;
            }
            DF::CommDExtendedLengthMessage { .. } => {
                writeln!(f, " Comm-D Extended Length Message")?;
                writeln!(f, "    ICAO Address:     {crc:02X?} (Mode S / ADS-B)")?;
            }
        }
        Ok(())
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
        pretty_print_label("ADS-B Raw Message", &mut output);
        pretty_print_field("", &self, &mut output);
        output
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_by_itself() {
        let input = "8DA0CA2DEA57F866C15C088DEF6F";

        let result = input.to_adsb_raw();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to decode message: {:?}", result);
        // 8d, ae, 54, ca, f8, 5, 0, 2, 0, 4a, b8, 67, a4, e

        // FIXME: This is a valid message, but it's not being decoded correctly. Disabling for now so that precommit works.
        let input = "8DAE54CAF8050002004AB867A40E";
        let result = input.to_adsb_raw();
        println!("Result: {:?}", result);
        assert!(result.is_ok(), "Failed to decode message: {:?}", result);
    }
}
