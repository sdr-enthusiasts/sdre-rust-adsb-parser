// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{
    helpers::prettyprint::{pretty_print_field, pretty_print_label},
    json::JSONMessage,
};

pub trait NewAircraftJSONMessage {
    fn to_aircraft_json(&self) -> MessageResult<AircraftJSON>;
}

impl NewAircraftJSONMessage for String {
    fn to_aircraft_json(&self) -> MessageResult<AircraftJSON> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl NewAircraftJSONMessage for str {
    fn to_aircraft_json(&self) -> MessageResult<AircraftJSON> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl NewAircraftJSONMessage for Vec<u8> {
    fn to_aircraft_json(&self) -> MessageResult<AircraftJSON> {
        match serde_json::from_slice(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl NewAircraftJSONMessage for &Vec<u8> {
    fn to_aircraft_json(&self) -> MessageResult<AircraftJSON> {
        match serde_json::from_slice(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// The JSON message readsb provided aircraft.json format.
/// This file is a list of JSONMessage with some additional metadata provided.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct AircraftJSON {
    #[serde(rename = "now")]
    pub timestamp: f32,
    pub messages: u64,
    pub aircraft: Vec<JSONMessage>,
}

impl AircraftJSON {
    /// Create a new `AircraftJSON` object from a `Vec<JSONMessage>` and a `u64`.
    pub fn new(aircraft: Vec<JSONMessage>, total_messages: u64) -> AircraftJSON {
        AircraftJSON {
            timestamp: chrono::Utc::now().timestamp() as f32,
            messages: total_messages,
            aircraft,
        }
    }
    /// Converts `AircraftJSON` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `AircraftJSON` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `AircraftJSON` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `AircraftJSON` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    pub fn pretty_print(&self) -> String {
        let mut output: String = String::new();

        pretty_print_label("Aircraft JSON", &mut output);
        pretty_print_field("Time", &self.timestamp, &mut output);
        pretty_print_field("Messages", &self.messages, &mut output);
        pretty_print_label("Aircraft", &mut output);

        for aircraft in &self.aircraft {
            output.push_str(&aircraft.pretty_print());
        }

        output
    }

    pub fn len(&self) -> usize {
        self.aircraft.len()
    }

    pub fn is_empty(&self) -> bool {
        self.aircraft.is_empty()
    }
}

impl fmt::Display for AircraftJSON {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aircraft.len())
    }
}
