extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;

use crate::json::{AircraftJSON, JSONMessage};
use serde::{Deserialize, Serialize};

pub mod json;

/// Common return type for all serialisation/deserialisation functions.
///
/// This serves as a wrapper for `serde_json::Error` as the Error type.
pub type MessageResult<T> = Result<T, serde_json::Error>;

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
        serde_json::from_str(self)
    }
}

/// Provides functionality for decoding a `str` to `ADSBMessage`.
///
/// This does not consume the `str`.
impl DecodeMessage for str {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        serde_json::from_str(self)
    }
}

/// Implementation of `ADSBMessage`.
impl ADSBMessage {
    /// Converts `ADSBMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string", &self);
        serde_json::to_string(self)
    }

    /// Converts `ADSBMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string and appending a newline", &self);
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error),
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
    JSONMessage(JSONMessage),
    AircraftJSON(AircraftJSON),
}

impl Default for ADSBMessage {
    fn default() -> Self {
        ADSBMessage::JSONMessage(JSONMessage::default())
    }
}