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
    pub mlat: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nac_p: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nac_v: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_altitude_mcp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_heading: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_modes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_qnh: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_baro: Option<i32>,
    pub r: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_dir: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_dst: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc: Option<i32>,
    pub rssi: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<i32>,
    pub seen: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_pos: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sil: Option<i32>,
    pub sil_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squawk: Option<String>,
    pub t: String,
    pub tisb: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_heading: Option<f32>,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
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
