use crate::MessageResult;
use serde::{Deserialize, Serialize};
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
        serde_json::from_str(self)
    }
}

/// Supporting `.to_acars()` for the type `str`.
///
/// This does not consume the `str`.
impl NewJSONMessage for str {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        serde_json::from_str(self)
    }
}

impl JSONMessage {
    /// Converts `JSONMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        serde_json::to_string(self)
    }

    /// Converts `JSONMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error),
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
pub struct JSONMessage {
    #[serde(skip_serializing_if = "Option::is_none", rename = "alert")]
    pub flight_status_bit_alert: Option<i32>,
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
    pub calculated_best_flight_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "geom_rate")]
    pub geometric_altitude_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gs")]
    pub ground_speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gva")]
    pub geometric_verticle_accuracy: Option<i32>,
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
    pub navigation_accuracy_position: Option<i32>, // TODO: should this be an enum?
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_v")]
    pub navigation_accuracy_velocity: Option<i32>, // TODO: should this be an enum?
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_mcp")]
    pub autopilot_selected_altitude: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_heading")]
    pub autopilot_selected_heading: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_modes")]
    pub autopilot_modes: Option<Vec<NavigationModes>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_qnh")]
    pub selected_altimeter: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic")]
    pub naviation_integrity_category: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic_baro")]
    pub barometeric_altitude_integrity_category: Option<i32>,
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
    pub source_integrity_level: Option<i32>, // TODO: should this be an enum?
    pub sil_type: SourceIntegrityLevel,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spi")]
    pub flight_status_special_position_id_bit: Option<i32>,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Altitude {
    I32(i32),
    ground,
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
            Altitude::ground => write!(f, "Ground"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
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

impl fmt::Display for Emergency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Emergency::none => write!(f, "None"),
            Emergency::general => write!(f, "General"),
            Emergency::lifeguard => write!(f, "Lifeguard"),
            Emergency::minfuel => write!(f, "Minimum Fuel"),
            Emergency::nordo => write!(f, "NORDO"),
            Emergency::unlawful => write!(f, "Unlawful"),
            Emergency::downed => write!(f, "Downed"),
            Emergency::reserved => write!(f, "Reserved"),
        }
    }
}

// emitter category https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
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

// create human readable names for each emitter type

