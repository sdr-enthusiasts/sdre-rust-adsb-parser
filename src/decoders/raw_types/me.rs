// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Error, Write};

use super::{
    airbornevelocity::AirborneVelocity,
    airbornevelocitysubtype::AirborneVelocitySubType,
    aircraftstatus::AircraftStatus,
    altitude::Altitude,
    autopilot_modes::{AltitudeHold, ApproachMode, AutopilotEngaged, LNAV, TCAS, VNAVEngaged},
    capability::Capability,
    emergencystate::EmergencyState,
    heading::SelectedHeadingStatus,
    icao::ICAO,
    identification::Identification,
    noposition::NoPosition,
    operationstatus::OperationStatus,
    operationstatusairborne::OperationStatusAirborne,
    operationstatussurface::OperationStatusSurface,
    surfaceposition::SurfacePosition,
    targetstateandstatusinformation::TargetStateAndStatusInformation,
};
/// ADS-B Message, 5 first bits are known as Type Code (TC)
///
/// reference: ICAO 9871 (A.2.3.1)
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
#[deku(id_type = "u8", bits = "5")]
pub enum ME {
    #[deku(id_pat = "9..=18")]
    AirbornePositionBaroAltitude(Altitude), // Done

    #[deku(id = "19")]
    AirborneVelocity(AirborneVelocity), // Done

    // FIXME: no position should also update the rc and nic
    #[deku(id = "0")]
    NoPosition(NoPosition), // Done

    #[deku(id_pat = "1..=4")]
    AircraftIdentification(Identification), // Done

    #[deku(id_pat = "5..=8")]
    SurfacePosition(SurfacePosition), // Done

    #[deku(id_pat = "20..=22")]
    AirbornePositionGNSSAltitude(Altitude), // Done

    #[deku(id = "23")]
    Reserved0([u8; 6]),

    #[deku(id_pat = "24")]
    SurfaceSystemStatus([u8; 6]),

    #[deku(id_pat = "25..=27")]
    Reserved1([u8; 6]),

    #[deku(id = "28")]
    AircraftStatus(AircraftStatus), // Done

    #[deku(id = "29")]
    TargetStateAndStatusInformation(TargetStateAndStatusInformation), // Done

    #[deku(id = "30")]
    AircraftOperationalCoordination([u8; 6]),

    #[deku(id = "31")]
    AircraftOperationStatus(OperationStatus), // Done
}

