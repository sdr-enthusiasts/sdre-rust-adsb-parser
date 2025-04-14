use crate::decoders::{
    helpers::cpr_calculators::{
        get_position_from_even_odd_cpr_positions_airborne,
        get_position_from_even_odd_cpr_positions_surface,
        get_position_from_locally_unabiguous_airborne,
        get_position_from_locally_unabiguous_surface, haversine_distance_position, is_lat_lon_sane,
    },
    json_types::timestamp::TimeStamp,
    raw_types::{cprheaders::CPRFormat, statusforgroundtrack::StatusForGroundTrack},
};

use super::{
    common_types::surveillancestatus::SurveillanceStatus,
    errors::conversion::ConversionError,
    helpers::{cpr_calculators::Position, time::get_time_as_timestamp},
    json::JSONMessage,
    json_types::{
        adsbversion::ADSBVersion, emergency::Emergency, emmittercategory::EmitterCategory,
        nacp::NavigationIntegrityCategory, nacv::NavigationAccuracyVelocity,
        navigationmodes::NavigationModes, sil::SourceIntegrityLevel,
        sourceintegritylevel::SourceIntegrityLevelType,
    },
    raw_types::{
        airbornevelocity::AirborneVelocity,
        airbornevelocitysubtype::AirborneVelocitySubType,
        aircraftstatus::AircraftStatus,
        autopilot_modes::{AltitudeHold, ApproachMode, AutopilotEngaged, LNAV, TCAS, VNAVEngaged},
        emergencystate::EmergencyState,
        fms::IsFMS,
        heading::SelectedHeadingStatus,
        identification::Identification,
        modevalidity::IsValidMode,
        noposition::NoPosition,
        operationstatus::{CapabilityClass, OperationStatus},
        surfaceposition::SurfacePosition,
        targetstateandstatusinformation::TargetStateAndStatusInformation,
        verticleratesource::VerticalRateSource,
    },
};

#[derive(Debug, PartialEq)]
enum PositionType {
    Airborne,
    Surface,
}

pub fn update_airborne_velocity(json: &mut JSONMessage, velocity: &AirborneVelocity) {
    if let Some((heading, ground_speed, vert_speed)) = velocity.calculate() {
        json.true_track_over_ground = Some(heading);
        match velocity.vrate_src {
            VerticalRateSource::BarometricPressureAltitude => {
                json.barometric_altitude_rate = Some(vert_speed);
            }
            VerticalRateSource::GeometricAltitude => {
                json.geometric_altitude_rate = Some(vert_speed);
            }
        }

        match velocity.sub_type {
            AirborneVelocitySubType::GroundSpeedDecoding(_ground_speed_decoding) => {
                json.ground_speed = Some(ground_speed);
            }
            AirborneVelocitySubType::AirspeedDecoding(_airspeed_decoding) => {
                json.indicated_air_speed = Some(ground_speed);
            }
            _ => (),
        }

        json.navigation_accuracy_velocity = Some(match velocity.nac_v {
            1 => NavigationAccuracyVelocity::Category1,
            2 => NavigationAccuracyVelocity::Category2,
            3 => NavigationAccuracyVelocity::Category3,
            4 => NavigationAccuracyVelocity::Category4,
            _ => NavigationAccuracyVelocity::Category0,
        });
    }
}

pub fn update_aircraft_identification(json: &mut JSONMessage, id: &Identification) {
    json.calculated_best_flight_id = Some(id.cn.clone().into());
    if let Ok(emitter_category) = EmitterCategory::new(id.tc, id.ca) {
        json.category = Some(emitter_category);
    }
}

