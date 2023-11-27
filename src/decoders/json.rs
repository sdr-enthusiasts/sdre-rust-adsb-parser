// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::fmt;

// TODO: Figure out NIC and create enum for it

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `AcarsMessage`.
pub trait NewJSONMessage {
    fn to_json(&self) -> MessageResult<JSONMessage>;
}

/// Implementing `.to_acars()` for the type `String`.
///
/// This does not consume the `String`.
impl NewJSONMessage for String {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_acars()` for the type `str`.
///
/// This does not consume the `str`.
impl NewJSONMessage for str {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl fmt::Display for JSONMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string().unwrap())
    }
}

impl JSONMessage {
    /// Converts `JSONMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `JSONMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `JSONMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `JSONMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }
}

// https://github.com/wiedehopf/readsb/blob/dev/README-json.md

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default)]
#[serde(deny_unknown_fields)]
pub struct JSONMessage {
    #[serde(skip_serializing_if = "Option::is_none", rename = "now")]
    pub timestamp: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alert")]
    pub flight_status_bit_alert: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_baro")]
    pub barometric_altitude: Option<Altitude>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_geom")]
    pub geometric_altitude: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "baro_rate")]
    pub barometric_altitude_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calc_track: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<EmitterCategory>,
    #[serde(skip_serializing, rename = "dbFlags")]
    pub db_flags: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency: Option<Emergency>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "flight")]
    pub calculated_best_flight_id: Option<CalculatedBestFlightID>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "geom_rate")]
    pub geometric_altitude_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gs")]
    pub ground_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gva")]
    pub geometric_verticle_accuracy: Option<u8>,
    #[serde(rename = "hex")]
    pub transponder_hex: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastPosition")]
    pub last_known_position: Option<LastKnownPosition>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lat")]
    pub latitude: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lon")]
    pub longitude: Option<f32>,
    #[serde(rename = "messages")]
    pub number_of_received_messages: i32,
    pub mlat: Vec<String>, // TODO: Figure out what this is
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_p")]
    pub navigation_accuracy_position: Option<u8>, // TODO: should this be an enum?
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_v")]
    pub navigation_accuracy_velocity: Option<u8>, // TODO: should this be an enum?
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_mcp")]
    pub autopilot_selected_altitude: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_heading")]
    pub autopilot_selected_heading: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_fms")]
    pub flight_management_system_selected_altitude: Option<i32>, // TODO: this naming convention for autopilot and fms stuff kinda sux
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_modes")]
    pub autopilot_modes: Option<Vec<NavigationModes>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_qnh")]
    pub selected_altimeter: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic")]
    pub naviation_integrity_category: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic_baro")]
    pub barometeric_altitude_integrity_category: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "r")]
    pub aircraft_registration_from_database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dir")]
    pub aircraft_direction_from_receiving_station: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dst")]
    pub aircract_distance_from_receiving_station: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "rc")]
    pub radius_of_containment: Option<i32>,
    pub rssi: f32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sda")]
    pub system_design_assurance: Option<i32>, // TODO: should this be an enum?
    #[serde(rename = "seen")]
    pub last_time_seen: f32,
    #[serde(skip_serializing_if = "Option::is_none", rename = "seen_pos")]
    pub last_time_seen_alt: Option<f32>, // FIXME: Do we need this? It's the same as last_time_seen maybe?
    #[serde(skip_serializing_if = "Option::is_none", rename = "sil")]
    pub source_integrity_level: Option<u8>, // TODO: should this be an enum?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sil_type: Option<SourceIntegrityLevel>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spi")]
    pub flight_status_special_position_id_bit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "squawk")]
    pub transponder_squawk_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "t")]
    pub aircraft_type_from_database: Option<String>,
    pub tisb: Vec<String>, // TODO: this should def be an enum
    #[serde(skip_serializing_if = "Option::is_none", rename = "track")]
    pub true_track_over_ground: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_heading: Option<f32>,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct AircraftJSON {
    pub now: f32,
    pub messages: i32,
    pub aircraft: Vec<JSONMessage>,
}

impl fmt::Display for AircraftJSON {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aircraft.len())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum CalculatedBestFlightID {
    String(String),
}

impl Default for CalculatedBestFlightID {
    fn default() -> Self {
        Self::String("".to_string())
    }
}

impl fmt::Display for CalculatedBestFlightID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatedBestFlightID::String(flight_id) => write!(f, "{}", flight_id.trim()),
        }
    }
}

impl fmt::Debug for CalculatedBestFlightID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatedBestFlightID::String(flight_id) => fmt::Display::fmt(&flight_id.trim(), f),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Altitude {
    I32(i32),
    String(String),
}

impl Default for Altitude {
    fn default() -> Self {
        Self::I32(0)
    }
}

impl fmt::Display for Altitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Altitude::I32(altitude) => write!(f, "{}", altitude),
            Altitude::String(_) => write!(f, "Ground"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize_enum_str, Serialize_enum_str)]
// #[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Emergency {
    none,
    general,
    lifeguard,
    minfuel,
    nordo,
    unlawful,
    downed,
    reserved,
}

impl Default for Emergency {
    fn default() -> Self {
        Self::none
    }
}

// emitter category https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize_enum_str, Serialize_enum_str)]
pub enum EmitterCategory {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

impl Default for EmitterCategory {
    fn default() -> Self {
        Self::A0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct LastKnownPosition {
    // lat, lon, nic, rc, seen_pos
    #[serde(rename = "lat")]
    latitude: f32,
    #[serde(rename = "lon")]
    longitude: f32,
    #[serde(rename = "nic")]
    naviation_integrity_category: i32,
    #[serde(rename = "rc")]
    radius_of_containment: i32,
    #[serde(rename = "seen_pos")]
    last_time_seen: f32,
}

impl Default for LastKnownPosition {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            naviation_integrity_category: 0,
            radius_of_containment: 0,
            last_time_seen: 0.0,
        }
    }
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum NavigationModes {
    autopilot,
    vnav,
    althold,
    approach,
    lnav,
    tcas,
    none,
}

impl Default for NavigationModes {
    fn default() -> Self {
        Self::none
    }
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum SourceIntegrityLevel {
    unknown,
    persample,
    perhour,
}

impl Default for SourceIntegrityLevel {
    fn default() -> Self {
        Self::unknown
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
// pub struct SquawkCode {
//     digit_1: u8,
//     digit_2: u8,
//     digit_3: u8,
//     digit_4: u8,
// }

// impl SquawkCode {
//     pub fn new(code: String) -> Self {
//         let mut squawk_code = Self {
//             digit_1: 0,
//             digit_2: 0,
//             digit_3: 0,
//             digit_4: 0,
//         };

//         let mut chars = code.chars();
//         // FIXME: should this validate we're in the range 0 - 8?
//         squawk_code.digit_1 = chars.next().unwrap_or('0').to_digit(10).unwrap_or(0) as u8;
//         squawk_code.digit_2 = chars.next().unwrap_or('0').to_digit(10).unwrap_or(0) as u8;
//         squawk_code.digit_3 = chars.next().unwrap_or('0').to_digit(10).unwrap_or(0) as u8;
//         squawk_code.digit_4 = chars.next().unwrap_or('0').to_digit(10).unwrap_or(0) as u8;

//         squawk_code
//     }
// }

// impl fmt::Display for SquawkCode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}{}{}{}",
//             self.digit_1, self.digit_2, self.digit_3, self.digit_4
//         )
//     }
// }

// #[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, PartialOrd)]
// pub enum ADSBVersion {
//     Version0 = 0,
//     Version1 = 1,
//     Version2 = 2,
//     Version3 = 3,
//     Version4 = 4,
//     Version5 = 5,
//     Version6 = 6,
//     Version7 = 7,
// }

// impl Default for ADSBVersion {
//     fn default() -> Self {
//         Self::Version0
//     }
// }

#[cfg(test)]
mod tests {
    use crate::DecodeMessage;
    use std::fs::{read_dir, read_to_string, File};
    use std::io::BufRead;

    #[test]
    fn decode_json_message_as_aircraft_json() {
        // open all aircraft_*.json files in test data. convert to JSONMessage and then back to string
        let test_data = read_dir("test_data").unwrap();

        for entry in test_data {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("aircraft_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file = read_to_string(&path).unwrap();
                    // count the number of "hex" fields in the file
                    let mut hex_count = 0;
                    file.lines().for_each(|l| {
                        if l.contains("\"hex\":") {
                            hex_count += 1;
                        }
                    });
                    let result = file.decode_message();
                    assert!(result.is_ok(), "Failed to decode JSONMessage {:?}", result);
                    let found_count = result.unwrap().len();

                    assert_eq!(
                        hex_count, found_count,
                        "Found {} hex fields but {} aircraft",
                        hex_count, found_count
                    );
                }
            }
        }
    }

    #[test]
    fn decode_json_message_individually() {
        // open all json_*.json files in test data. convert to JSONMessage and then back to string
        let test_data = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let mut line_number = 1;
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file = File::open(path).unwrap();
                    let reader = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader.lines().for_each(|l| {
                        line = l.unwrap();

                        // if the line starts with anything but a {, skip it
                        if line.starts_with("{") && line.trim().len() > 0 {
                            // encode the line as JSONMessage
                            // remove the trailing newline and any other characters after the '}'
                            let final_message_to_process = line.trim().trim_end_matches(',');
                            assert!(
                                final_message_to_process.ends_with("}"),
                                "Line {} in file does not end with a curly bracket",
                                line_number
                            );
                            let json_message = final_message_to_process.decode_message();

                            assert!(
                                json_message.is_ok(),
                                "Failed to decode JSONMessage {:?}",
                                final_message_to_process
                            );
                        }
                        line_number += 1;
                    });
                }
            }
        }
    }
}
