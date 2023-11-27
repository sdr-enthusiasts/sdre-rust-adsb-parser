// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
//use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{beast_types::messagetype::MessageType, raw_types::df::DF};

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
impl NewAdsbBeastMessage for String {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        let bytes = hex::decode(self)?;
        match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `str`.
///
/// This does not consume the `str`.
/// The expected input is a hexadecimal string.
impl NewAdsbBeastMessage for str {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        let bytes = hex::decode(self)?;
        match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `Vec<u8>`.
/// The expected input is a a Vec<u8> of *bytes*.
impl NewAdsbBeastMessage for &Vec<u8> {
    fn to_adsb_beast(&self) -> MessageResult<AdsbBeastMessage> {
        match AdsbBeastMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `[u8]`.
/// The expected input is a a [u8] of *bytes*.
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
            "ADSB Beast Message: Type: {}, MLAT Timestamp: {:?}, Signal Level: {}, Message: {:x?}",
            self.message_type, self.mlat_timestamp, self.signal_level, self.message
        )
    }
}

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
    message: DF,
}
