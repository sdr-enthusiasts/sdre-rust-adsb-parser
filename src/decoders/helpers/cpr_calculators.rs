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

use crate::decoders::raw_types::cprheaders::CPRFormat;

const NZ: f64 = 15.0;
const D_LAT_EVEN_AIRBORNE: f64 = 360.0 / (4.0 * NZ);
const D_LAT_ODD_AIRBORNE: f64 = 360.0 / (4.0 * NZ - 1.0);
const D_LAT_EVEN_SURFACE: f64 = 90.0 / (4.0 * NZ);
const D_LAT_ODD_SURFACE: f64 = 90.0 / (4.0 * NZ - 1.0);

/// 2^17 (Max of 17 bits)
const CPR_MAX: f64 = 131_072.0;

/// Post-processing of CPR into Latitude/Longitude
#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

/// The NL function uses the precomputed table from 1090-WP-9-14
/// This code is translated from <https://github.com/wiedehopf/readsb/blob/dev/cpr.c>
pub(crate) fn cpr_nl(lat: f64) -> u64 {
    let mut lat = lat;
    if lat < 0.0 {
        // Table is symmetric about the equator
        lat = -lat;
    }
    if lat < 29.911_356_86 {
        if lat < 10.470_471_30 {
            return 59;
        }
        if lat < 14.828_174_37 {
            return 58;
        }
        if lat < 18.186_263_57 {
            return 57;
        }
        if lat < 21.029_394_93 {
            return 56;
        }
        if lat < 23.545_044_87 {
            return 55;
        }
        if lat < 25.829_247_07 {
            return 54;
        }
        if lat < 27.938_987_10 {
            return 53;
        }
        // < 29.91135686
        return 52;
    }
    if lat < 44.194_549_51 {
        if lat < 31.772_097_08 {
            return 51;
        }
        if lat < 33.539_934_36 {
            return 50;
        }
        if lat < 35.228_995_98 {
            return 49;
        }
        if lat < 36.850_251_08 {
            return 48;
        }
        if lat < 38.412_418_92 {
            return 47;
        }
        if lat < 39.922_566_84 {
            return 46;
        }
        if lat < 41.386_518_32 {
            return 45;
        }
        if lat < 42.809_140_12 {
            return 44;
        }
        // < 44.19454951
        return 43;
    }
    if lat < 59.954_592_77 {
        if lat < 45.546_267_23 {
            return 42;
        }
        if lat < 46.867_332_52 {
            return 41;
        }
        if lat < 48.160_391_28 {
            return 40;
        }
        if lat < 49.427_764_39 {
            return 39;
        }
        if lat < 50.671_501_66 {
            return 38;
        }
        if lat < 51.893_424_69 {
            return 37;
        }
        if lat < 53.095_161_53 {
            return 36;
        }
        if lat < 54.278_174_72 {
            return 35;
        }
        if lat < 55.443_784_44 {
            return 34;
        }
        if lat < 56.593_187_56 {
            return 33;
        }
        if lat < 57.727_473_54 {
            return 32;
        }
        if lat < 58.847_637_76 {
            return 31;
        }
        // < 59.95459277
        return 30;
    }
    if lat < 61.049_177_74 {
        return 29;
    }
    if lat < 62.132_166_59 {
        return 28;
    }
    if lat < 63.204_274_79 {
        return 27;
    }
    if lat < 64.266_165_23 {
        return 26;
    }
    if lat < 65.318_453_10 {
        return 25;
    }
    if lat < 66.361_710_08 {
        return 24;
    }
    if lat < 67.396_467_74 {
        return 23;
    }
    if lat < 68.423_220_22 {
        return 22;
    }
    if lat < 69.442_426_31 {
        return 21;
    }
    if lat < 70.454_510_75 {
        return 20;
    }
    if lat < 71.459_864_73 {
        return 19;
    }
    if lat < 72.458_845_45 {
        return 18;
    }
    if lat < 73.451_774_42 {
        return 17;
    }
    if lat < 74.438_934_16 {
        return 16;
    }
    if lat < 75.420_562_57 {
        return 15;
    }
    if lat < 76.396_843_91 {
        return 14;
    }
    if lat < 77.367_894_61 {
        return 13;
    }
    if lat < 78.333_740_83 {
        return 12;
    }
    if lat < 79.294_282_25 {
        return 11;
    }
    if lat < 80.249_232_13 {
        return 10;
    }
    if lat < 81.198_013_49 {
        return 9;
    }
    if lat < 82.139_569_81 {
        return 8;
    }
    if lat < 83.071_994_45 {
        return 7;
    }
    if lat < 83.991_735_63 {
        return 6;
    }
    if lat < 84.891_661_91 {
        return 5;
    }
    if lat < 85.755_416_21 {
        return 4;
    }
    if lat < 86.535_369_98 {
        return 3;
    }
    if lat < 87.000_000_00 {
        return 2;
    }
    1
}