/// Updates the JSON message with the operational status information.
/// # Errors
/// Returns an error if the operational status is invalid.
pub fn update_operational_status(
    json: &mut JSONMessage,
    operation_status: &OperationStatus,
) -> Result<(), ConversionError> {
    // If this is not an airborne message or sufrace we can't do anything with it.
    if operation_status.is_reserved() {
        return Err(ConversionError::UnknownMessageType {
            message_me: "Operation Status".into(),
            me_type: "Reserved0".into(),
        });
    }

    if operation_status.is_surface() {
        json.barometric_altitude = Some("ground".into());
    }

    match operation_status.get_adsb_version() {
        super::raw_types::adsbversion::ADSBVersion::ADSBVersion0 => {
            json.version = Some(ADSBVersion::Version0);
        }
        super::raw_types::adsbversion::ADSBVersion::ADSBVersion1 => {
            json.version = Some(ADSBVersion::Version1);
        }
        super::raw_types::adsbversion::ADSBVersion::ADSBVersion2 => {
            json.version = Some(ADSBVersion::Version2);
        }
        super::raw_types::adsbversion::ADSBVersion::ADSBVersion3 => {
            json.version = Some(ADSBVersion::Version3);
        }
        super::raw_types::adsbversion::ADSBVersion::Unknown => {
            return Err(ConversionError::UnknownADSBVersion);
        }
    }

    match operation_status.get_capability_class() {
        CapabilityClass::Airborne(_airborne) => {
            //json.capability_class = Some(airborne);
        }
        CapabilityClass::Surface(surface) => {
            json.nic_supplement_c = Some(surface.nic_supplement_c);
            json.nic_supplement_b = None;
        }
        CapabilityClass::Unknown => {
            return Err(ConversionError::UnknownCapabilityClass);
        }
    }

    match operation_status.get_operational_mode() {
        Some(mode) => {
            json.ident_active = mode.ident_switch_active;
            json.system_design_assurance = Some(mode.system_design_assurance);
            // TODO: handle TCAS RA active
        }
        None => {
            return Err(ConversionError::UnknownOperationalMode);
        }
    }

    if let Some(nic) = operation_status.get_nic_supplement_a() {
        json.nic_supplement_a = Some(nic);
        update_nic_and_radius_of_containement(json);
    }

    if let Some(gva) = operation_status.get_geometric_vertical_accuracy() {
        json.geometric_vertical_accuracy = Some(gva.into());
    }

    if let Some(nacp) = operation_status.get_navigational_accuracy_category() {
        json.navigation_accuracy_position =
            Some(NavigationIntegrityCategory::try_from(nacp).unwrap_or_default());
    }

    if let Some(sil_supplement) = operation_status.get_sil_supplement() {
        json.sil_type = Some(sil_supplement.into());
    } else {
        json.sil_type = Some(SourceIntegrityLevelType::Unknown);
    }

    if let Some(sil) = operation_status.get_source_integrity_level() {
        json.source_integrity_level =
            Some(SourceIntegrityLevel::try_from(sil).unwrap_or(SourceIntegrityLevel::Level0));
    } else {
        json.source_integrity_level = Some(SourceIntegrityLevel::Level0);
    }

    Ok(())
}

pub fn update_aircraft_status(json: &mut JSONMessage, operation_status: &AircraftStatus) {
    match operation_status.emergency_state {
        EmergencyState::None => {
            json.emergency = Some(Emergency::None);
        }
        EmergencyState::DownedAircraft => {
            json.emergency = Some(Emergency::Downed);
        }
        EmergencyState::General => {
            json.emergency = Some(Emergency::General);
        }
        EmergencyState::Lifeguard => {
            json.emergency = Some(Emergency::Lifeguard);
        }
        EmergencyState::MinimumFuel => {
            json.emergency = Some(Emergency::Minfuel);
        }
        EmergencyState::NoCommunication => {
            json.emergency = Some(Emergency::Nordo);
        }
        EmergencyState::Reserved2 => {
            json.emergency = Some(Emergency::Reserved);
        }
        EmergencyState::UnlawfulInterference => {
            json.emergency = Some(Emergency::Unlawful);
        }
    }

    json.transponder_squawk_code = Some(operation_status.get_squawk_as_octal_string().into());
}