impl ME {
    /// `to_string` with `DF.id()` input
    // FIXME: Can/should this be refactored in to less lines?
    #[allow(clippy::too_many_lines)]
    pub(crate) fn to_string(
        &self,
        icao: ICAO,
        address_type: &str,
        capability: Capability,
        is_transponder: bool,
    ) -> Result<String, Error> {
        let transponder: &str = if is_transponder {
            " "
        } else {
            " (Non-Transponder) "
        };

        let mut f: String = String::new();
        match self {
            ME::NoPosition(_) => {
                writeln!(f, " Extended Squitter{transponder}No position information")?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
            }
            ME::AircraftIdentification(Identification { tc, ca, cn }) => {
                writeln!(
                    f,
                    " Extended Squitter{transponder}Aircraft identification and category"
                )?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
                writeln!(f, "  Ident:         {cn}")?;
                writeln!(f, "  Category:      {tc}{ca}")?;
            }
            ME::SurfacePosition(..) => {
                writeln!(f, " Extended Squitter{transponder}Surface position")?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
            }
            ME::AirbornePositionBaroAltitude(altitude) => {
                writeln!(
                    f,
                    " Extended Squitter{transponder}Airborne position (barometric altitude)"
                )?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
                write!(f, "{altitude}")?;
            }
            ME::AirborneVelocity(airborne_velocity) => match &airborne_velocity.sub_type {
                AirborneVelocitySubType::GroundSpeedDecoding(_) => {
                    writeln!(
                        f,
                        " Extended Squitter{transponder}Airborne velocity over ground, subsonic"
                    )?;
                    writeln!(f, "  Address:       {icao} {address_type}")?;
                    writeln!(f, "  Air/Ground:    {capability}")?;
                    writeln!(
                        f,
                        "  GNSS delta:    {}{} ft",
                        airborne_velocity.gnss_sign, airborne_velocity.gnss_baro_diff
                    )?;
                    if let Some((heading, ground_speed, vertical_rate)) =
                        airborne_velocity.calculate()
                    {
                        if let Some(heading) = heading.get_heading() {
                            writeln!(f, "  Heading:       {}", libm::ceil(heading))?;
                        }
                        writeln!(
                            f,
                            "  Speed:         {} kt groundspeed",
                            libm::floor(ground_speed.get_speed())
                        )?;
                        writeln!(
                            f,
                            "  Vertical rate: {} ft/min {}",
                            vertical_rate, airborne_velocity.vrate_src
                        )?;
                    } else {
                        writeln!(f, "  Invalid packet")?;
                    }
                }
                AirborneVelocitySubType::AirspeedDecoding(airspeed_decoding) => {
                    writeln!(
                        f,
                        " Extended Squitter{transponder}Airspeed and heading, subsonic",
                    )?;
                    writeln!(f, "  Address:       {icao} {address_type}")?;
                    writeln!(f, "  Air/Ground:    {capability}")?;
                    writeln!(f, "  IAS:           {} kt", airspeed_decoding.airspeed)?;
                    if airborne_velocity.vrate_value > 0 {
                        writeln!(
                            f,
                            "  Baro rate:     {}{} ft/min",
                            airborne_velocity.vrate_sign,
                            (airborne_velocity.vrate_value - 1) * 64
                        )?;
                    }
                    writeln!(f, "  NACv:          {}", airborne_velocity.nac_v)?;
                }
                AirborneVelocitySubType::Reserved0(_) | AirborneVelocitySubType::Reserved1(_) => {
                    writeln!(
                        f,
                        " Extended Squitter{transponder}Airborne Velocity status (reserved)",
                    )?;
                    writeln!(f, "  Address:       {icao} {address_type}")?;
                }
            },
            ME::AirbornePositionGNSSAltitude(altitude) => {
                writeln!(
                    f,
                    " Extended Squitter{transponder}Airborne position (GNSS altitude)",
                )?;
                writeln!(f, "  Address:      {icao} {address_type}")?;
                write!(f, "{altitude}")?;
            }
            ME::Reserved0(_) | ME::Reserved1(_) => {
                writeln!(f, " Extended Squitter{transponder}Unknown")?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
            }
            ME::SurfaceSystemStatus(_) => {
                writeln!(
                    f,
                    " Extended Squitter{transponder}Reserved for surface system status",
                )?;
                writeln!(f, "  Address:       {icao} {address_type}")?;
                writeln!(f, "  Air/Ground:    {capability}")?;
            }
            ME::AircraftStatus(AircraftStatus {
                emergency_state,
                squawk,
                ..
            }) => {
                print_aircraft_status(
                    &mut f,
                    transponder,
                    icao,
                    capability,
                    address_type,
                    *emergency_state,
                    *squawk,
                )?;
            }
            ME::TargetStateAndStatusInformation(target_info) => {
                print_target_state_and_status_information(
                    &mut f,
                    transponder,
                    icao,
                    capability,
                    address_type,
                    target_info,
                )?;
            }
            ME::AircraftOperationalCoordination(_) => {
                print_aircraft_operational_coordination_message(
                    &mut f,
                    transponder,
                    icao,
                    address_type,
                )?;
            }
            ME::AircraftOperationStatus(OperationStatus::Airborne(opstatus_airborne)) => {
                print_operation_status_airborne(
                    &mut f,
                    transponder,
                    icao,
                    capability,
                    address_type,
                    opstatus_airborne,
                )?;
            }
            ME::AircraftOperationStatus(OperationStatus::Surface(opstatus_surface)) => {
                print_operation_status_surface(
                    &mut f,
                    transponder,
                    icao,
                    capability,
                    address_type,
                    opstatus_surface,
                )?;
            }
            ME::AircraftOperationStatus(OperationStatus::Reserved(..)) => {
                print_operation_status_reserved(&mut f, transponder, icao, address_type)?;
            }
        }
        Ok(f)
    }
}

