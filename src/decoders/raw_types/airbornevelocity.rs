// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    airbornevelocitysubtype::AirborneVelocitySubType, sign::Sign,
    verticleratesource::VerticalRateSource,
};

/// [`ME::AirborneVelocity`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
pub struct AirborneVelocity {
    #[deku(bits = "3")]
    pub st: u8,
    #[deku(bits = "1")]
    pub intent_change: u8,
    #[deku(bits = "1")]
    pub reserved1: u8,
    #[deku(bits = "3")]
    pub nac_v: u8,
    #[deku(ctx = "*st")]
    pub sub_type: AirborneVelocitySubType,
    pub vrate_src: VerticalRateSource,
    pub vrate_sign: Sign,
    #[deku(endian = "big", bits = "9")]
    pub vrate_value: u16,
    #[deku(bits = "2")]
    pub reserved2: u8,
    pub gnss_sign: Sign,
    #[deku(
        bits = "7",
        map = "|gnss_baro_diff: u16| -> Result<_, DekuError> {Ok(if gnss_baro_diff > 1 {(gnss_baro_diff - 1)* 25} else { 0 })}"
    )]
    pub gnss_baro_diff: u16,
}

impl AirborneVelocity {
    #[must_use]
    pub const fn is_reserved_zero(&self) -> bool {
        self.reserved1 == 0 && self.reserved2 == 0
    }

    /// Return effective (`heading`, `ground_speed`, `vertical_rate`) for groundspeed
    #[must_use]
    pub fn calculate(&self) -> Option<(f32, f32, i16)> {
        let AirborneVelocitySubType::GroundSpeedDecoding(ground_speed) = self.sub_type else {
            return None;
        };

        let gs_ew_vel = match i16::try_from(ground_speed.ew_vel) {
            Ok(success) => success,
            Err(e) => {
                error!(
                    "Failed to convert ground_speed.ew_vel ({}) from u16 to i16. {e}",
                    ground_speed.ew_vel
                );
                return None;
            }
        };

        let gs_ns_vel = match i16::try_from(ground_speed.ns_vel) {
            Ok(success) => success,
            Err(e) => {
                error!(
                    "Failed to convert ground_speed.ns_vel ({}) from u16 to i16. {e}",
                    ground_speed.ns_vel
                );
                return None;
            }
        };

        let v_ew: f32 = f32::from((gs_ew_vel - 1) * ground_speed.ew_sign.value());
        let v_ns: f32 = f32::from((gs_ns_vel - 1) * ground_speed.ns_sign.value());
        let h: f32 = libm::atan2f(v_ew, v_ns) * (360.0 / (2.0 * std::f32::consts::PI));
        let heading: f32 = if h < 0.0 { h + 360.0 } else { h };

        // TODO: We should handle sub types 2-4 here
        let Some(vrate) = self
            .vrate_value
            .checked_sub(1)
            .and_then(|v: u16| v.checked_mul(64))
        else {
            return None;
        };

        let vrate = match i16::try_from(vrate) {
            Ok(success) => success * self.vrate_sign.value(),
            Err(_) => return None,
        };
        //.map(|v: u16| (v as i16) * self.vrate_sign.value());
        // let Some(vrate) = vrate else {
        //     return None;
        // };
        Some((heading, libm::hypotf(v_ew, v_ns), vrate))
    }
}

#[cfg(test)]
mod tests {
    use sdre_rust_logging::SetupLogging;

    use super::*;
    use crate::decoders::{
        raw::NewAdsbRawMessage,
        raw_types::{df::DF, groundspeeddecoding::GroundSpeedDecoding},
    };

    #[test]
    fn test_airborne_velocity() {
        "debug".enable_logging();

        let message = "8DC05BCF9909CF0DD00417286F1E";
        let decoded = message.to_adsb_raw().unwrap();

        let expected = AirborneVelocity {
            st: 1,
            intent_change: 0,
            reserved1: 0,
            nac_v: 1,
            sub_type: AirborneVelocitySubType::GroundSpeedDecoding(GroundSpeedDecoding {
                ew_sign: Sign::Positive,
                ew_vel: 463,
                ns_sign: Sign::Positive,
                ns_vel: 110,
            }),
            vrate_src: VerticalRateSource::GeometricAltitude,
            vrate_sign: Sign::Positive,
            vrate_value: 0b000000001,
            reserved2: 0b00,
            gnss_sign: Sign::Positive,
            gnss_baro_diff: 550,
        };

        info!("Decoded Message: {:?}", &decoded);

        if let DF::ADSB(adsb) = decoded.df {
            match adsb.me {
                crate::decoders::raw_types::me::ME::AirborneVelocity(me) => {
                    assert_eq!(me, expected);
                    let (heading, ground_speed, vertical_rate) = me.calculate().unwrap();
                    assert_eq!(heading, 76.724915);
                    assert_eq!(ground_speed, 474.68410548490033);
                    assert_eq!(vertical_rate, 0);
                }
                _ => panic!("Expected AirborneVelocity"),
            }
        }
    }
}