pub fn update_from_no_position(json: &mut JSONMessage, no_position: &NoPosition) {
    json.barometric_altitude = no_position.altitude.map(std::convert::Into::into);
}

pub fn update_target_state_and_status_information(
    json: &mut JSONMessage,
    target_state_and_status_information: &TargetStateAndStatusInformation,
) {
    let altitude = target_state_and_status_information.altitude;
    json.selected_altimeter = Some(target_state_and_status_information.qnh.into());
    if target_state_and_status_information.is_fms == IsFMS::FMS {
        json.flight_management_system_selected_altitude = Some(altitude.into());
    } else {
        json.autopilot_selected_altitude = Some(altitude.into());
    }

    if target_state_and_status_information.is_heading == SelectedHeadingStatus::Valid {
        json.autopilot_selected_heading = Some(target_state_and_status_information.heading.into());
    }

    json.navigation_accuracy_position = Some(
        NavigationIntegrityCategory::try_from(target_state_and_status_information.nacp)
            .unwrap_or_default(),
    );
    json.barometeric_altitude_integrity_category =
        Some(target_state_and_status_information.nicbaro);
    json.source_integrity_level = Some(
        SourceIntegrityLevel::try_from(target_state_and_status_information.sil).unwrap_or_default(),
    );

    if target_state_and_status_information.mode_validity == IsValidMode::ValidMode {
        let mut output_modes: Vec<NavigationModes> = Vec::new();

        if target_state_and_status_information.autopilot == AutopilotEngaged::Engaged {
            output_modes.push(NavigationModes::Autopilot);
        }

        if target_state_and_status_information.vnac == VNAVEngaged::Engaged {
            output_modes.push(NavigationModes::VNAV);
        }

        if target_state_and_status_information.alt_hold == AltitudeHold::Engaged {
            output_modes.push(NavigationModes::AltHold);
        }

        if target_state_and_status_information.approach == ApproachMode::Engaged {
            output_modes.push(NavigationModes::Approach);
        }

        if target_state_and_status_information.tcas == TCAS::Engaged {
            output_modes.push(NavigationModes::TCAS);
        }

        if target_state_and_status_information.lnav == LNAV::Engaged {
            output_modes.push(NavigationModes::LNAV);
        }

        json.autopilot_modes = Some(output_modes);
    } else {
        json.autopilot_modes = None;
    }
}

fn calculate_position_from_even_odd(
    json: &mut JSONMessage,
    even_frame: Option<&Position>,
    odd_frame: Option<&Position>,
    reference_position: &Position,
    cpr_flag: CPRFormat,
    position_type: &PositionType,
) -> Result<(), ()> {
    // if we have both even and odd, calculate the position
    if let (Some(even_frame), Some(odd_frame)) = (&even_frame, &odd_frame) {
        let calculated_position = if *position_type == PositionType::Airborne {
            get_position_from_even_odd_cpr_positions_airborne(even_frame, odd_frame, cpr_flag)
        } else {
            get_position_from_even_odd_cpr_positions_surface(
                even_frame,
                odd_frame,
                cpr_flag,
                reference_position,
            )
        };

        if let Some(position) = calculated_position {
            debug!("{} Even/Odd position {:?}", json.transponder_hex, position);
            if is_lat_lon_sane(position) {
                // only update the lat/lon if they are different
                if json.latitude != Some(position.latitude.into())
                    || json.longitude != Some(position.longitude.into())
                {
                    json.latitude = Some(position.latitude.into());
                    json.longitude = Some(position.longitude.into());
                }

                // Success! We have a position. Time to bail out.
                return Ok(());
            }
            debug!("Position from even/odd was invalid.");
            match position_type {
                PositionType::Airborne => {
                    debug!("{} {:?}", json.transponder_hex, json.cpr_even_airborne);
                    debug!("{} {:?}", json.transponder_hex, json.cpr_odd_airborne);
                }
                PositionType::Surface => {
                    debug!("{} {:?}", json.transponder_hex, json.cpr_even_surface);
                    debug!("{} {:?}", json.transponder_hex, json.cpr_odd_surface);
                }
            }
            debug!("{} {:?}", json.transponder_hex, position);
        }
    }

    Err(())
}

