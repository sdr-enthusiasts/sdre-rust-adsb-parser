// Copyright (c) 2024 Frederick Clausen II and authors of libadsb_deku

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://raw.githubusercontent.com/rsadsb/adsb_deku/master/libadsb_deku/src/cpr.rs

/*!
Compact Position Reporting for [`Position`] Reporting

reference: ICAO 9871 (D.2.4.7)
!*/

// FIXME: surface position decoding needs verification, especially in southern hemisphere

use serde::{Deserialize, Serialize};

use crate::decoders::raw_types::cprheaders::CPRFormat;

const NZ: f64 = 15.0;
const D_LAT_EVEN_AIRBORNE: f64 = 360.0 / (4.0 * NZ);
const D_LAT_ODD_AIRBORNE: f64 = 360.0 / (4.0 * NZ - 1.0);
const D_LAT_EVEN_SURFACE: f64 = 90.0 / (4.0 * NZ);
const D_LAT_ODD_SURFACE: f64 = 90.0 / (4.0 * NZ - 1.0);

/// 2^17 (Max of 17 bits)
const CPR_MAX: f64 = 131_072.0;

/// Post-processing of CPR into Latitude/Longitude
#[derive(Debug, PartialEq, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

fn cpr_nl_less_than_twenty_nine(lat: f64) -> f64 {
    if lat < 10.470_471_30 {
        return 59.0;
    }
    if lat < 14.828_174_37 {
        return 58.0;
    }
    if lat < 18.186_263_57 {
        return 57.0;
    }
    if lat < 21.029_394_93 {
        return 56.0;
    }
    if lat < 23.545_044_87 {
        return 55.0;
    }
    if lat < 25.829_247_07 {
        return 54.0;
    }
    if lat < 27.938_987_10 {
        return 53.0;
    }
    // < 29.91135686
    52.0
}

fn cpr_nl_less_than_forty_four(lat: f64) -> f64 {
    if lat < 31.772_097_08 {
        return 51.0;
    }
    if lat < 33.539_934_36 {
        return 50.0;
    }
    if lat < 35.228_995_98 {
        return 49.0;
    }
    if lat < 36.850_251_08 {
        return 48.0;
    }
    if lat < 38.412_418_92 {
        return 47.0;
    }
    if lat < 39.922_566_84 {
        return 46.0;
    }
    if lat < 41.386_518_32 {
        return 45.0;
    }
    if lat < 42.809_140_12 {
        return 44.0;
    }
    // < 44.19454951
    43.0
}

fn cpr_lat_less_than_fifty_nine(lat: f64) -> f64 {
    if lat < 45.546_267_23 {
        return 42.0;
    }
    if lat < 46.867_332_52 {
        return 41.0;
    }
    if lat < 48.160_391_28 {
        return 40.0;
    }
    if lat < 49.427_764_39 {
        return 39.0;
    }
    if lat < 50.671_501_66 {
        return 38.0;
    }
    if lat < 51.893_424_69 {
        return 37.0;
    }
    if lat < 53.095_161_53 {
        return 36.0;
    }
    if lat < 54.278_174_72 {
        return 35.0;
    }
    if lat < 55.443_784_44 {
        return 34.0;
    }
    if lat < 56.593_187_56 {
        return 33.0;
    }
    if lat < 57.727_473_54 {
        return 32.0;
    }
    if lat < 58.847_637_76 {
        return 31.0;
    }
    // < 59.95459277
    30.0
}