pub fn haversine_distance_position(position: &Position, other: &Position) -> f64 {
    let lat1 = position.latitude;
    let lat2 = other.latitude;
    let long1 = position.longitude;
    let long2 = other.longitude;
    haversine_distance((lat1, long1), (lat2, long2))
}

// https://en.wikipedia.org/wiki/Haversine_formula
pub fn haversine_distance(s: (f64, f64), other: (f64, f64)) -> f64 {
    // kilometers
    let lat1_rad = s.0.to_radians();
    let lat2_rad = other.0.to_radians();
    let long1_rad = s.1.to_radians();
    let long2_rad = other.1.to_radians();

    let x_lat = libm::sin((lat2_rad - lat1_rad) / 2.00);
    let x_long = libm::sin((long2_rad - long1_rad) / 2.00);

    let a = x_lat * x_lat
        + libm::cos(lat1_rad)
            * libm::cos(lat2_rad)
            * f64::from(libm::powf(libm::sin(x_long) as f32, 2.0));

    let c = 2.0 * libm::atan2(libm::sqrt(a), libm::sqrt(1.0 - a));

    let r = 6371.00;
    r * c
}

pub fn calc_modulo(x: f64, y: f64) -> f64 {
    x - y * libm::floor(x / y)
}

pub fn get_position_from_locally_unabiguous_surface(
    aircraft_frame: &Position,
    local: &Position,
    cpr_flag: CPRFormat,
) -> Position {
    let mut i = 0;
    let d_lat = match cpr_flag {
        CPRFormat::Even => D_LAT_EVEN_SURFACE,
        CPRFormat::Odd => {
            i = 1;
            D_LAT_ODD_SURFACE
        }
    };

    let lat_cpr = aircraft_frame.latitude / CPR_MAX;
    let lon_cpr = aircraft_frame.longitude / CPR_MAX;

    let j = libm::floor(local.latitude / d_lat)
        + libm::floor(calc_modulo(local.latitude, d_lat) / d_lat - lat_cpr + 0.5);

    let lat = d_lat * (j + lat_cpr);

    let d_lon = 90.0 / libm::fmax((cpr_nl(lat) - i) as f64, 1.0);

    let m = libm::floor(local.longitude / d_lon)
        + libm::floor(calc_modulo(local.longitude, d_lon) / d_lon - lon_cpr + 0.5);

    let lon = d_lon * (m + lon_cpr);

    Position {
        latitude: lat,
        longitude: lon,
    }
}