fn calculate_position_from_user_reference_position(
    json: &mut JSONMessage,
    aircraft_frame: &Position,
    reference_position: &Position,
    cpr_flag: CPRFormat,
    position_type: &PositionType,
) -> Result<(), ()> {
    // we ended up here because even/odd failed or we didn't have both even and odd
    // if we have a reference position from the user, try to use that to calculate the position

    let position = if *position_type == PositionType::Airborne {
        get_position_from_locally_unabiguous_airborne(aircraft_frame, reference_position, cpr_flag)
    } else {
        get_position_from_locally_unabiguous_surface(aircraft_frame, reference_position, cpr_flag)
    };

    debug!("{} Reference position {:?}", json.transponder_hex, position);
    if is_lat_lon_sane(position) {
        debug!("{} {:?}", json.transponder_hex, position);
        // validate the haversine distance between the reference position and the calculated position is reasonable
        if haversine_distance_position(&position, reference_position) < 500.0 {
            if json.latitude != Some(position.latitude.into())
                || json.longitude != Some(position.longitude.into())
            {
                json.latitude = Some(position.latitude.into());
                json.longitude = Some(position.longitude.into());
            }

            // Success! We have a position. Time to bail out.
            return Ok(());
        }

        warn!(
            "{}: Reference position is too far away from calculated position. Not updating.",
            json.transponder_hex
        );
    } else {
        debug!("Position from reference antenna was invalid.");
        match position_type {
            PositionType::Airborne => {
                debug!("{} {:?}", json.transponder_hex, json.cpr_even_airborne);
                debug!("{} {:?}", json.transponder_hex, json.cpr_odd_airborne);
            }
            PositionType::Surface => {
                debug!("{} {:?}", json.transponder_hex, json.cpr_even_surface);
                debug!("{} {:?}", json.transponder_hex, json.cpr_odd_surface);
            }
        }
        debug!("{} {:?}", json.transponder_hex, position);
    }

    Err(())
}