fn cpr_greater_than(lat: f64) -> f64 {
    if lat < 61.049_177_74 {
        return 29.0;
    }
    if lat < 62.132_166_59 {
        return 28.0;
    }
    if lat < 63.204_274_79 {
        return 27.0;
    }
    if lat < 64.266_165_23 {
        return 26.0;
    }
    if lat < 65.318_453_10 {
        return 25.0;
    }
    if lat < 66.361_710_08 {
        return 24.0;
    }
    if lat < 67.396_467_74 {
        return 23.0;
    }
    if lat < 68.423_220_22 {
        return 22.0;
    }
    if lat < 69.442_426_31 {
        return 21.0;
    }
    if lat < 70.454_510_75 {
        return 20.0;
    }
    if lat < 71.459_864_73 {
        return 19.0;
    }
    if lat < 72.458_845_45 {
        return 18.0;
    }
    if lat < 73.451_774_42 {
        return 17.0;
    }
    if lat < 74.438_934_16 {
        return 16.0;
    }
    if lat < 75.420_562_57 {
        return 15.0;
    }
    if lat < 76.396_843_91 {
        return 14.0;
    }
    if lat < 77.367_894_61 {
        return 13.0;
    }
    if lat < 78.333_740_83 {
        return 12.0;
    }
    if lat < 79.294_282_25 {
        return 11.0;
    }
    if lat < 80.249_232_13 {
        return 10.0;
    }
    if lat < 81.198_013_49 {
        return 9.0;
    }
    if lat < 82.139_569_81 {
        return 8.0;
    }
    if lat < 83.071_994_45 {
        return 7.0;
    }
    if lat < 83.991_735_63 {
        return 6.0;
    }
    if lat < 84.891_661_91 {
        return 5.0;
    }
    if lat < 85.755_416_21 {
        return 4.0;
    }
    if lat < 86.535_369_98 {
        return 3.0;
    }
    if lat < 87.000_000_00 {
        return 2.0;
    }
    1.0
}

/// The NL function uses the precomputed table from 1090-WP-9-14
/// This code is translated from <https://github.com/wiedehopf/readsb/blob/dev/cpr.c>
pub(crate) fn cpr_nl(lat: f64) -> f64 {
    let mut lat = lat;
    if lat < 0.0 {
        // Table is symmetric about the equator
        lat = -lat;
    }

    if lat < 29.911_356_86 {
        return cpr_nl_less_than_twenty_nine(lat);
    }

    if lat < 44.194_549_51 {
        return cpr_nl_less_than_forty_four(lat);
    }

    if lat < 59.954_592_77 {
        return cpr_lat_less_than_fifty_nine(lat);
    }

    cpr_greater_than(lat)
}

#[must_use]
pub fn haversine_distance_position(position: &Position, other: &Position) -> f64 {
    let lat1 = position.latitude;
    let lat2 = other.latitude;
    let long1 = position.longitude;
    let long2 = other.longitude;
    haversine_distance((lat1, long1), (lat2, long2))
}

// https://en.wikipedia.org/wiki/Haversine_formula
#[must_use]
pub fn haversine_distance(s: (f64, f64), other: (f64, f64)) -> f64 {
    // kilometers
    let lat1_rad = s.0.to_radians();
    let lat2_rad = other.0.to_radians();
    let long1_rad = s.1.to_radians();
    let long2_rad = other.1.to_radians();

    let x_lat = libm::sin((lat2_rad - lat1_rad) / 2.00);
    let x_long = libm::sin((long2_rad - long1_rad) / 2.00);

    let a = x_lat * x_lat
        + libm::cos(lat1_rad) * libm::cos(lat2_rad) * libm::pow(libm::sin(x_long), 2.0);

    let c = 2.0 * libm::atan2(libm::sqrt(a), libm::sqrt(1.0 - a));

    let r = 6371.00;
    r * c
}

#[must_use]
pub fn calc_modulo(x: f64, y: f64) -> f64 {
    x - y * libm::floor(x / y)
}

#[must_use]
pub fn get_position_from_locally_unabiguous_surface(
    aircraft_frame: &Position,
    local: &Position,
    cpr_flag: CPRFormat,
) -> Position {
    let mut i = 0.0;
    let d_lat = match cpr_flag {
        CPRFormat::Even => D_LAT_EVEN_SURFACE,
        CPRFormat::Odd => {
            i = 1.0;
            D_LAT_ODD_SURFACE
        }
    };

    let lat_cpr = aircraft_frame.latitude / CPR_MAX;
    let lon_cpr = aircraft_frame.longitude / CPR_MAX;

    let j = libm::floor(local.latitude / d_lat)
        + libm::floor(calc_modulo(local.latitude, d_lat) / d_lat - lat_cpr + 0.5);
    let lat = d_lat * (j + lat_cpr);

    let d_lon = 90.0 / libm::fmax(cpr_nl(lat) - i, 1.0);

    let m = libm::floor(local.longitude / d_lon)
        + libm::floor(calc_modulo(local.longitude, d_lon) / d_lon - lon_cpr + 0.5);

    let lon = d_lon * (m + lon_cpr);

    Position {
        latitude: lat,
        longitude: lon,
    }
}