fn print_aircraft_status(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    capability: Capability,
    address_type: &str,
    emergency_state: EmergencyState,
    squawk: u32,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Emergency/priority status",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;
    writeln!(f, "  Air/Ground:    {capability}")?;
    writeln!(f, "  Squawk:        {squawk:02X?}")?;
    writeln!(f, "  Emergency/priority:    {emergency_state}")?;

    Ok(())
}

fn print_target_state_and_status_information(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    capability: Capability,
    address_type: &str,
    target_info: &TargetStateAndStatusInformation,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Target state and status (V2)",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;
    writeln!(f, "  Air/Ground:    {capability}")?;
    writeln!(f, "  Target State and Status:")?;
    writeln!(f, "    Target altitude:   MCP, {} ft", target_info.altitude)?;
    writeln!(f, "    Altimeter setting: {} millibars", target_info.qnh)?;
    if target_info.is_heading == SelectedHeadingStatus::Valid {
        writeln!(f, "    Target heading:    {}", target_info.heading)?;
    }
    if target_info.tcas == TCAS::Engaged {
        write!(f, "    ACAS:              operational ")?;
        if target_info.autopilot == AutopilotEngaged::Engaged {
            write!(f, "autopilot ")?;
        }
        if target_info.vnac == VNAVEngaged::Engaged {
            write!(f, "vnav ")?;
        }
        if target_info.alt_hold == AltitudeHold::Engaged {
            write!(f, "altitude-hold ")?;
        }
        if target_info.approach == ApproachMode::Engaged {
            write!(f, " approach")?;
        }
        if target_info.lnav == LNAV::Engaged {
            write!(f, " lnav")?;
        }
        writeln!(f)?;
    } else {
        writeln!(f, "    ACAS:              NOT operational")?;
    }
    writeln!(f, "    NACp:              {}", target_info.nacp)?;
    writeln!(f, "    NICbaro:           {}", target_info.nicbaro)?;
    writeln!(f, "    SIL:               {} (per sample)", target_info.sil)?;
    writeln!(f, "    QNH:               {} millibars", target_info.qnh)?;

    Ok(())
}

fn print_aircraft_operational_coordination_message(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    address_type: &str,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Aircraft Operational Coordination",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;

    Ok(())
}

fn print_operation_status_airborne(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    capability: Capability,
    address_type: &str,
    opstatus_airborne: &OperationStatusAirborne,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Aircraft operational status (airborne)",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;
    writeln!(f, "  Air/Ground:    {capability}")?;
    write!(f, "  Aircraft Operational Status:\n{opstatus_airborne}")?;

    Ok(())
}

fn print_operation_status_surface(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    capability: Capability,
    address_type: &str,
    opstatus_surface: &OperationStatusSurface,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Aircraft operational status (surface)",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;
    writeln!(f, "  Air/Ground:    {capability}")?;
    write!(f, "  Aircraft Operational Status:\n {opstatus_surface}")?;

    Ok(())
}

fn print_operation_status_reserved(
    f: &mut String,
    transponder: &str,
    icao: ICAO,
    address_type: &str,
) -> Result<(), Error> {
    writeln!(
        f,
        " Extended Squitter{transponder}Aircraft operational status (reserved)",
    )?;
    writeln!(f, "  Address:       {icao} {address_type}")?;

    Ok(())
}
