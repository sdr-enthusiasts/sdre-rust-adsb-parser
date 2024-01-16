use radix_fmt::radix;

use crate::decoders::{
    helpers::cpr_calculators::{
        get_position_from_even_odd_cpr_positions_airborne,
        get_position_from_even_odd_cpr_positions_surface,
        get_position_from_locally_unabiguous_airborne,
        get_position_from_locally_unabiguous_surface, haversine_distance_position, is_lat_lon_sane,
    },
    json::get_timestamp,
    json_types::timestamp::TimeStamp,
    raw_types::{cprheaders::CPRFormat, statusforgroundtrack::StatusForGroundTrack},
};

use super::{
    helpers::cpr_calculators::Position,
    json::JSONMessage,
    raw_types::{
        airbornevelocity::AirborneVelocity, aircraftstatus::AircraftStatus,
        identification::Identification, operationstatus::OperationStatus,
        surfaceposition::SurfacePosition,
    },
};

#[derive(Debug, PartialEq)]
enum PositionType {
    Airborne,
    Surface,
}

pub fn update_airborne_velocity(json: &mut JSONMessage, velocity: &AirborneVelocity) {
    if let Some((heading, ground_speed, vert_speed)) = velocity.calculate() {
        json.true_track_over_ground = Some(heading.into());
        json.ground_speed = Some(ground_speed.into());
        json.barometric_altitude_rate = Some(vert_speed.into());
        // TODO: verify this should be baro rate
    }
}

pub fn update_aircraft_identification(json: &mut JSONMessage, id: &Identification) {
    // TODO: Type coding?
    json.calculated_best_flight_id = Some(id.cn.clone().into());
    // TODO: Verify this field
}

pub fn update_operational_status(json: &mut JSONMessage, operation_status: &OperationStatus) {
    if let OperationStatus::Surface(_) = operation_status {
        json.barometric_altitude = Some("ground".into());
    }
}

pub fn update_aircraft_status(json: &mut JSONMessage, operation_status: &AircraftStatus) {
    // TODO: the rest of the fields
    json.transponder_squawk_code = Some(format!("{:04}", radix(operation_status.squawk, 16)));
}