impl fmt::Display for EmitterCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmitterCategory::A0 => write!(f, "A0: No ADS-B emitter category information. Do not use this emitter category. If no emitter category fits your installation, seek guidance from the FAA as appropriate."),
            EmitterCategory::A1 => write!(f, "A1: Light (< 15500 lbs) – Any airplane with a maximum takeoff weight less than 15,500 pounds. This includes very light aircraft (light sport aircraft) that do not meet the requirements of 14 CFR § 103.1."),
            EmitterCategory::A2 => write!(f, "A2: Small (15500 to 75000 lbs) – Any airplane with a maximum takeoff weight greater than or equal to15,500 pounds but less than 75,000 pounds."),
            EmitterCategory::A3 => write!(f, "A3: Large (75000 to 300000 lbs) – Any airplane with a maximum takeoff weight greater than or equal to 75,000 pounds but less than 300,000 pounds that does not qualify for the high vortex category."),
            EmitterCategory::A4 => write!(f, "A4: High vortex large (aircraft such as B-757) – Any airplane with a maximum takeoff weight greater than or equal to 75,000 pounds but less than 300,000 pounds that has been determined to generate a high wake vortex. Currently, the Boeing 757 is the only example."),
            EmitterCategory::A5 => write!(f, "A5: Heavy (> 300000 lbs) – Any airplane with a maximum takeoff weight equal to or above 300,000 pounds."),
            EmitterCategory::A6 => write!(f, "A6: High performance (> 5g acceleration and 400 kts) – Any airplane, regardless of weight, which can maneuver in excess of 5 G’s and maintain true airspeed above 400 knots."),
            EmitterCategory::A7 => write!(f, "A7: Rotorcraft – Any rotorcraft regardless of weight."),
            EmitterCategory::B0 => write!(f, "B0: No ADS-B emitter category information"),
            EmitterCategory::B1 => write!(f, "B1: Glider / sailplane – Any glider or sailplane regardless of weight."),
            EmitterCategory::B2 => write!(f, "B2: Lighter-than-air – Any lighter than air (airship or balloon) regardless of weight."),
            EmitterCategory::B3 => write!(f, "B3: Parachutist / skydiver"),
            EmitterCategory::B4 => write!(f, "B4: Ultralight / hang-glider / paraglider – A vehicle that meets the requirements of 14 CFR § 103.1. Light sport aircraft should not use the ultralight emitter category unless they meet 14 CFR § 103.1."),
            EmitterCategory::B5 => write!(f, "B5: Reserved"),
            EmitterCategory::B6 => write!(f, "B6: Unmanned aerial vehicle – Any unmanned aerial vehicle or unmanned aircraft system regardless of weight."),
            EmitterCategory::B7 => write!(f, "Space / trans-atmospheric vehicle"),
            EmitterCategory::C0 => write!(f, "C0: No ADS-B emitter category information"),
            EmitterCategory::C1 => write!(f, "C1: Surface vehicle – emergency vehicle"),
            EmitterCategory::C2 => write!(f, "C2: Surface vehicle – service vehicle"),
            EmitterCategory::C3 => write!(f, "C3: Point obstacle (includes tethered balloons)"),
            EmitterCategory::C4 => write!(f, "C4: Cluster obstacle"),
            EmitterCategory::C5 => write!(f, "C5: Line obstacle"),
            EmitterCategory::C6 => write!(f, "C6: Reserved"),
            EmitterCategory::C7 => write!(f, "C7: Reserved"),
        }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum NavigationModes {
    // 'autopilot', 'vnav', 'althold', 'approach', 'lnav', 'tcas'
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

impl fmt::Display for NavigationModes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NavigationModes::autopilot => write!(f, "Autopilot"),
            NavigationModes::vnav => write!(f, "Vertical Navigation"),
            NavigationModes::althold => write!(f, "Altitude Hold"),
            NavigationModes::approach => write!(f, "Approach"),
            NavigationModes::lnav => write!(f, "Lateral Navigation"),
            NavigationModes::tcas => write!(f, "Traffic Collision Avoidance System"),
            NavigationModes::none => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
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

impl fmt::Display for SourceIntegrityLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SourceIntegrityLevel::unknown => write!(f, "Unknown"),
            SourceIntegrityLevel::persample => write!(f, "Per Sample"),
            SourceIntegrityLevel::perhour => write!(f, "Per Hour"),
        }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum ADSBVersion {
    Version0 = 0,
    Version1 = 1,
    Version2 = 2,
    Version3 = 3,
    Version4 = 4,
    Version5 = 5,
    Version6 = 6,
    Version7 = 7,
}

impl Default for ADSBVersion {
    fn default() -> Self {
        Self::Version0
    }
}

impl fmt::Display for ADSBVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ADSBVersion::Version0 => write!(f, "ADSB Version 0"),
            ADSBVersion::Version1 => write!(f, "ADSB Version 1"),
            ADSBVersion::Version2 => write!(f, "ADSB Version 2"),
            ADSBVersion::Version3 => write!(f, "ADSB Version 3 (Reserved)"),
            ADSBVersion::Version4 => write!(f, "ADSB Version 4 (Reserved)"),
            ADSBVersion::Version5 => write!(f, "ADSB Version 5 (Reserved)"),
            ADSBVersion::Version6 => write!(f, "ADSB Version 6 (Reserved)"),
            ADSBVersion::Version7 => write!(f, "ADSB Version 7 (Reserved)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{read_dir, File};
    use std::io::BufRead;

    #[test]
    fn decode_json_message() {
        // open all json_*.json files in test data. convert to JSONMessage and then back to string
        let test_data = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    let file = File::open(path).unwrap();
                    let reader = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader.lines().for_each(|l| {
                        line = l.unwrap();

                        // if the line starts with anything but a {, skip it
                        if line.starts_with("{") {
                            // encode the line as JSONMessage
                            let json_message = line.to_json();
                            assert!(
                                json_message.is_ok(),
                                "Failed to decode JSONMessage {:?}",
                                json_message
                            );
                        }
                    });
                }
            }
        }
    }
}