fn calculate_position_from_last_known_position(
    json: &mut JSONMessage,
    aircraft_frame: &Position,
    cpr_flag: CPRFormat,
    position_type: &PositionType,
    current_time: f64,
) -> Result<(), ConversionError> {
    if let (Some(lat), Some(lon)) = (&json.latitude, &json.longitude) {
        if lat.latitude == 0.0 || lon.longitude == 0.0 {
            return Err(ConversionError::LatitudeOrLongitudeIsZero {
                lat: lat.latitude,
                lon: lon.longitude,
            });
        }

        let reference_position = Position {
            latitude: lat.latitude,
            longitude: lon.longitude,
        };

        let position = if *position_type == PositionType::Airborne {
            get_position_from_locally_unabiguous_airborne(
                aircraft_frame,
                &reference_position,
                cpr_flag,
            )
        } else {
            get_position_from_locally_unabiguous_surface(
                aircraft_frame,
                &reference_position,
                cpr_flag,
            )
        };

        info!(
            "{} Last known position calculated {:?}",
            json.transponder_hex, position
        );
        if is_lat_lon_sane(position) {
            let mut update = true;
            // get the haversine distance between the reference position and the calculated position
            let distance = haversine_distance_position(&position, &reference_position);

            // validate the haversine distance between the reference position and the calculated position is reasonable
            // We'll factor in the timestamp of the OLDEST of the two positions (json.last_cpr_even_update_time / json.last_cpr_odd_update_time) + aircraft speed to get a rough idea of how far the aircraft could have moved since the last position was received.

            let mut oldest_timestamp = 0.0;

            if let Some(last_cpr_even_update_time) = if *position_type == PositionType::Airborne {
                &json.last_cpr_even_update_time_airborne
            } else {
                &json.last_cpr_even_update_time_surface
            } {
                oldest_timestamp = last_cpr_even_update_time.get_time();
            }

            if let Some(last_cpr_odd_update_time) = if *position_type == PositionType::Airborne {
                &json.last_cpr_odd_update_time_airborne
            } else {
                &json.last_cpr_odd_update_time_surface
            } {
                if last_cpr_odd_update_time.get_time() < oldest_timestamp {
                    oldest_timestamp = last_cpr_odd_update_time.get_time();
                }
            }

            // get the time delta between the oldest timestamp and now
            let time_delta = current_time - oldest_timestamp;

            // get the speed of the aircraft in knots
            let speed = match &json.ground_speed {
                Some(speed) => speed.get_speed(),
                None => 0.0,
            };

            // get the distance the aircraft could have traveled in the time delta only if speed is not 0

            let distance_traveled = if speed == 0.0 {
                0.0
            } else {
                speed * time_delta
            };

            // if the distance travelled is within 10% of the distance between the reference position and the calculated position, we'll update the position

            if speed != 0.0 && distance_traveled != 0.0 {
                if distance_traveled <= distance * 1.1 && distance_traveled >= distance * 0.9 {
                    error!(
                        "{} Distance traveled {} is within 10% of distance between reference position and calculated position {}",
                        json.transponder_hex, distance_traveled, distance
                    );
                } else {
                    error!(
                        "{} Distance traveled {} is NOT within 10% of distance between reference position and calculated position {}",
                        json.transponder_hex, distance_traveled, distance
                    );

                    update = false;
                }
            }

            // only update the lat/lon if they are different
            if update
                && (json.latitude != Some(position.latitude.into())
                    || json.longitude != Some(position.longitude.into()))
            {
                json.latitude = Some(position.latitude.into());
                json.longitude = Some(position.longitude.into());

                // Success! We have a position. Time to bail out.
                return Ok(());
            }
        } else {
            debug!("Position from last known position was invalid.");
            match position_type {
                PositionType::Airborne => {
                    debug!("{} {:?}", json.transponder_hex, json.cpr_even_airborne);
                    debug!("{} {:?}", json.transponder_hex, json.cpr_odd_airborne);
                }
                PositionType::Surface => {
                    debug!("{} {:?}", json.transponder_hex, json.cpr_even_surface);
                    debug!("{} {:?}", json.transponder_hex, json.cpr_odd_surface);
                }
            }
        }
    }

    Err(ConversionError::UnableToCalculatePosition)
}

fn update_position(
    json: &mut JSONMessage,
    even_frame: Option<&Position>,
    odd_frame: Option<&Position>,
    reference_position: &Position,
    cpr_flag: CPRFormat,
    current_time: f64,
    position_type: &PositionType,
) -> Result<(), ConversionError> {
    if calculate_position_from_even_odd(
        json,
        even_frame,
        odd_frame,
        reference_position,
        cpr_flag,
        position_type,
    )
    .is_ok()
    {
        return Ok(());
    }

    let aircraft_frame = if cpr_flag == CPRFormat::Even {
        even_frame.as_ref().unwrap()
    } else {
        odd_frame.as_ref().unwrap()
    };

    if calculate_position_from_user_reference_position(
        json,
        aircraft_frame,
        reference_position,
        cpr_flag,
        position_type,
    )
    .is_ok()
    {
        return Ok(());
    }

    // we ended up here because everything else failed. The last try is to use the last known position

    calculate_position_from_last_known_position(
        json,
        aircraft_frame,
        cpr_flag,
        position_type,
        current_time,
    )
}

