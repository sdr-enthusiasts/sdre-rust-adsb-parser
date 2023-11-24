use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{airspeeddecoding::AirspeedDecoding, groundspeeddecoding::GroundSpeedDecoding};

/// Airborne Velocity Message “Subtype” Code Field Encoding
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
#[deku(ctx = "st: u8", id = "st")]
pub enum AirborneVelocitySubType {
    #[deku(id = "0")]
    Reserved0(#[deku(bits = "22")] u32),

    #[deku(id_pat = "1..=2")]
    GroundSpeedDecoding(GroundSpeedDecoding),

    #[deku(id_pat = "3..=4")]
    AirspeedDecoding(AirspeedDecoding),

    #[deku(id_pat = "5..=7")]
    Reserved1(#[deku(bits = "22")] u32),
}

impl fmt::Display for AirborneVelocitySubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AirborneVelocitySubType::Reserved0(_) | AirborneVelocitySubType::Reserved1(_) => {
                write!(f, "reserved")
            }
            AirborneVelocitySubType::GroundSpeedDecoding(_ground_speed) => {
                write!(f, "ground speed decoding")
            }
            AirborneVelocitySubType::AirspeedDecoding(_airspeed) => {
                write!(f, "airspeed decoding")
            }
        }
    }
}
