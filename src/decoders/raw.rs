// With MASSIVE thanks to https://github.com/rsadsb/adsb_deku

use crate::MessageResult;
use deku::bitvec::{BitSlice, Msb0};
use deku::prelude::*;
use hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::raw_helpers::{df::DF, helper_functions::modes_checksum, me::ME};

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `ADSBMessage`.
pub trait NewAdsbRawMessage {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage>;
}

/// Implementing `.to_adsb_raw()` for the type `String`.
///
/// This does not consume the `String`.
/// The expected input is a hexadecimal string.
impl NewAdsbRawMessage for String {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        info!("here!");
        let bytes = hex::decode(self)?;
        match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `str`.
///
/// This does not consume the `str`.
/// The expected input is a hexadecimal string.
impl NewAdsbRawMessage for str {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        let bytes = hex::decode(self)?;
        match AdsbRawMessage::from_bytes((&bytes, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `Vec<u8>`.
/// The expected input is a a Vec<u8> of *bytes*.
impl NewAdsbRawMessage for &Vec<u8> {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        match AdsbRawMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_adsb_raw()` for the type `Vec<u8>`.
/// This does not consume the `[u8]`.
/// The expected input is a a [u8] of *bytes*.
impl NewAdsbRawMessage for &[u8] {
    fn to_adsb_raw(&self) -> MessageResult<AdsbRawMessage> {
        match AdsbRawMessage::from_bytes((self, 0)) {
            Ok((_, v)) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Downlink ADS-B Packet
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
pub struct AdsbRawMessage {
    /// Starting with 5 bit identifier, decode packet
    pub df: DF,
    /// Calculated from all bits, used as ICAO for Response packets
    #[deku(reader = "Self::read_crc(df, deku::input_bits)")]
    pub crc: u32,
}

impl fmt::Display for AdsbRawMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let AdsbRawMessage { df, crc: _ } = self;

        if let DF::ADSB(adsb) = df {
            match &adsb.me {
                ME::AircraftIdentification(identification) => {
                    write!(f, "Identification {} {}", adsb.icao, identification)
                }
                ME::AirborneVelocity(vel) => write!(f, "Velocity: {} {}", adsb.icao, vel),
                ME::AirbornePositionGNSSAltitude(altitude)
                | ME::AirbornePositionBaroAltitude(altitude) => {
                    write!(f, "Altitude {} {}", adsb.icao, altitude)
                }
                ME::SurfacePosition(surface_position) => {
                    write!(f, "Surface position {} {}", adsb.icao, surface_position)
                }
                ME::AircraftStatus(status) => write!(f, "Status {} {}", adsb.icao, status),
                ME::TargetStateAndStatusInformation(target_info) => {
                    write!(f, "Target info {} {}", adsb.icao, target_info)
                }
                ME::AircraftOperationStatus(opstatus) => {
                    write!(f, "Operation status {} {}", adsb.icao, opstatus)
                }
                ME::NoPosition(position) => write!(f, "No position {} {:?}", adsb.icao, position),
                ME::Reserved0(reserved) => write!(f, "Reserved {} {:?}", adsb.icao, reserved),
                ME::Reserved1(reserved) => write!(f, "Reserved {} {:?}", adsb.icao, reserved),
                ME::SurfaceSystemStatus(status) => {
                    write!(f, "Surface system status {} {:?}", adsb.icao, status)
                }
                ME::AircraftOperationalCoordination(coordination) => {
                    write!(
                        f,
                        "Aircraft operational coordination {} {:?}",
                        adsb.icao, coordination
                    )
                }
            }
        }
        // log out AllCallReply and others
        else if let DF::AllCallReply {
            capability,
            icao,
            p_icao,
        } = df
        {
            write!(f, "AllCallReply {} {} {}", capability, icao, p_icao)
        } else if let DF::ShortAirAirSurveillance {
            vs,
            cc,
            unused,
            sl,
            unused1,
            ri,
            unused2,
            altitude,
            parity,
        } = df
        {
            write!(
                f,
                "ShortAirAirSurveillance {} {} {} {} {} {} {} {} {}",
                vs, cc, unused, sl, unused1, ri, unused2, altitude, parity
            )
        } else if let DF::SurveillanceAltitudeReply { fs, dr, um, ac, ap } = df {
            write!(
                f,
                "SurveillanceAltitudeReply {} {} {} {} {}",
                fs, dr, um, ac, ap
            )
        } else if let DF::SurveillanceIdentityReply { fs, dr, um, id, ap } = df {
            write!(
                f,
                "SurveillanceIdentityReply {} {} {} {} {}",
                fs, dr, um, id, ap
            )
        } else if let DF::LongAirAir {
            vs,
            spare1,
            sl,
            spare2,
            ri,
            spare3,
            altitude,
            mv,
            parity,
        } = df
        {
            write!(
                f,
                "LongAirAir {} {} {} {} {} {} {} {:?} {}",
                vs, spare1, sl, spare2, ri, spare3, altitude, mv, parity
            )
        } else if let DF::TisB { cf, pi } = df {
            write!(f, "TisB {} {}", cf, pi)
        } else if let DF::ExtendedQuitterMilitaryApplication { af } = df {
            write!(f, "ExtendedQuitterMilitaryApplication {}", af)
        } else if let DF::CommBAltitudeReply {
            flight_status,
            dr,
            um,
            alt,
            bds,
            parity,
        } = df
        {
            write!(
                f,
                "CommBAltitudeReply {} {} {} {} {} {}",
                flight_status, dr, um, alt, bds, parity
            )
        } else if let DF::CommBIdentityReply {
            fs,
            dr,
            um,
            id,
            bds,
            parity,
        } = df
        {
            write!(
                f,
                "CommBIdentityReply {} {} {} {} {} {}",
                fs, dr, um, id, bds, parity
            )
        } else {
            write!(f, "{:?}", df)
        }
    }
}

impl AdsbRawMessage {
    /// Read rest as CRC bits
    fn read_crc<'b>(
        df: &DF,
        rest: &'b BitSlice<u8, Msb0>,
    ) -> Result<(&'b BitSlice<u8, Msb0>, u32), DekuError> {
        const MODES_LONG_MSG_BYTES: usize = 14;
        const MODES_SHORT_MSG_BYTES: usize = 7;

        let bit_len = if let Ok(id) = df.deku_id() {
            if id & 0x10 != 0 {
                MODES_LONG_MSG_BYTES * 8
            } else {
                MODES_SHORT_MSG_BYTES * 8
            }
        } else {
            // In this case, it's the DF::CommD, which has multiple ids
            MODES_LONG_MSG_BYTES * 8
        };

        let (_, remaining_bytes, _) = rest.domain().region().unwrap();
        let crc = modes_checksum(remaining_bytes, bit_len)?;
        Ok((rest, crc))
    }

    /// Converts `ADSBsMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Converts `ADSBJsonMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `ADSBJsonMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `ADSBJsonMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    pub fn get_time(&self) -> Option<f64> {
        Some(0.0)
    }
}
