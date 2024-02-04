//! This module contains the implementation of a Rust ADS-B parser.
//!
//! The parser supports decoding messages in JSON, Beast, and Raw formats.
//! It provides functionality for decoding strings, byte slices, and vectors of bytes
//! into `ADSBMessage` structs, which represent the parsed ADS-B messages.
//!
//! # Example
//!
//! ```
//! use log::info;
//! use sdre_rust_adsb_parser::DecodeMessage;
//! use sdre_rust_logging::SetupLogging;
//!
//! 3u8.enable_logging();
//!
//! let message = "8EADC035002D7000000000B02845";
//!
//! let decoded_message = message.decode_message().unwrap();
//!
//! match decoded_message {
//!     sdre_rust_adsb_parser::ADSBMessage::JSONMessage(aircraft) => {
//!         info!("ICAO: {}", aircraft.transponder_hex);
//!         info!("Altitude: {:?} feet", aircraft.barometric_altitude);
//!         info!("Latitude: {:?}", aircraft.latitude);
//!         info!("Longitude: {:?}", aircraft.longitude);
//!     }
//!     _ => {
//!         info!("Invalid message type");
//!     }
//! }
//! ```
//!
//! For more information on the supported message formats and decoding options,
//! please refer to the documentation of the `DecodeMessage` trait.
//!
//! # License
//!
//! This code is licensed under the MIT License. See the [LICENSE](https://opensource.org/licenses/MIT) file for more information.
//!
//! # Acknowledgements
//!
//! This ADS-B parser is developed by Frederick Clausen II.
//! Special thanks to the contributors and the open-source community for their support.

// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![warn(clippy::pedantic)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate log;

use core::fmt;

use decoders::beast::AdsbBeastMessage;
use error_handling::deserialization_error::{DeserializationError, WrongType};

#[cfg(feature = "json")]
use decoders::aircraftjson::AircraftJSON;
#[cfg(feature = "json")]
use decoders::json::JSONMessage;
#[cfg(feature = "raw")]
use decoders::raw::AdsbRawMessage;
use deku::prelude::*;
use serde::{Deserialize, Serialize};
pub mod decoders {
    pub mod rawtojson;
    #[cfg(feature = "raw")]
    pub mod raw_types {
        pub mod ac13field;
        pub mod adsb;
        pub mod adsbversion;
        pub mod airbornevelocity;
        pub mod airbornevelocitysubfields;
        pub mod airbornevelocitysubtype;
        pub mod airbornevelocitytype;
        pub mod aircraftstatus;
        pub mod aircraftstatustype;
        pub mod airspeeddecoding;
        pub mod altitude;
        pub mod autopilot_modes;
        pub mod bds;
        pub mod capability;
        pub mod capabilityclassairborne;
        pub mod capabilityclasssurface;
        pub mod controlfield;
        pub mod controlfieldtype;
        pub mod cprheaders;
        pub mod datalinkcapability;
        pub mod df;
        pub mod direction_nsew;
        pub mod downlinkrequest;
        pub mod emergencystate;
        pub mod flightstatus;
        pub mod fms;
        pub mod groundspeed;
        pub mod groundspeeddecoding;
        pub mod heading;
        pub mod helper_functions;
        pub mod icao;
        pub mod identification;
        pub mod identitycode;
        pub mod ke;
        pub mod me;
        pub mod modevalidity;
        pub mod noposition;
        pub mod operationalmode;
        pub mod operationcodesurface;
        pub mod operationstatus;
        pub mod operationstatusairborne;
        pub mod operationstatussurface;
        pub mod sign;
        pub mod signbitgnssbaroaltitudesdiff;
        pub mod signbitverticalrate;
        pub mod sourcebitverticalrate;
        pub mod statusforgroundtrack;
        pub mod surfaceposition;
        pub mod targetstateandstatusinformation;
        pub mod typecoding;
        pub mod utilitymessage;
        pub mod utilitymessagetype;
        pub mod verticleratesource;
    }
    #[cfg(feature = "beast")]
    pub mod beast;
    #[cfg(feature = "beast")]
    pub mod beast_types {
        pub mod messagetype;
    }
    #[cfg(feature = "json")]
    pub mod aircraftjson;
    #[cfg(feature = "json")]
    pub mod json;
    #[cfg(feature = "json")]
    pub mod json_types {
        pub mod adsbversion;
        pub mod altimeter;
        pub mod altitude;
        pub mod barorate;
        pub mod calculatedbestflightid;
        pub mod dbflags;
        pub mod emergency;
        pub mod emmittercategory;
        pub mod geometricverticalaccuracy;
        pub mod heading;
        pub mod lastknownposition;
        pub mod latitude;
        pub mod longitude;
        pub mod messagetype;
        pub mod meters;
        pub mod mlat;
        pub mod nacp;
        pub mod nacv;
        pub mod navigationmodes;
        pub mod receivedmessages;
        pub mod secondsago;
        pub mod signalpower;
        pub mod sil;
        pub mod sourceintegritylevel;
        pub mod speed;
        pub mod squawk;
        pub mod timestamp;
        pub mod tisb;
        pub mod transponderhex;
    }
    pub mod common_types {
        pub mod sda;
        pub mod surveillancestatus;
    }
    #[cfg(feature = "raw")]
    pub mod raw;
    pub mod helpers {
        pub mod cpr_calculators;
        pub mod prettyprint;
        pub mod time;
    }
}