#[must_use]
pub fn get_position_from_locally_unabiguous_airborne(
    aircraft_frame: &Position,
    local: &Position,
    cpr_flag: CPRFormat,
) -> Position {
    let mut i = 0.0;
    let d_lat = match cpr_flag {
        CPRFormat::Even => D_LAT_EVEN_AIRBORNE,
        CPRFormat::Odd => {
            i = 1.0;
            D_LAT_ODD_AIRBORNE
        }
    };

    let lat_cpr = aircraft_frame.latitude / CPR_MAX;
    let lon_cpr = aircraft_frame.longitude / CPR_MAX;

    let j = libm::floor(local.latitude / d_lat)
        + libm::floor(calc_modulo(local.latitude, d_lat) / d_lat - lat_cpr + 0.5);

    let lat = d_lat * (j + lat_cpr);

    let d_lon = 360.0 / libm::fmax(cpr_nl(lat) - i, 1.0);

    let m = libm::floor(local.longitude / d_lon)
        + libm::floor(calc_modulo(local.longitude, d_lon) / d_lon - lon_cpr + 0.5);

    let lon = d_lon * (m + lon_cpr);

    Position {
        latitude: lat,
        longitude: lon,
    }
}

/// Calculate Globally unambiguous position decoding
///
/// Using both an Odd and Even `Altitude`, calculate the latitude/longitude
///
/// reference: ICAO 9871 (D.2.4.7.7)
#[must_use]
pub fn get_position_from_even_odd_cpr_positions_airborne(
    even_frame: &Position,
    odd_frame: &Position,
    latest_frame_flag: CPRFormat,
) -> Option<Position> {
    let cpr_lat_even = even_frame.latitude / CPR_MAX;
    let cpr_lat_odd = odd_frame.latitude / CPR_MAX;
    let cpr_lon_even = even_frame.longitude / CPR_MAX;
    let cpr_lon_odd = odd_frame.longitude / CPR_MAX;

    let j = libm::floor(59.0 * cpr_lat_even - 60.0 * cpr_lat_odd + 0.5);

    let mut lat_even = D_LAT_EVEN_AIRBORNE * (calc_modulo(j, 60.0) + cpr_lat_even);
    let mut lat_odd = D_LAT_ODD_AIRBORNE * (calc_modulo(j, 59.0) + cpr_lat_odd);

    if lat_even >= 270.0 {
        lat_even -= 360.0;
    }

    if lat_odd >= 270.0 {
        lat_odd -= 360.0;
    }
    // are the frame zones the same?

    let nl_even = cpr_nl(lat_even);
    let nl_odd = cpr_nl(lat_odd);

    if (nl_even - nl_odd).abs() > f64::EPSILON {
        debug!("NL even and NL odd are not the same");
        debug!("NL even: {nl_even}");
        debug!("NL odd: {nl_odd}");
        return None;
    }

    // the final latitude position
    let lat = if latest_frame_flag == CPRFormat::Even {
        lat_even
    } else {
        lat_odd
    };

    let m = libm::floor(cpr_lon_even * (nl_even - 1.0) - cpr_lon_odd * nl_even + 0.5);

    let n_even_calc = libm::fmax(nl_even, 1.0);
    let n_odd_calc = libm::fmax(nl_odd - 1.0, 1.0);

    let d_lon_even = 360.0 / n_even_calc;
    let d_lon_odd = 360.0 / n_odd_calc;

    let lon_even = d_lon_even * (calc_modulo(m, n_even_calc) + cpr_lon_even);
    let lon_odd = d_lon_odd * (calc_modulo(m, n_odd_calc) + cpr_lon_odd);

    let mut lon = if latest_frame_flag == CPRFormat::Even {
        lon_even
    } else {
        lon_odd
    };

    if lon >= 180.0 {
        lon -= 360.0;
    }

    Some(Position {
        latitude: lat,
        longitude: lon,
    })
}

