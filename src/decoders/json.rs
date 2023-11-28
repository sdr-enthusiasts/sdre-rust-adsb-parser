// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

use super::json_types::{
    adsbversion::ADSBVersion, altitude::Altitude, barorate::BaroRate,
    calculatedbestflightid::CalculatedBestFlightID, dbflags::DBFlags, emergency::Emergency,
    emmittercategory::EmitterCategory, flightstatus::FlightStatusAlertBit,
    lastknownposition::LastKnownPosition, nacp::NavigationIntegrityCategory,
    navigationmodes::NavigationModes, sourceintegritylevel::SourceIntegrityLevelType,
};

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `JSONMessage`.
pub trait NewJSONMessage {
    fn to_json(&self) -> MessageResult<JSONMessage>;
}

/// Implementing `.to_json()` for the type `String`.
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

/// Supporting `.to_json()` for the type `str`.
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

/// Supporting `.to_json()` for the type `[u8]`.
///
/// This does not consume the `[u8]`.

impl NewJSONMessage for [u8] {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_slice(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_json()` for the type `Vec<u8>`.
///
/// This does not consume the `Vec<u8>`.

impl NewJSONMessage for Vec<u8> {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_slice(self) {
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

// Not all messages have a timestamp, so we'll use the current time if one isn't provided.
fn get_timestamp() -> f64 {
    match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => n.as_secs_f64(),
        Err(_) => 0.0,
    }
}

// https://github.com/wiedehopf/readsb/blob/dev/README-json.md

/// The JSON message format.
/// This is for a single aircraft of JSON data.
/// TODO: There is a metric load of "Option" types here. 99.9% of the time they are present in
/// the payload. It may be well worth it to remove the Option types and just use the default,
/// or see if the message structure is consistent if certain fields are missing and create a different
/// struct for those messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default)]
#[serde(deny_unknown_fields)]
pub struct JSONMessage {
    /// The timestamp of the message in seconds since the epoch.
    #[serde(rename = "now", default = "get_timestamp")]
    pub timestamp: f64,
    /// The Flight Status bit field. 2.2.3.2.3.2
    #[serde(skip_serializing_if = "Option::is_none", rename = "alert")]
    pub flight_status: Option<FlightStatusAlertBit>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_baro")]
    /// Aircraft altitude reported from the barometric altimeter.
    pub barometric_altitude: Option<Altitude>,
    /// Aircraft altitude reported from the GNSS/INS system on the aircraft
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_geom")]
    pub geometric_altitude: Option<i32>,
    /// Rate of change in the barometric altitude in feet per minute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baro_rate")]
    pub barometric_altitude_rate: Option<BaroRate>,
    /// Emitter category to identify the aircraft or vehicle class. 2.2.3.2.5.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<EmitterCategory>,
    /// Wiedehopf's aircraft.json indicator for interesting aircraft.
    /// Possible Values are military, interesting, PIA and LADD.
    #[serde(skip_serializing, rename = "dbFlags")]
    pub db_flags: Option<DBFlags>,
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
    pub navigation_accuracy_position: Option<NavigationIntegrityCategory>,
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
    pub sil_type: Option<SourceIntegrityLevelType>,
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
    pub version: Option<ADSBVersion>,
}

/// The JSON message readsb provided aircraft.json format.
/// This file is a list of JSONMessage with some additional metadata provided.

// TODO: When deserializing no planes in this format will include a timestamp field.
// However, AircraftJSON provides it. We should inject that timestamp field in to the
// JSONMessage before deserializing it.
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

#[cfg(test)]
mod tests {
    use crate::DecodeMessage;
    use std::fs::{read_dir, read_to_string, File};
    use std::io::BufRead;

    #[test]
    fn decode_json_message_as_aircraft_json() {
        // open all aircraft_*.json files in test data. convert to JSONMessage and then back to string
        let test_data: std::fs::ReadDir = read_dir("test_data").unwrap();

        for entry in test_data {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: std::path::PathBuf = entry.path();
            if path.is_file() {
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("aircraft_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file: String = read_to_string(&path).unwrap();
                    // count the number of "hex" fields in the file
                    let mut hex_count: usize = 0;
                    file.lines().for_each(|l: &str| {
                        if l.contains("\"hex\":") {
                            hex_count += 1;
                        }
                    });
                    let result = file.decode_message();
                    assert!(result.is_ok(), "Failed to decode JSONMessage {:?}", result);
                    let found_count: usize = result.unwrap().len();

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
        let test_data: std::fs::ReadDir = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: std::path::PathBuf = entry.path();
            if path.is_file() {
                let mut line_number: i32 = 1;
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file: File = File::open(path).unwrap();
                    let reader: std::io::BufReader<File> = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader
                        .lines()
                        .for_each(|l: Result<String, std::io::Error>| {
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
