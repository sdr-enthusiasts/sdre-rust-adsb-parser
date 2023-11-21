extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;

use error_handling::deserialization_error::DeserializationError;

#[cfg(feature = "json")]
use decoders::json::{AircraftJSON, JSONMessage};
#[cfg(feature = "raw")]
use decoders::raw::AdsbRawMessage;
use serde::{Deserialize, Serialize};
pub mod decoders {
    #[cfg(feature = "json")]
    pub mod json;
    #[cfg(feature = "raw")]
    pub mod raw;
}

pub mod error_handling {
    pub mod adsb_raw_error;
    pub mod deserialization_error;
}

pub mod helpers {
    pub mod encode_adsb_raw_input;
}

/// Common return type for all serialisation/deserialisation functions.
///
/// This serves as a wrapper for `serde_json::Error` as the Error type.
pub type MessageResult<T> = Result<T, DeserializationError>;

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
pub trait DecodeMessage {
    fn decode_message(&self) -> MessageResult<ADSBMessage>;
}

/// Provides functionality for decoding a `String` to `ADSBMessage`.
///
/// This does not consume the `String`.
impl DecodeMessage for String {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Provides functionality for decoding a `str` to `ADSBMessage`.
///
/// This does not consume the `str`.
impl DecodeMessage for str {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Provides functionality for decoding a `&[u8]` to `ADSBMessage`.
///
/// This does not consume the `&[u8]`.
impl DecodeMessage for &[u8] {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        let string = String::from_utf8_lossy(self);
        match serde_json::from_str(&string) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Implementation of `ADSBMessage`.
impl ADSBMessage {
    /// Converts `ADSBMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string", &self);
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `ADSBMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string and appending a newline", &self);
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `ADSBMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        trace!("Converting {:?} into a string and encoding as bytes", &self);
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `ADSBMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        trace!(
            "Converting {:?} into a string, appending a newline and encoding as bytes",
            &self
        );
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Returns the number of aircraft in the message.
    ///
    /// the output is a `usize`.

    pub fn len(&self) -> usize {
        match self {
            ADSBMessage::JSONMessage(_) => 1,
            ADSBMessage::AircraftJSON(aircraft_json) => aircraft_json.aircraft.len(),
            ADSBMessage::AdsbRawMessage(_) => 1, // FIXME: this ain't right
        }
    }
}

/// This will automagically serialise to either TODO: Fix the docs here.
///
/// This simplifies the handling of messaging by not needing to identify it first.
/// It handles identification by looking at the provided data and seeing which format matches it best.
#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ADSBMessage {
    #[cfg(feature = "json")]
    JSONMessage(JSONMessage),
    #[cfg(feature = "json")]
    AircraftJSON(AircraftJSON),
    #[cfg(feature = "raw")]
    AdsbRawMessage(AdsbRawMessage),
}

impl Default for ADSBMessage {
    fn default() -> Self {
        ADSBMessage::JSONMessage(JSONMessage::default())
    }
}