#[must_use]
pub fn get_position_from_even_odd_cpr_positions_surface(
    even_frame: &Position,
    odd_frame: &Position,
    latest_frame_flag: CPRFormat,
    reference_position: &Position,
) -> Option<Position> {
    let cpr_lat_even = even_frame.latitude / CPR_MAX;
    let cpr_lat_odd = odd_frame.latitude / CPR_MAX;
    let cpr_lon_even = even_frame.longitude / CPR_MAX;
    let cpr_lon_odd = odd_frame.longitude / CPR_MAX;

    let j = libm::floor(59.0 * cpr_lat_even - 60.0 * cpr_lat_odd + 0.5);

    let lat_even = D_LAT_EVEN_SURFACE * (calc_modulo(j, 60.0) + cpr_lat_even);
    let lat_odd = D_LAT_ODD_SURFACE * (calc_modulo(j, 59.0) + cpr_lat_odd);

    // validate the NZ values are the same

    let nl_even = cpr_nl(lat_even);
    let nl_odd = cpr_nl(lat_odd);

    if (nl_even - nl_odd).abs() > f64::EPSILON {
        debug!("NL even and NL odd are not the same");
        debug!("NL even: {nl_even}");
        debug!("NL odd: {nl_odd}");
        return None;
    }

    let lat_northern = if latest_frame_flag == CPRFormat::Even {
        lat_odd
    } else {
        lat_even // the fuck? This matches the solution in the mode-s.org work....
    };

    let lat_southern = &lat_northern - 90.0;

    let m = libm::floor(cpr_lon_even * (nl_even - 1.0) - cpr_lon_odd * nl_even + 0.5);

    let n = if latest_frame_flag == CPRFormat::Even {
        libm::fmax(nl_even, 1.0)
    } else {
        libm::fmax(nl_odd - 1.0, 1.0)
    };

    let d_lon = 90.0 / n;

    let use_lon = if latest_frame_flag == CPRFormat::Even {
        cpr_lon_even
    } else {
        cpr_lon_odd
    };

    let lon_one = d_lon * (calc_modulo(m, n) + use_lon);
    let lon_two = &lon_one + 90.0;
    let lon_three = &lon_one + 180.0;
    let lon_four = &lon_one + 270.0;

    // find the closest latitude to the reference position

    let lat = if (reference_position.latitude - lat_northern).abs()
        < (reference_position.latitude - lat_southern).abs()
    {
        lat_northern
    } else {
        lat_southern
    };

    // using haversign distance, now that we have a lat, find the closest lat/lon pair from lon_one, lon_two, lon_three, lon_four to the reference position

    let mut lon = lon_one;

    let mut min_distance = haversine_distance_position(
        reference_position,
        &Position {
            latitude: lat,
            longitude: lon_one,
        },
    );

    let distance_two = haversine_distance_position(
        reference_position,
        &Position {
            latitude: lat,
            longitude: lon_two,
        },
    );

    let distance_three = haversine_distance_position(
        reference_position,
        &Position {
            latitude: lat,
            longitude: lon_three,
        },
    );

    let distance_four = haversine_distance_position(
        reference_position,
        &Position {
            latitude: lat,
            longitude: lon_four,
        },
    );

    if distance_two < min_distance {
        min_distance = distance_two;
        lon = lon_two;
    }

    if distance_three < min_distance {
        min_distance = distance_three;
        lon = lon_three;
    }

    if distance_four < min_distance {
        lon = lon_four;
    }

    Some(Position {
        latitude: lat,
        longitude: lon,
    })
}

#[must_use]
pub fn get_bearing_from_positions(position: &Position, other: &Position) -> f64 {
    let lat1 = position.latitude.to_radians();
    let lat2 = other.latitude.to_radians();
    let long1 = position.longitude.to_radians();
    let long2 = other.longitude.to_radians();

    let delta_long = long2 - long1;

    let y = libm::sin(delta_long) * libm::cos(lat2);
    let x = libm::cos(lat1) * libm::sin(lat2)
        - libm::sin(lat1) * libm::cos(lat2) * libm::cos(delta_long);

    let bearing = libm::atan2(y, x).to_degrees();
    // degrees should be between 0 and 360

    if bearing < 0.0 {
        bearing + 360.0
    } else {
        bearing
    }
}