fn update_nic_and_radius_of_containment_nic_a_and_b(json: &mut JSONMessage) -> bool {
    if let (Some(nic_supplement_b), Some(nic_supplement_a), Some(airborne_type_code)) = (
        &json.nic_supplement_b,
        &json.nic_supplement_a,
        &json.airborne_type_code,
    ) {
        match airborne_type_code {
            0 | 18 | 22 => {
                json.radius_of_containment = None;
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Unknown);
                return true;
            }
            17 => {
                // 37.04km
                json.radius_of_containment = Some(37040.0.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category1);
                return true;
            }
            16 => {
                if *nic_supplement_a == 0 && *nic_supplement_b == 0 {
                    // 14.816 km
                    json.radius_of_containment = Some(14816.0.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category2);
                    return true;
                }

                if *nic_supplement_a == 1 && *nic_supplement_b == 1 {
                    // 7.408 km
                    json.radius_of_containment = Some(7408.0.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category3);
                    return true;
                }

                return false;
            }
            15 => {
                // 3.704 km
                json.radius_of_containment = Some(3704.0.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category4);
                return true;
            }
            14 => {
                // 1.852 km
                json.radius_of_containment = Some(1852.0.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category5);
                return true;
            }
            13 => {
                if *nic_supplement_a == 1 && *nic_supplement_b == 1 {
                    // 1111.2 m
                    json.radius_of_containment = Some(1111.2.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category6);
                    return true;
                }

                if *nic_supplement_a == 0 && *nic_supplement_b == 0 {
                    // 926 m
                    json.radius_of_containment = Some(926.0.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category6);
                    return true;
                }

                if *nic_supplement_a == 0 && *nic_supplement_b == 1 {
                    // 555.6 m
                    json.radius_of_containment = Some(555.6.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category6);
                    return true;
                }

                return false;
            }
            12 => {
                // 370.4 m
                json.radius_of_containment = Some(370.4.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category7);

                return true;
            }
            11 => {
                if *nic_supplement_a == 0 && *nic_supplement_b == 0 {
                    // 185.2 m
                    json.radius_of_containment = Some(185.2.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category8);
                    return true;
                }
                if *nic_supplement_a == 1 && *nic_supplement_b == 1 {
                    // 75 m
                    json.radius_of_containment = Some(75.0.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category9);
                    return true;
                }

                return false;
            }
            10 | 21 => {
                // 25 m
                json.radius_of_containment = Some(25.0.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category10);
                return true;
            }
            9 | 20 => {
                // 7.5 m
                json.radius_of_containment = Some(7.5.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category11);
                return true;
            }
            _ => return false,
        }
    }
    false
}

fn update_nic_and_radius_of_containment_a_and_c(json: &mut JSONMessage) -> bool {
    if let (Some(nic_supplment_a), Some(nic_supplment_c), Some(surface_type_code)) = (
        &json.nic_supplement_a,
        &json.nic_supplement_c,
        &json.surface_type_code,
    ) {
        match surface_type_code {
            0 => {
                json.radius_of_containment = None;
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Unknown);
                return true;
            }
            8 => {
                if *nic_supplment_a == 0 && *nic_supplment_c == 0 {
                    json.radius_of_containment = None;
                    json.navigation_integrity_category = Some(NavigationIntegrityCategory::Unknown);
                    return true;
                }

                if *nic_supplment_a == 0 && *nic_supplment_c == 1 {
                    // 1111.2 m
                    json.radius_of_containment = Some(1111.2.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category6);
                    return true;
                }

                if *nic_supplment_a == 1 && *nic_supplment_c == 0 {
                    // 555.6 m
                    json.radius_of_containment = Some(555.6.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category6);
                    return true;
                }

                if *nic_supplment_a == 1 && *nic_supplment_c == 1 {
                    // 370.4 m
                    json.radius_of_containment = Some(370.4.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category7);
                    return true;
                }

                return false;
            }
            7 => {
                if *nic_supplment_a == 0 && *nic_supplment_c == 0 {
                    // 185.2 m
                    json.radius_of_containment = Some(185.2.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category8);
                    return true;
                }

                if *nic_supplment_a == 1 && *nic_supplment_c == 0 {
                    // 75 m
                    json.radius_of_containment = Some(75.0.into());
                    json.navigation_integrity_category =
                        Some(NavigationIntegrityCategory::Category9);
                    return true;
                }

                return false;
            }
            6 => {
                // 25 m
                json.radius_of_containment = Some(25.0.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category10);
                return true;
            }
            5 => {
                // 7.5 m
                json.radius_of_containment = Some(7.5.into());
                json.navigation_integrity_category = Some(NavigationIntegrityCategory::Category11);
                return true;
            }
            _ => return false,
        }
    }

    false
}

