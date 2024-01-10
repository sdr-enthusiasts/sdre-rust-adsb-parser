// Copyright (c) 2024 Frederick Clausen II and authors of libadsb_deku

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://raw.githubusercontent.com/rsadsb/adsb_deku/master/libadsb_deku/src/cpr.rs

/*!
Compact Position Reporting for [`Position`] Reporting

reference: ICAO 9871 (D.2.4.7)
!*/

use crate::decoders::raw_types::{altitude::Altitude, cprheaders::CPRFormat};

const NZ: f64 = 15.0;
const D_LAT_EVEN: f64 = 360.0 / (4.0 * NZ);
const D_LAT_ODD: f64 = 360.0 / (4.0 * NZ - 1.0);

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

pub fn calc_modulo(x: f64, y: f64) -> f64 {
    x - y * libm::floor(x / y)
}

/// Calculate Globally unambiguous position decoding
///
/// Using both an Odd and Even `Altitude`, calculate the latitude/longitude
///
/// reference: ICAO 9871 (D.2.4.7.7)

pub fn get_position(
    even_frame: &Altitude,
    odd_frame: &Altitude,
    latest_frame_flag: CPRFormat,
) -> Option<Position> {
    let cpr_lat_even = f64::from(even_frame.lat_cpr) / CPR_MAX;
    let cpr_lat_odd = f64::from(odd_frame.lat_cpr) / CPR_MAX;
    let cpr_lon_even = f64::from(even_frame.lon_cpr) / CPR_MAX;
    let cpr_lon_odd = f64::from(odd_frame.lon_cpr) / CPR_MAX;

    let j = libm::floor(59.0 * cpr_lat_even - 60.0 * cpr_lat_odd + 0.5);

    let mut lat_even = D_LAT_EVEN * (calc_modulo(j, 60.0) + cpr_lat_even);
    let mut lat_odd = D_LAT_ODD * (calc_modulo(j, 59.0) + cpr_lat_odd);

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
        warn!("NL even and NL odd are not the same");
        warn!("NL even: {}", nl_even);
        warn!("NL odd: {}", nl_odd);
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
    fn cpr_calculate_position() {
        let odd = Altitude {
            odd_flag: CPRFormat::Odd,
            lat_cpr: 74158,
            lon_cpr: 50194,
            ..Altitude::default()
        };
        let even = Altitude {
            odd_flag: CPRFormat::Even,
            lat_cpr: 93000,
            lon_cpr: 51372,
            ..Altitude::default()
        };

        let position = get_position(&even, &odd, even.odd_flag).unwrap();
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
        let even = Altitude {
            odd_flag: CPRFormat::Even,
            lat_cpr: 108_011,
            lon_cpr: 110_088,
            ..Altitude::default()
        };
        let odd = Altitude {
            odd_flag: CPRFormat::Odd,
            lat_cpr: 75_050,
            lon_cpr: 36_777,
            ..Altitude::default()
        };
        let position = get_position(&even, &odd, odd.odd_flag).unwrap();
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
        let even = Altitude {
            odd_flag: CPRFormat::Even,
            lat_cpr: 3_487,
            lon_cpr: 4_958,
            ..Altitude::default()
        };
        // *8f7c0017581bb481393da48aef5d;
        let odd = Altitude {
            odd_flag: CPRFormat::Odd,
            lat_cpr: 16_540,
            lon_cpr: 81_316,
            ..Altitude::default()
        };
        let position = get_position(&even, &odd, odd.odd_flag).unwrap();
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
