// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
//use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    beast_types::messagetype::MessageType,
    helpers::prettyprint::{pretty_print_field, pretty_print_label},
    raw::AdsbRawMessage,
};

// Beast format sources:
// https://wiki.jetvision.de/wiki/Mode-S_Beast:Data_Output_Formats
// https://github.com/firestuff/adsb-tools/blob/master/protocols/beast.md#examples

// <esc> "1" : 6 byte MLAT timestamp, 1 byte signal level, 2 byte Mode-AC
// <esc> "2" : 6 byte MLAT timestamp, 1 byte signal level, 7 byte Mode-S short frame
// <esc> "3" : 6 byte MLAT timestamp, 1 byte signal level, 14 byte Mode-S long frame
// <esc> "4" : 6 byte MLAT timestamp, 1 byte unused, DIP switch configuration settings, time stamp error ticks as int8_t (1 tick is 15ns) (message "4" not on Mode-S Beast classic)
// <esc><esc>: true 0x1a
// <esc> is 0x1a, and "1", "2" and "3" are 0x31, 0x32 and 0x33

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in beast format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `ADSBMessage`.
pub trait NewAdsbBeastMessage {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage>;
}

/// Implementing `.to_adsb_beast()` for the type `String`.
///
/// This does not consume the `String`.
/// The expected input is a hexadecimal string.
///
/// Expecting to be able to use this method will likely fail. Most of the beast
/// Input contains non-ascii characters (technically, those from 0x80 to 0xff).
/// Rust strings are utf-8, and thus cannot contain those characters.
impl NewAdsbBeastMessage for String {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        let bytes: Vec<u8> = hex::decode(self)?;
        match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_beast()` for the type `str`.
///
/// This does not consume the `str`.
/// The expected input is a hexadecimal string.
///
/// Expecting to be able to use this method will likely fail. Most of the beast
/// Input contains non-ascii characters (technically, those from 0x80 to 0xff).
/// Rust strings are utf-8, and thus cannot contain those characters.
impl NewAdsbBeastMessage for str {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        let bytes: Vec<u8> = hex::decode(self)?;
        match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_beast()` for the type `Vec<u8>`.
/// This does not consume the `Vec<u8>`.
/// The expected input is a a Vec<u8> of *bytes*.
///
/// Please note that in order for beast to be decoded, the message should NOT be escaped, meaning the leading 0x1a should be removed.
///
/// Additionally, incoming message payloads will have a 0x1a 0x1a sequence to represent a single 0x1a byte.
/// This is not handled by this library, and should be handled by the user by only emitting one 0x1a byte in the payload that is processed here.
/// Both of those are handled by helpers::encode_adsb_beast_input::format_* methods.
impl NewAdsbBeastMessage for &Vec<u8> {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        match AdsbBeastMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_beast()` for the type `Vec<u8>`.
/// This does not consume the `[u8]`.
/// The expected input is a a [u8] of *bytes*.
///
/// Please note that in order for beast to be decoded, the message should NOT be escaped, meaning the leading 0x1a should be removed.
///
/// Additionally, incoming message payloads will have a 0x1a 0x1a sequence to represent a single 0x1a byte.
/// This is not handled by this library, and should be handled by the user by only emitting one 0x1a byte in the payload that is processed here.
/// Both of those are handled by helpers::encode_adsb_beast_input::format_* methods.
impl NewAdsbBeastMessage for &[u8] {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        match AdsbBeastMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl fmt::Display for AdsbBeastMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ADSB Beast Message: Type: {}, MLAT Timestamp: {:?}, Signal Level: {}, Message: {:02X?}",
            self.message_type, self.mlat_timestamp, self.signal_level, self.message
        )
    }
}

/// The struct containing the decoded ADSB Beast message.
/// In order for beast to be decoded, the message should NOT be escaped, meaning the leading 0x1a should be removed.
///
/// Additionally, incoming message payloads will have a 0x1a 0x1a sequence to represent a single 0x1a byte.
/// This is not handled by this library, and should be handled by the user by only emitting one 0x1a byte in the payload that is processed here.
/// Both of those are handled by helpers::encode_adsb_beast_input::format_* methods.
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AdsbBeastMessage {
    /// 1: Message Type
    message_type: MessageType,
    /// 2: MLAT Timestamp
    #[deku(endian = "big", bits = "48")]
    mlat_timestamp: u64,
    /// 3: Signal Level
    #[deku(bits = "8")]
    signal_level: u8,
    /// 4: Message
    message: AdsbRawMessage,
}

impl AdsbBeastMessage {
    /// Converts `AdsbBeastMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        Ok(serde_json::to_string(self)?)
    }

    /// Converts `AdsbBeastMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `AdsbBeastMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `AdsbBeastMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    pub fn pretty_print(&self) -> String {
        let mut output = String::new();
        pretty_print_label("ADS-B Beast Message", &mut output);
        pretty_print_field("Message Type", &self.message_type, &mut output);
        pretty_print_field("MLAT Timestamp", &self.mlat_timestamp, &mut output);
        pretty_print_field("Signal Level", &self.signal_level, &mut output);
        pretty_print_label("ADS-B Beast Message", &mut output);
        pretty_print_field("", &self.message, &mut output);

        output
    }
}