fn update_nic_and_radius_of_containement(json: &mut JSONMessage) {
    // if json.nic_supplement_b and json.nic_supplement_a are both some, lets process

    if update_nic_and_radius_of_containment_nic_a_and_b(json) {
        return;
    }

    if update_nic_and_radius_of_containment_a_and_c(json) {
        return;
    }

    // We've made it to here and can't sus out the radius of containment. Set it to None.
    json.radius_of_containment = None;
    json.navigation_integrity_category = Some(NavigationIntegrityCategory::Unknown);
}

/// Updates the JSON message with the surface position information.
/// # Errors
/// Returns an error if the position is invalid.
pub fn update_aircraft_position_surface(
    json: &mut JSONMessage,
    surface_position: &SurfacePosition,
    reference_position: &Position,
) -> Result<(), ConversionError> {
    json.barometric_altitude = Some("ground".into());
    json.surface_type_code = Some(surface_position.type_code);

    match surface_position.s {
        StatusForGroundTrack::Valid => {
            if let Some(groundspeed) = surface_position.get_ground_speed() {
                match groundspeed.calculate() {
                    Some(speed) => json.ground_speed = Some(speed.into()),
                    None => json.ground_speed = None,
                }
            }

            json.true_track_over_ground =
                surface_position.get_heading().map(std::convert::Into::into);
        }
        StatusForGroundTrack::Invalid => {
            json.ground_speed = Some(0.0.into());
        }
    }

    let current_time = match get_time_as_timestamp() {
        TimeStamp::TimeStampAsF64(current_time) => current_time,
        TimeStamp::None => 0.0,
    };

    match surface_position.f {
        CPRFormat::Even => {
            json.cpr_even_surface = Some(*surface_position);
            json.last_cpr_even_update_time_surface = Some(get_time_as_timestamp());

            // if json.cpr_odd is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_odd_update_time) = &json.last_cpr_odd_update_time_surface {
                // get the f64 value of the timestamp
                if last_cpr_odd_update_time.add_time(10.0) < current_time {
                    json.cpr_odd_surface = None;
                    debug!(
                        "{}: Received Even CPR packet, but odd is too old ({} seconds past 10 second valid window) Not updating.",
                        json.transponder_hex,
                        current_time - last_cpr_odd_update_time.add_time(10.0)
                    );
                }
            }
        }
        CPRFormat::Odd => {
            json.cpr_odd_surface = Some(*surface_position);
            json.last_cpr_odd_update_time_surface = Some(get_time_as_timestamp());

            // if json.cpr_even is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_even_update_time) = &json.last_cpr_even_update_time_surface {
                if last_cpr_even_update_time.add_time(10.0) < current_time {
                    json.cpr_even_surface = None;
                    debug!(
                        "{}: Received Odd CPR packet, but even is too old ({} seconds past 10 second valid window). Not updating.",
                        json.transponder_hex,
                        current_time - last_cpr_even_update_time.add_time(10.0)
                    );
                }
            }
        }
    }

    let even_frame = json.cpr_even_surface.map(|frame| Position {
        latitude: f64::from(frame.lat_cpr),
        longitude: f64::from(frame.lon_cpr),
    });

    let odd_frame = json.cpr_odd_surface.map(|frame| Position {
        latitude: f64::from(frame.lat_cpr),
        longitude: f64::from(frame.lon_cpr),
    });

    update_position(
        json,
        even_frame.as_ref(),
        odd_frame.as_ref(),
        reference_position,
        surface_position.f,
        current_time,
        &PositionType::Surface,
    )
}