fn update_position(
    json: &mut JSONMessage,
    even_frame: &Option<Position>,
    odd_frame: &Option<Position>,
    reference_position: &Position,
    cpr_flag: CPRFormat,
    current_time: f64,
    position_type: PositionType,
) {
    // if we have both even and odd, calculate the position
    if let (Some(even_frame), Some(odd_frame)) = (&even_frame, &odd_frame) {
        let calculated_position = if position_type == PositionType::Airborne {
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
                return;
            } else {
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
    }

    let aircraft_frame = if cpr_flag == CPRFormat::Even {
        even_frame.as_ref().unwrap()
    } else {
        odd_frame.as_ref().unwrap()
    };

    // we ended up here because even/odd failed or we didn't have both even and odd
    // if we have a reference position from the user, try to use that to calculate the position

    let position = if position_type == PositionType::Airborne {
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

                // Success! We have a position. Time to bail out.
                return;
            }
        } else {
            warn!(
                "{}: Reference position is too far away from calculated position. Not updating.",
                json.transponder_hex
            );
        }
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

    // we ended up here because everything else failed. The last try is to use the last known position

    if let (Some(lat), Some(lon)) = (&json.latitude, &json.longitude) {
        let reference_position = Position {
            latitude: lat.latitude,
            longitude: lon.longitude,
        };

        let position = if position_type == PositionType::Airborne {
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

        debug!(
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

            if let Some(last_cpr_even_update_time) = if position_type == PositionType::Airborne {
                &json.last_cpr_even_update_time_airborne
            } else {
                &json.last_cpr_even_update_time_surface
            } {
                oldest_timestamp = last_cpr_even_update_time.get_time();
            };

            if let Some(last_cpr_odd_update_time) = if position_type == PositionType::Airborne {
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

            let distance_traveled = if speed != 0.0 {
                speed * time_delta
            } else {
                0.0
            };

            // if the distance travelled is within 10% of the distance between the reference position and the calculated position, we'll update the position

            if speed != 0.0 && distance_traveled != 0.0 {
                if distance_traveled <= distance * 1.1 && distance_traveled >= distance * 0.9 {
                    info!(
                    "{} Distance traveled {} is within 10% of distance between reference position and calculated position {}",
                    json.transponder_hex, distance_traveled, distance
                );
                } else {
                    info!(
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
                return;
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

    // we ended up here because everything else failed.
    warn!("{}: Unable to calculate position.", json.transponder_hex);
}

pub fn update_aircraft_position_surface(
    json: &mut JSONMessage,
    surface_position: &SurfacePosition,
    reference_position: &Position,
) {
    json.barometric_altitude = Some("ground".into());

    // TODO: I can't figure out what tar1090 is doing for what values it's using for ground speed and track, and if it factors in the validity of the surface position. I'm going to assume it does for now.
    // Also there seems to be some fucked up thing where I may or may not be factoring in setting speed to 0 properly. Or tar1090 isn't. Well it def isn't at some point but who knows

    match surface_position.s {
        StatusForGroundTrack::Valid => {
            if let Some(groundspeed) = surface_position.get_ground_speed() {
                match groundspeed.calculate() {
                    Some(speed) => json.ground_speed = Some(speed.into()),
                    None => json.ground_speed = None,
                }
            }

            json.true_track_over_ground = surface_position.get_heading().map(|v| v.into());
        }
        StatusForGroundTrack::Invalid => {
            json.ground_speed = Some(0.0.into());
        }
    }

    let current_time = match get_timestamp() {
        TimeStamp::TimeStampAsF64(current_time) => current_time,
        TimeStamp::None => 0.0,
    };

    match surface_position.f {
        CPRFormat::Even => {
            json.cpr_even_surface = Some(*surface_position);
            json.last_cpr_even_update_time_surface = Some(get_timestamp());

            // if json.cpr_odd is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_odd_update_time) = &json.last_cpr_odd_update_time_airborne {
                // get the f64 value of the timestamp
                if last_cpr_odd_update_time.add_time(10.0) < current_time {
                    json.cpr_odd_surface = None;
                    debug!("{}: Received Even CPR packet, but odd is too old ({} seconds past 10 second valid window) Not updating.", json.transponder_hex, current_time - last_cpr_odd_update_time.add_time(10.0));
                }
            }
        }
        CPRFormat::Odd => {
            json.cpr_odd_surface = Some(*surface_position);
            json.last_cpr_odd_update_time_surface = Some(get_timestamp());

            // if json.cpr_even is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_even_update_time) = &json.last_cpr_even_update_time_airborne {
                if last_cpr_even_update_time.add_time(10.0) < current_time {
                    json.cpr_even_surface = None;
                    debug!("{}: Received Odd CPR packet, but even is too old ({} seconds past 10 second valid window). Not updating.", json.transponder_hex, current_time - last_cpr_even_update_time.add_time(10.0));
                }
            }
        }
    }

    let even_frame = if json.cpr_even_surface.is_some() {
        let frame = json.cpr_even_surface.as_ref().unwrap();
        Some(Position {
            latitude: frame.lat_cpr as f64,
            longitude: frame.lon_cpr as f64,
        })
    } else {
        None
    };

    let odd_frame = if json.cpr_odd_surface.is_some() {
        let frame = json.cpr_odd_surface.as_ref().unwrap();
        Some(Position {
            latitude: frame.lat_cpr as f64,
            longitude: frame.lon_cpr as f64,
        })
    } else {
        None
    };

    update_position(
        json,
        &even_frame,
        &odd_frame,
        reference_position,
        surface_position.f,
        current_time,
        PositionType::Surface,
    );
}

pub fn update_aircraft_position_airborne(
    json: &mut JSONMessage,
    altitude: &super::raw_types::altitude::Altitude,
    baro_altitude: bool,
    reference_position: &Position,
) {
    if let Some(alt) = &altitude.alt {
        // check the ME type to see if we have baro or GNSS altitude
        // TODO: can we do this better? We've already checked the type above and
        // Ended up here. The lat/lon positioning is the same for both, so we
        // need to use the same code for both.

        if baro_altitude {
            json.barometric_altitude = Some((*alt).into());
        } else {
            json.geometric_altitude = Some((*alt).into());
        }
    }

    let current_time = match get_timestamp() {
        TimeStamp::TimeStampAsF64(current_time) => current_time,
        TimeStamp::None => 0.0,
    };

    match altitude.odd_flag {
        CPRFormat::Even => {
            json.cpr_even_airborne = Some(*altitude);
            json.last_cpr_even_update_time_airborne = Some(get_timestamp());

            // if json.cpr_odd is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_odd_update_time) = &json.last_cpr_odd_update_time_airborne {
                // get the f64 value of the timestamp
                if last_cpr_odd_update_time.add_time(10.0) < current_time {
                    json.cpr_odd_airborne = None;
                    debug!("{}: Received Even CPR packet, but odd is too old ({} seconds past 10 second valid window) Not updating.", json.transponder_hex, current_time - last_cpr_odd_update_time.add_time(10.0));
                }
            }
        }
        CPRFormat::Odd => {
            json.cpr_odd_airborne = Some(*altitude);
            json.last_cpr_odd_update_time_airborne = Some(get_timestamp());

            // if json.cpr_even is older than 10 seconds we don't have a valid position

            if let Some(last_cpr_even_update_time) = &json.last_cpr_even_update_time_airborne {
                if last_cpr_even_update_time.add_time(10.0) < current_time {
                    json.cpr_even_airborne = None;
                    debug!("{}: Received Odd CPR packet, but even is too old ({} seconds past 10 second valid window). Not updating.", json.transponder_hex, current_time - last_cpr_even_update_time.add_time(10.0));
                }
            }
        }
    }

    let even_frame = if json.cpr_even_airborne.is_some() {
        let frame = json.cpr_even_airborne.as_ref().unwrap();
        Some(Position {
            latitude: frame.lat_cpr as f64,
            longitude: frame.lon_cpr as f64,
        })
    } else {
        None
    };

    let odd_frame = if json.cpr_odd_airborne.is_some() {
        let frame = json.cpr_odd_airborne.as_ref().unwrap();
        Some(Position {
            latitude: frame.lat_cpr as f64,
            longitude: frame.lon_cpr as f64,
        })
    } else {
        None
    };

    update_position(
        json,
        &even_frame,
        &odd_frame,
        reference_position,
        altitude.odd_flag,
        current_time,
        PositionType::Airborne,
    );
}