pub mod error_handling {
    pub mod adsb_beast_error;
    pub mod adsb_json_error;
    pub mod adsb_raw_error;
    pub mod deserialization_error;
}

pub mod helpers {
    pub mod encode_adsb_beast_input;
    pub mod encode_adsb_json_input;
    pub mod encode_adsb_raw_input;
}

pub mod data_structures {
    pub mod airplane;
}

pub mod state_machine {
    pub mod state;
}

/// Common return type for all serialisation/deserialisation functions.
///
/// This serves as a wrapper for `serde_json::Error` as the Error type.
pub type MessageResult<T> = Result<T, DeserializationError>;

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON, Beast or Raw format. Vectors of bytes are also supported.
pub trait DecodeMessage {
    /// Decodes the message and returns it as an `ADSBMessage` struct.
    /// # Errors
    /// This function will return an error if the message is not in JSON, Beast, or Raw format.
    fn decode_message(&self) -> MessageResult<ADSBMessage>;
    /// Decodes the message and returns it as an `AircraftJSON` struct.
    /// # Errors
    /// This function will return an error if the message is not an aircraft.
    fn decode_message_as_aircraft(&self) -> MessageResult<AircraftJSON> {
        if let ADSBMessage::AircraftJSON(aircraft_json) = self.decode_message()? {
            Ok(aircraft_json)
        } else {
            let error = WrongType::WrongTypeForAircraft {
                message: "The message is not an aircraft".to_string(),
            };

            Err(error.into())
        }
    }
}

/// Provides functionality for decoding a `String` to `ADSBMessage`.
///
/// This does not consume the `String`.
impl DecodeMessage for String {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        let error_serde: DeserializationError = match serde_json::from_str(self) {
            Ok(v) => return Ok(v),
            Err(e) => e.into(),
        };

        let bytes: Vec<u8> = match hex::decode(self) {
            Ok(v) => v,
            Err(e) => {
                // return e and serde error
                // we can't attempt to use the other decoders here, because we didn't get sane bytes
                return Err(DeserializationError::CombinedError(vec![
                    error_serde,
                    e.into(),
                ]));
            }
        };
        // try to decode it as a raw frame
        let error_raw: DeserializationError = match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbRawMessage(body)),
            Err(e) => e.into(),
        };

        let error_beast: DeserializationError = match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbBeastMessage(body)),
            Err(e) => e.into(),
        };

        // create a combined error
        let errors: Vec<DeserializationError> = vec![error_serde, error_raw, error_beast];
        Err(DeserializationError::CombinedError(errors))
    }

    fn decode_message_as_aircraft(&self) -> MessageResult<AircraftJSON> {
        if let ADSBMessage::AircraftJSON(aircraft_json) = self.decode_message()? {
            Ok(aircraft_json)
        } else {
            let error = WrongType::WrongTypeForAircraft {
                message: "The message is not an aircraft".to_string(),
            };

            Err(error.into())
        }
    }
}

/// Provides functionality for decoding a `str` to `ADSBMessage`.
///
/// This does not consume the `str`.
impl DecodeMessage for str {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        let error_serde: DeserializationError = match serde_json::from_str(self) {
            Ok(v) => return Ok(v),
            Err(e) => e.into(),
        };

        let bytes: Vec<u8> = match hex::decode(self) {
            Ok(v) => v,
            Err(e) => {
                // return e and serde error
                // we can't attempt to use the other decoders here, because we didn't get sane bytes
                return Err(DeserializationError::CombinedError(vec![
                    error_serde,
                    e.into(),
                ]));
            }
        };
        // try to decode it as a raw frame
        let error_raw: DeserializationError = match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbRawMessage(body)),
            Err(e) => e.into(),
        };

        let error_beast: DeserializationError = match AdsbBeastMessage::from_bytes((&bytes, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbBeastMessage(body)),
            Err(e) => e.into(),
        };

        // create a combined error
        let errors: Vec<DeserializationError> = vec![error_serde, error_raw, error_beast];
        Err(DeserializationError::CombinedError(errors))
    }

    fn decode_message_as_aircraft(&self) -> MessageResult<AircraftJSON> {
        if let ADSBMessage::AircraftJSON(aircraft_json) = self.decode_message()? {
            Ok(aircraft_json)
        } else {
            let error = WrongType::WrongTypeForAircraft {
                message: "The message is not an aircraft".to_string(),
            };

            Err(error.into())
        }
    }
}