#[must_use]
pub fn km_to_nm(km: f64) -> f64 {
    km * 0.539_957
}

#[must_use]
pub fn get_distance_and_direction_from_reference_position(
    aircraft_position: &Position,
    reference_position: &Position,
) -> (f64, f64) {
    let distance = haversine_distance_position(aircraft_position, reference_position);
    let bearing = get_bearing_from_positions(aircraft_position, reference_position);

    (distance, bearing)
}

#[must_use]
pub fn is_lat_lon_sane(position: Position) -> bool {
    position.latitude >= -90.0
        && position.latitude <= 90.0
        && position.longitude >= -180.0
        && position.longitude <= 180.0
}

#[cfg(test)]
mod tests {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::{
        raw::NewAdsbRawMessage,
        raw_types::{df::DF, groundspeed::GroundSpeed},
    };

    fn compare_epsilon_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn cpr_nl_high_low_lat() {
        assert!(compare_epsilon_f64(cpr_nl(89.9), 1.0));
        assert!(compare_epsilon_f64(cpr_nl(-89.9), 1.0));
        assert!(compare_epsilon_f64(cpr_nl(86.9), 2.0));
        assert!(compare_epsilon_f64(cpr_nl(-86.9), 2.0));
    }

    #[test]
    fn calculate_surface_position() {
        "debug".enable_logging();
        let even = Position {
            latitude: 115_609.0,
            longitude: 116_941.0,
        };
        let odd = Position {
            latitude: 39199.0,
            longitude: 110_269.0,
        };
        let reference_position = Position {
            latitude: 51.990,
            longitude: 4.375,
        };
        let position = get_position_from_even_odd_cpr_positions_surface(
            &even,
            &odd,
            CPRFormat::Even,
            &reference_position,
        )
        .unwrap();
        let expected_lat = 52.320_607_072_215_964;
        let expected_lon = 4.730_472_564_697_266;
        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn calculate_surface_position_from_local() {
        "debug".enable_logging();
        let aircraft_frame = Position {
            latitude: 39195.0,
            longitude: 110_320.0,
        };

        let local = Position {
            latitude: 52.320_607,
            longitude: 4.734_735,
        };

        let expected_lat = 52.320_560_519_978_15;
        let expected_lon = 4.735_735_212_053_572;

        let position =
            get_position_from_locally_unabiguous_surface(&aircraft_frame, &local, CPRFormat::Odd);

        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );

        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn calculate_surface_position_from_local_again() {
        "debug".enable_logging();
        let aircraft_frame = Position {
            latitude: 39199.0,
            longitude: 110_269.0,
        };

        let local = Position {
            latitude: 52.320_607,
            longitude: 4.734_735,
        };

        let expected_lat = 52.320_607_072_215_964;
        let expected_lon = 4.734_734_671_456_474;

        let position =
            get_position_from_locally_unabiguous_surface(&aircraft_frame, &local, CPRFormat::Odd);

        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );

        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn calculate_surface_position_from_local_kabq() {
        "debug".enable_logging();
        let aircraft_frame = Position {
            latitude: 126_995.0,
            longitude: 18218.0,
        };

        let local = Position {
            latitude: 35.040_277_777_8,
            longitude: -106.609_166_666_7,
        };

        let expected_lat = 35.037_297_394_316_07;
        let expected_lon = -106.614_389_419_555_66;

        let position =
            get_position_from_locally_unabiguous_surface(&aircraft_frame, &local, CPRFormat::Odd);

        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );

        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn decode_from_raw_surface_position() {
        "debug".enable_logging();
        let message = "8CAAC4BB401C0175F7E88A134707";

        // DF: 10001 (17/ADSB)
        // Capability: 100 (4/Level 2 xponder)
        // ICAO: 101010101100010010111011

        // Type Code: 01000 (8/Surface Position)
        // Movement: 0000001 (1/Stopped)
        // Heading/Track Status: 1 (Valid)
        // Heading/Ground Track: 1000000 (64, using maths 180*)
        // Reserved: 0
        // CPR Format: 0 (Even)
        // CPR Lat: 01011101011111011 (47867)
        // CPR Lon: 11110100010001010 (125066)

        // CRC: 000100110100011100000111

        let position = message.to_adsb_raw().unwrap();
        assert_eq!(position.crc, 0);

        // print the ICAO
        if let DF::ADSB(adsb) = &position.df {
            let transponderhex = adsb.icao.to_string();
            assert_eq!(transponderhex, "AAC4BB");
            // make sure it's decoded as a surface position

            match adsb.me {
                crate::decoders::raw_types::me::ME::SurfacePosition(surface_position) => {
                    info!("Surface position: {surface_position:?}");
                    //assert_eq!(surface_position.mov.calculate(), Some(17.0));
                    //assert_eq!(surface_position.get_heading(), Some(14.1));
                    let local = Position {
                        latitude: 35.040_277_777_8,
                        longitude: -106.609_166_666_7,
                    };

                    let aircraft_frame = Position {
                        latitude: f64::from(surface_position.lat_cpr),
                        longitude: f64::from(surface_position.lon_cpr),
                    };

                    let expected_lat = 35.047_794_342_041_016;
                    let expected_lon = -106.614_775_365_712_69;

                    let position = get_position_from_locally_unabiguous_surface(
                        &aircraft_frame,
                        &local,
                        CPRFormat::Even,
                    );
                    info!("Calculated position: {position:?}");
                    info!(
                        "Expected position: {:?}",
                        Position {
                            latitude: expected_lat,
                            longitude: expected_lon,
                        }
                    );
                    assert!(compare_epsilon_f64(position.latitude, expected_lat));
                    assert!(compare_epsilon_f64(position.longitude, expected_lon));
                    assert!(surface_position.get_ground_speed() == Some(GroundSpeed::Stopped));
                }
                _ => {
                    // return an error
                    error!("Not a surface position");
                }
            }
        }
    }

    #[test]
    fn calculate_local_unambiguous() {
        "debug".enable_logging();
        let aircraft_frame = Position {
            latitude: 93000.0,
            longitude: 51372.0,
        };
        let local = Position {
            latitude: 52.258,
            longitude: 3.919,
        };

        let expected_lat = 52.257_202_148_437_5;
        let expected_lon = 3.919_372_558_593_75;
        let position =
            get_position_from_locally_unabiguous_airborne(&aircraft_frame, &local, CPRFormat::Even);
        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn cpr_calculate_position() {
        "debug".enable_logging();
        let odd = Position {
            latitude: 74158.0,
            longitude: 50194.0,
        };
        let even = Position {
            latitude: 93000.0,
            longitude: 51372.0,
        };

        let position =
            get_position_from_even_odd_cpr_positions_airborne(&even, &odd, CPRFormat::Even)
                .unwrap();
        let expected_lat = 52.257_202_148_437_5;
        let expected_lon = 3.919_372_558_593_75;
        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn cpr_calculate_position_high_lat() {
        "debug".enable_logging();
        let even = Position {
            latitude: 108_011.0,
            longitude: 110_088.0,
        };
        let odd = Position {
            latitude: 75_050.0,
            longitude: 36_777.0,
        };
        let position =
            get_position_from_even_odd_cpr_positions_airborne(&even, &odd, CPRFormat::Odd).unwrap();
        let expected_lat = 88.917_474_261_784_96;
        let expected_lon = 101.011_047_363_281_25;
        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }

    #[test]
    fn cpr_calculate_position_negative_m() {
        "debug".enable_logging();

        /*
         * The `m` value can be negative. This test provides input
         * to test that code path.
         */

        // *8f7c0017581bb01b3e135e818c6f;
        let even = Position {
            latitude: 3_487.0,
            longitude: 4_958.0,
        };
        // *8f7c0017581bb481393da48aef5d;
        let odd = Position {
            latitude: 16_540.0,
            longitude: 81_316.0,
        };
        let position =
            get_position_from_even_odd_cpr_positions_airborne(&even, &odd, CPRFormat::Odd).unwrap();
        let expected_lat = -35.840_195_478_019_1;
        let expected_lon = 150.283_852_435_172_9;
        info!("Calculated position: {position:?}");
        info!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!(compare_epsilon_f64(position.latitude, expected_lat));
        assert!(compare_epsilon_f64(position.longitude, expected_lon));
    }
}