pub fn get_position_from_locally_unabiguous_airborne(
    aircraft_frame: &Position,
    local: &Position,
    cpr_flag: CPRFormat,
) -> Position {
    let mut i = 0;
    let d_lat = match cpr_flag {
        CPRFormat::Even => D_LAT_EVEN_AIRBORNE,
        CPRFormat::Odd => {
            i = 1;
            D_LAT_ODD_AIRBORNE
        }
    };

    let lat_cpr = aircraft_frame.latitude / CPR_MAX;
    let lon_cpr = aircraft_frame.longitude / CPR_MAX;

    let j = libm::floor(local.latitude / d_lat)
        + libm::floor(calc_modulo(local.latitude, d_lat) / d_lat - lat_cpr + 0.5);

    let lat = d_lat * (j + lat_cpr);

    let d_lon = 360.0 / libm::fmax((cpr_nl(lat) - i) as f64, 1.0);

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

    if nl_even != nl_odd {
        debug!("NL even and NL odd are not the same");
        debug!("NL even: {}", nl_even);
        debug!("NL odd: {}", nl_odd);
        return None;
    }

    // the final latitude position
    let lat = if latest_frame_flag == CPRFormat::Even {
        lat_even
    } else {
        lat_odd
    };

    let m = libm::floor(cpr_lon_even * (nl_even as f64 - 1.0) - cpr_lon_odd * nl_even as f64 + 0.5);

    let n_even = libm::fmax(nl_even as f64, 1.0);
    let n_odd = libm::fmax(nl_odd as f64 - 1.0, 1.0);

    let d_lon_even = 360.0 / n_even;
    let d_lon_odd = 360.0 / n_odd;

    let lon_even = d_lon_even * (calc_modulo(m, n_even) + cpr_lon_even);
    let lon_odd = d_lon_odd * (calc_modulo(m, n_odd) + cpr_lon_odd);

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

    if nl_even != nl_odd {
        debug!("NL even and NL odd are not the same");
        debug!("NL even: {}", nl_even);
        debug!("NL odd: {}", nl_odd);
        return None;
    }

    let lat_northern = if latest_frame_flag == CPRFormat::Even {
        lat_odd
    } else {
        lat_even // the fuck? This matches the solution in the mode-s.org work....
    };

    let lat_southern = &lat_northern - 90.0;

    let m = libm::floor(cpr_lon_even * (nl_even as f64 - 1.0) - cpr_lon_odd * nl_even as f64 + 0.5);

    let n = if latest_frame_flag == CPRFormat::Even {
        libm::fmax(nl_even as f64, 1.0)
    } else {
        libm::fmax(nl_odd as f64 - 1.0, 1.0)
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

pub fn is_lat_lon_sane(position: Position) -> bool {
    position.latitude >= -90.0
        && position.latitude <= 90.0
        && position.longitude >= -180.0
        && position.longitude <= 180.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpr_nl_high_low_lat() {
        assert_eq!(cpr_nl(89.9), 1);
        assert_eq!(cpr_nl(-89.9), 1);
        assert_eq!(cpr_nl(86.9), 2);
        assert_eq!(cpr_nl(-86.9), 2);
    }

    #[test]
    fn calculate_surface_position() {
        let even = Position {
            latitude: 115609.0,
            longitude: 116941.0,
        };
        let odd = Position {
            latitude: 39199.0,
            longitude: 110269.0,
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
        let expected_lat = 52.320607072215964;
        let expected_lon = 4.730472564697266;
        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
        assert!((position.longitude - expected_lon).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_surface_position_from_local() {
        let aircraft_frame = Position {
            latitude: 8055.0,
            longitude: 8756.0,
        };

        let local = Position {
            latitude: 35.18,
            longitude: -106.57,
        };

        let expected_lat = 35.180_000_305_175_78;
        let expected_lon = -106.569_999_694_824_22;

        let position =
            get_position_from_locally_unabiguous_surface(&aircraft_frame, &local, CPRFormat::Even);

        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
    }

    #[test]
    fn calculate_local_unambiguous() {
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
        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
        assert!((position.longitude - expected_lon).abs() < f64::EPSILON);
    }

    #[test]
    fn cpr_calculate_position() {
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
        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
        assert!((position.longitude - expected_lon).abs() < f64::EPSILON);
    }

    #[test]
    fn cpr_calculate_position_high_lat() {
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
        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
        assert!((position.longitude - expected_lon).abs() < f64::EPSILON);
    }

    #[test]
    fn cpr_calculate_position_negative_m() {
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
        let expected_lat = -35.840_195_478_019_07;
        let expected_lon = 150.283_852_435_172_9;
        println!("Calculated position: {:?}", position);
        println!(
            "Expected position: {:?}",
            Position {
                latitude: expected_lat,
                longitude: expected_lon,
            }
        );
        assert!((position.latitude - expected_lat).abs() < f64::EPSILON);
        assert!((position.longitude - expected_lon).abs() < f64::EPSILON);
    }
}