/// Provides functionality for decoding a `&[u8]` to `ADSBMessage`.
///
/// This does not consume the `&[u8]`.
impl DecodeMessage for &[u8] {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        let error_beast: DeserializationError = match AdsbBeastMessage::from_bytes((&self, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbBeastMessage(body)),
            Err(e) => e.into(),
        };

        // try to decode it as a raw frame
        let error_raw: DeserializationError = match AdsbRawMessage::from_bytes((&self, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbRawMessage(body)),
            Err(e) => e.into(),
        };

        let error_serde: DeserializationError = match serde_json::from_slice(self) {
            Ok(v) => return Ok(v),
            Err(e) => e.into(),
        };

        // create a combined error
        let errors: Vec<DeserializationError> = vec![error_serde, error_raw, error_beast];
        Err(DeserializationError::CombinedError(errors))
    }

    fn decode_message_as_aircraft(&self) -> MessageResult<AircraftJSON> {
        if let ADSBMessage::AircraftJSON(aircraft_json) = self.decode_message()? {
            Ok(aircraft_json)
        } else {
            let error = WrongType::WrongTypeForAircraft {
                message: "The message is not an aircraft".to_string(),
            };

            Err(error.into())
        }
    }
}

impl DecodeMessage for Vec<u8> {
    fn decode_message(&self) -> MessageResult<ADSBMessage> {
        let error_beast: DeserializationError = match AdsbBeastMessage::from_bytes((&self, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbBeastMessage(body)),
            Err(e) => e.into(),
        };

        // try to decode it as a raw frame
        let error_raw: DeserializationError = match AdsbRawMessage::from_bytes((&self, 0)) {
            Ok((_, body)) => return Ok(ADSBMessage::AdsbRawMessage(body)),
            Err(e) => e.into(),
        };

        let error_serde: DeserializationError = match serde_json::from_slice(self) {
            Ok(v) => return Ok(v),
            Err(e) => e.into(),
        };

        // create a combined error
        let errors: Vec<DeserializationError> = vec![error_serde, error_raw, error_beast];
        Err(DeserializationError::CombinedError(errors))
    }

    fn decode_message_as_aircraft(&self) -> MessageResult<AircraftJSON> {
        if let ADSBMessage::AircraftJSON(aircraft_json) = self.decode_message()? {
            Ok(aircraft_json)
        } else {
            let error = WrongType::WrongTypeForAircraft {
                message: "The message is not an aircraft".to_string(),
            };

            Err(error.into())
        }
    }
}

impl fmt::Display for ADSBMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ADSBMessage::JSONMessage(json_message) => write!(f, "{json_message}"),
            ADSBMessage::AircraftJSON(aircraft_json) => write!(f, "{aircraft_json}"),
            ADSBMessage::AdsbRawMessage(adsb_raw_message) => write!(f, "{adsb_raw_message}"),
            ADSBMessage::AdsbBeastMessage(adsb_beast_message) => {
                write!(f, "{adsb_beast_message}")
            }
        }
    }
}

/// Implementation of `ADSBMessage`.
impl ADSBMessage {
    /// Converts `ADSBMessage` to `String`.
    /// # Errors
    /// This function will return an error if the conversion to a string fails.
    pub fn to_string(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string", &self);
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `ADSBMessage` to `String` and appends a `\n` to the end.
    /// # Errors
    /// This function will return an error if the conversion to a string fails.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        trace!("Converting {:?} to a string and appending a newline", &self);
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{string}\n")),
        }
    }

    /// Converts `ADSBMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    /// # Errors
    /// This function will return an error if the conversion to a string fails.
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
    /// # Errors
    /// This function will return an error if the conversion to a string fails.
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

    #[must_use]
    pub fn pretty_print(&self) -> String {
        match self {
            ADSBMessage::JSONMessage(json_message) => json_message.pretty_print(),
            ADSBMessage::AircraftJSON(aircraft_json) => aircraft_json.pretty_print(),
            ADSBMessage::AdsbRawMessage(adsb_raw_message) => adsb_raw_message.pretty_print(),
            ADSBMessage::AdsbBeastMessage(adsb_beast_message) => adsb_beast_message.pretty_print(),
        }
    }

    /// Returns the number of aircraft in the message.
    ///
    /// the output is a `usize`.

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            ADSBMessage::AircraftJSON(aircraft_json) => aircraft_json.len(),
            _ => 1,
        }
    }

    /// Returns `true` if the message is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            ADSBMessage::AircraftJSON(aircraft_json) => aircraft_json.is_empty(),
            _ => false,
        }
    }
}

/// This will automagically serialise to either
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
    #[cfg(feature = "beast")]
    AdsbBeastMessage(AdsbBeastMessage),
}

impl Default for ADSBMessage {
    fn default() -> Self {
        ADSBMessage::JSONMessage(JSONMessage::default())
    }
}
