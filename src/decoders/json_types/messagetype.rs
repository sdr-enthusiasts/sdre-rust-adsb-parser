// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum MessageType {
    /// messages from a Mode S or ADS-B transponder, using a 24-bit ICAO address
    /// Original json "adsb_icao"
    ADSBICAO,
    /// messages from an ADS-B equipped "non-transponder" emitter e.g. a ground vehicle, using a 24-bit ICAO address
    /// Original json "adsb_icao_nt"
    ADSBICAONONTRANSPONDER,
    /// rebroadcast of ADS-B messages originally sent via another data link e.g. UAT, using a 24-bit ICAO address
    /// Original json "adsr_icao"
    ADSBICAOREBROADCAST,
    /// traffic information about a non-ADS-B target identified by a 24-bit ICAO address, e.g. a Mode S target tracked by secondary radar
    /// Original json "tisb_icao"
    ADSBICAOSECONDARYSURVEILLANCE,
    /// (received by monitoring satellite downlinks)
    /// Original json "adsc"
    ADSC,
    /// MLAT, position calculated arrival time differences using multiple receivers, outliers and varying accuracy is expected.
    /// Original json "mlat"
    MLAT,
    /// miscellaneous data received via Basestation / SBS format, quality / source is unknown.
    /// Original json "other"
    OTHER,
    /// ModeS data from the planes transponder (no position transmitted)
    /// Original json "mode_s"
    MODES,
    /// messages from an ADS-B transponder using a non-ICAO address, e.g. anonymized address
    /// Original json "adsb_other"
    ADSBOTHER,
    /// rebroadcast of ADS-B messages originally sent via another data link e.g. UAT, using a non-ICAO address
    /// Original json "adsbr_other"
    ADSBOTHERREBROADCAST,
    /// traffic information about a non-ADS-B target using a non-ICAO address
    /// Original json "tisb_other"
    ADSBOTHERSECONDARYSURVEILLANCE,
    /// traffic information about a non-ADS-B target using a track/file identifier, typically from primary or Mode A/C radar
    /// Original json "tisb_trackfile"
    ADSBTRACKFILE,
    #[default]
    /// Unknown
    UNKNOWN,
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageType::ADSBICAO => serializer.serialize_str("adsb_icao"),
            MessageType::ADSBICAONONTRANSPONDER => serializer.serialize_str("adsb_icao_nt"),
            MessageType::ADSBICAOREBROADCAST => serializer.serialize_str("adsr_icao"),
            MessageType::ADSBICAOSECONDARYSURVEILLANCE => serializer.serialize_str("tisb_icao"),
            MessageType::ADSC => serializer.serialize_str("adsc"),
            MessageType::MLAT => serializer.serialize_str("mlat"),
            MessageType::OTHER => serializer.serialize_str("other"),
            MessageType::MODES => serializer.serialize_str("mode_s"),
            MessageType::ADSBOTHER => serializer.serialize_str("adsb_other"),
            MessageType::ADSBOTHERREBROADCAST => serializer.serialize_str("adsr_other"),
            MessageType::ADSBOTHERSECONDARYSURVEILLANCE => serializer.serialize_str("tisb_other"),
            MessageType::ADSBTRACKFILE => serializer.serialize_str("tisb_trackfile"),
            MessageType::UNKNOWN => serializer.serialize_str("unknown"),
        }
    }
}

impl TryFrom<String> for MessageType {
    type Error = String;

    fn try_from(message_type: String) -> Result<Self, Self::Error> {
        match message_type.as_str() {
            "adsb_icao" => Ok(MessageType::ADSBICAO),
            "adsb_icao_nt" => Ok(MessageType::ADSBICAONONTRANSPONDER),
            "adsr_icao" => Ok(MessageType::ADSBICAOREBROADCAST),
            "tisb_icao" => Ok(MessageType::ADSBICAOSECONDARYSURVEILLANCE),
            "adsc" => Ok(MessageType::ADSC),
            "mlat" => Ok(MessageType::MLAT),
            "other" => Ok(MessageType::OTHER),
            "mode_s" => Ok(MessageType::MODES),
            "adsb_other" => Ok(MessageType::ADSBOTHER),
            "adsr_other" => Ok(MessageType::ADSBOTHERREBROADCAST),
            "tisb_other" => Ok(MessageType::ADSBOTHERSECONDARYSURVEILLANCE),
            "tisb_trackfile" => Ok(MessageType::ADSBTRACKFILE),
            "unknown" => Ok(MessageType::UNKNOWN),
            _ => Err(format!("Invalid MessageType field: {message_type}")),
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::ADSBICAO => write!(f, "ADSB, ICAO Address"),
            MessageType::ADSBICAONONTRANSPONDER => {
                write!(f, "ADSB, ICAO Address, Non-Transponder")
            }
            MessageType::ADSBICAOREBROADCAST => {
                write!(f, "ADSB, ICAO Address, Rebroadcast (eg UAT)")
            }
            MessageType::ADSBICAOSECONDARYSURVEILLANCE => {
                write!(f, "ADSB, ICAO Address, Secondary Surveillance (\"TISB\")")
            }
            MessageType::ADSC => write!(f, "ADS-C"),
            MessageType::MLAT => write!(f, "MLAT"),
            MessageType::OTHER => write!(f, "Other"),
            MessageType::MODES => write!(f, "Mode S"),
            MessageType::ADSBOTHER => write!(f, "ADSB, Other Address"),
            MessageType::ADSBOTHERREBROADCAST => {
                write!(f, "ADSB, Other Address, Rebroadcast (eg UAT)")
            }
            MessageType::ADSBOTHERSECONDARYSURVEILLANCE => {
                write!(f, "ADSB, Other Address, Secondary Surveillance (\"TISB\")")
            }
            MessageType::ADSBTRACKFILE => write!(f, "ADSB, Track/File Identifier"),
            MessageType::UNKNOWN => write!(f, "Unknown"),
        }
    }
}