/// Updates the JSON message with the altitude information.
/// This function is used for both airborne and surface messages.
/// # Errors
/// Returns an error if the altitude is invalid.
pub fn update_aircraft_position_airborne(
    json: &mut JSONMessage,
    altitude: &super::raw_types::altitude::Altitude,
    baro_altitude: bool,
    reference_position: &Position,
) -> Result<(), ConversionError> {
    if let Some(alt) = &altitude.alt {
        if baro_altitude {
            json.barometric_altitude = Some((*alt).into());
        } else {
            json.geometric_altitude = Some((*alt).into());
        }
    }

    json.nic_supplement_b = Some(altitude.saf_or_imf);
    json.nic_supplement_c = None;
    json.airborne_type_code = Some(altitude.tc);

    update_nic_and_radius_of_containement(json);

    // TODO: I feel like the alert bit should maybe be set with the SPI condition
    // but somewhere else from another value. Maybe perhaps. I don't know. I'm not sure.
    json.flight_status = Some(altitude.ss);
    match altitude.ss {
        SurveillanceStatus::NoCondition => {
            json.flight_status_special_position_id_bit = Some(0);
        }
        SurveillanceStatus::PermanentAlert | SurveillanceStatus::TemporaryAlert => (),
        SurveillanceStatus::SPICondition => {
            json.flight_status_special_position_id_bit = Some(1);
        }
    }

    // NOTE: We are dropping the antenna flag.

    let current_time = match get_time_as_timestamp() {
        TimeStamp::TimeStampAsF64(current_time) => current_time,
        TimeStamp::None => 0.0,
    };

    match altitude.odd_flag {
        CPRFormat::Even => {
            json.cpr_even_airborne = Some(*altitude);
            json.last_cpr_even_update_time_airborne = Some(get_time_as_timestamp());

            // if json.cpr_odd is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_odd_update_time) = &json.last_cpr_odd_update_time_airborne {
                // get the f64 value of the timestamp
                if last_cpr_odd_update_time.add_time(10.0) < current_time {
                    json.cpr_odd_airborne = None;
                    debug!(
                        "{}: Received Even CPR packet, but odd is too old ({} seconds past 10 second valid window) Not updating.",
                        json.transponder_hex,
                        current_time - last_cpr_odd_update_time.add_time(10.0)
                    );
                }
            }
        }
        CPRFormat::Odd => {
            json.cpr_odd_airborne = Some(*altitude);
            json.last_cpr_odd_update_time_airborne = Some(get_time_as_timestamp());

            // if json.cpr_even is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_even_update_time) = &json.last_cpr_even_update_time_airborne {
                if last_cpr_even_update_time.add_time(10.0) < current_time {
                    json.cpr_even_airborne = None;
                    debug!(
                        "{}: Received Odd CPR packet, but even is too old ({} seconds past 10 second valid window). Not updating.",
                        json.transponder_hex,
                        current_time - last_cpr_even_update_time.add_time(10.0)
                    );
                }
            }
        }
    }

    let even_frame = json.cpr_even_airborne.map(|frame| Position {
        latitude: f64::from(frame.lat_cpr),
        longitude: f64::from(frame.lon_cpr),
    });

    let odd_frame = json.cpr_odd_airborne.map(|frame| Position {
        latitude: f64::from(frame.lat_cpr),
        longitude: f64::from(frame.lon_cpr),
    });

    update_position(
        json,
        even_frame.as_ref(),
        odd_frame.as_ref(),
        reference_position,
        altitude.odd_flag,
        current_time,
        &PositionType::Airborne,
    )
}
