// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::MessageResult;
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

use super::json_types::{
    adsbversion::ADSBVersion,
    altitude::Altitude,
    barorate::BaroRate,
    calculatedbestflightid::CalculatedBestFlightID,
    dbflags::DBFlags,
    emergency::Emergency,
    emmittercategory::EmitterCategory,
    flightstatus::FlightStatusAlertBit,
    heading::Heading,
    lastknownposition::LastKnownPosition,
    latitude::Latitude,
    longitude::Longitude,
    messagetype::MessageType,
    meters::{Meters, NauticalMiles},
    nacp::NavigationIntegrityCategory,
    nacv::NavigationAccuracyVelocity,
    navigationmodes::NavigationModes,
    secondsago::SecondsAgo,
    signalpower::SignalPower,
    sil::SourceIntegrityLevel,
    sourceintegritylevel::SourceIntegrityLevelType,
    speed::Speed,
    squawk::Squawk,
    timestamp::TimeStamp,
    transponderhex::TransponderHex,
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
fn get_timestamp() -> TimeStamp {
    match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => TimeStamp::from(n.as_secs_f64()),
        Err(_) => TimeStamp::default(),
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
    pub timestamp: TimeStamp,
    /// The Flight Status bit field. 2.2.3.2.3.2
    #[serde(skip_serializing_if = "Option::is_none", rename = "alert")]
    pub flight_status: Option<FlightStatusAlertBit>, // FIXME: I doubt this is right
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_baro")]
    /// Aircraft altitude reported from the barometric altimeter.
    pub barometric_altitude: Option<Altitude>,
    /// Aircraft altitude reported from the GNSS/INS system on the aircraft
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_geom")]
    pub geometric_altitude: Option<Altitude>,
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
    /// ADS-B emergency/priority status, a superset of the 7x00 squawks (2.2.3.2.7.8.1.1)
    /// (none, general, lifeguard, minfuel, nordo, unlawful, downed, reserved)
    pub emergency: Option<Emergency>,
    /// The aircraft callsign, Flight Name, or Tail Number. Most likely the id used by air traffic control.
    /// to interact with the flight. (2.2.8.2.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "flight")]
    pub calculated_best_flight_id: Option<CalculatedBestFlightID>,
    /// Rate of change of geometric (GNSS / INS) altitude, feet/minute
    #[serde(skip_serializing_if = "Option::is_none", rename = "geom_rate")]
    pub geometric_altitude_rate: Option<BaroRate>,
    /// Ground speed in knots.
    #[serde(skip_serializing_if = "Option::is_none", rename = "gs")]
    pub ground_speed: Option<Speed>,
    /// Geometric Vertical Accuracy (2.2.3.2.7.2.8)
    #[serde(skip_serializing_if = "Option::is_none", rename = "gva")]
    pub geometric_verticle_accuracy: Option<u8>, // FIXME: I doubt this is right
    /// The transponder hex identifier of the aircraft.
    #[serde(rename = "hex")]
    pub transponder_hex: TransponderHex,
    /// {lat, lon, nic, rc, seen_pos} when the regular lat and lon are older than 60 seconds they are no longer considered valid,
    /// this will provide the last position and show the age for the last position. aircraft will only be in the aircraft json
    /// if a position has been received in the last 60 seconds or if any message has been received in the last 30 seconds.
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastPosition")]
    pub last_known_position: Option<LastKnownPosition>,
    /// The aircraft latitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "lat")]
    pub latitude: Option<Latitude>,
    /// The aircraft longitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "lon")]
    pub longitude: Option<Longitude>,
    /// The number of messages received for this aircraft.
    #[serde(rename = "messages")]
    pub number_of_received_messages: i32,
    /// list of fields derived from MLAT data
    pub mlat: Vec<String>, // FIXME: I doubt this is right
    /// Navigation Accuracy for Position (2.2.5.1.35)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_p")]
    pub navigation_accuracy_position: Option<NavigationIntegrityCategory>, // FIXME: I doubt this is right
    /// Navigation Accuracy for Velocity (2.2.5.1.19)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_v")]
    pub navigation_accuracy_velocity: Option<NavigationAccuracyVelocity>, // FIXME: I doubt this is right
    /// selected altitude from the Mode Control Panel / Flight Control Unit (MCP/FCU) or equivalent equipment
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_mcp")]
    pub autopilot_selected_altitude: Option<Altitude>,
    /// selected heading (True or Magnetic is not defined in DO-260B, mostly Magnetic as that is the de facto standard) (2.2.3.2.7.1.3.7)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_heading")]
    pub autopilot_selected_heading: Option<Heading>,
    /// selected altitude from the Flight Manaagement System (FMS) (2.2.3.2.7.1.3.3)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_fms")]
    pub flight_management_system_selected_altitude: Option<Altitude>,
    /// set of engaged automation modes: 'autopilot', 'vnav', 'althold', 'approach', 'lnav', 'tcas'
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_modes")]
    pub autopilot_modes: Option<Vec<NavigationModes>>,
    /// altimeter setting (QFE or QNH/QNE), hPa
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_qnh")]
    pub selected_altimeter: Option<f32>,
    /// Navigation Integrity Category (2.2.3.2.7.2.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic")]
    pub naviation_integrity_category: Option<NavigationIntegrityCategory>, // FIXME: I doubt this is right
    /// Navigation Integrity Category for Barometric Altitude (2.2.5.1.35)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic_baro")]
    pub barometeric_altitude_integrity_category: Option<u8>, // FIXME: I doubt this is right
    #[serde(skip_serializing_if = "Option::is_none", rename = "r")]
    /// Wiedehopf's aircraft.json aircraft registration pulled from database
    pub aircraft_registration_from_database: Option<String>,
    /// distance from supplied center point in nmi
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dir")]
    pub aircraft_direction_from_receiving_station: Option<Heading>,
    /// true direction of the aircraft from the supplied center point (degrees)
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dst")]
    pub aircract_distance_from_receiving_station: Option<NauticalMiles>,
    /// Radius of Containment, meters; a measure of position integrity derived from NIC & supplementary bits. (2.2.3.2.7.2.6, Table 2-69)
    #[serde(skip_serializing_if = "Option::is_none", rename = "rc")]
    pub radius_of_containment: Option<Meters>,
    /// recent average RSSI (signal power), in dbFS; this will always be negative.
    pub rssi: SignalPower,
    /// System Design Assurance (2.2.3.2.7.2.4.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "sda")]
    pub system_design_assurance: Option<i32>, // FIXME: I doubt this is right
    /// how long ago (in seconds before "now") a message was last received from this aircraft
    #[serde(rename = "seen")]
    pub last_time_seen: SecondsAgo,
    /// how long ago (in seconds before "now") the position was last updated
    #[serde(skip_serializing_if = "Option::is_none", rename = "seen_pos")]
    pub last_time_seen_pos_andalt: Option<f32>,
    /// Source Integity Level (2.2.5.1.40)
    #[serde(skip_serializing_if = "Option::is_none", rename = "sil")]
    pub source_integrity_level: Option<SourceIntegrityLevel>, // FIXME: I doubt this is right
    /// interpretation of SIL: unknown, perhour, persample
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sil_type: Option<SourceIntegrityLevelType>, // FIXME: I doubt this is right
    /// Flight status special position identification bit (2.2.3.2.3.2)
    #[serde(skip_serializing_if = "Option::is_none", rename = "spi")]
    pub flight_status_special_position_id_bit: Option<u8>, // FIXME: I doubt this is right
    /// Mode A code (Squawk), encoded as 4 octal digits
    #[serde(skip_serializing_if = "Option::is_none", rename = "squawk")]
    pub transponder_squawk_code: Option<Squawk>,
    /// wiedehopf's aircraft.json aircraft type pulled from database
    #[serde(skip_serializing_if = "Option::is_none", rename = "t")]
    pub aircraft_type_from_database: Option<String>,
    /// wiedehopf's aircraft.json aircraft type pulled from database, long name
    #[serde(skip_serializing_if = "Option::is_none", rename = "desc")]
    pub aircraft_type_from_database_long_name: Option<String>,
    /// list of fields derived from TIS-B data
    pub tisb: Vec<String>, // TODO: this should def be an enum
    /// true track over ground in degrees (0-359)
    #[serde(skip_serializing_if = "Option::is_none", rename = "track")]
    pub true_track_over_ground: Option<Heading>,
    /// Heading, degrees clockwise from true north (usually only transmitted on ground, in the air usually derived from the magnetic heading using magnetic model WMM2020)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_heading: Option<Heading>,
    /// type of underlying messages / best source of current data for this position / aircraft
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// ADS-B Version Number 0, 1, 2 (3-7 are reserved) (2.2.3.2.7.5)
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
