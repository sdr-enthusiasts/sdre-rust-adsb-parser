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
    #[deku(bits = "5")]
    pub nac_v: u8,
    #[deku(ctx = "*st")]
    pub sub_type: AirborneVelocitySubType,
    pub vrate_src: VerticalRateSource,
    pub vrate_sign: Sign,
    #[deku(endian = "big", bits = "9")]
    pub vrate_value: u16,
    #[deku(bits = "2")]
    pub reverved: u8,
    pub gnss_sign: Sign,
    #[deku(
        bits = "7",
        map = "|gnss_baro_diff: u16| -> Result<_, DekuError> {Ok(if gnss_baro_diff > 1 {(gnss_baro_diff - 1)* 25} else { 0 })}"
    )]
    pub gnss_baro_diff: u16,
}

impl AirborneVelocity {
    /// Return effective (`heading`, `ground_speed`, `vertical_rate`) for groundspeed
    #[must_use]
    pub fn calculate(&self) -> Option<(f32, f64, i16)> {
        let AirborneVelocitySubType::GroundSpeedDecoding(ground_speed) = self.sub_type else {
            return None;
        };
        let v_ew: f64 = f64::from((ground_speed.ew_vel as i16 - 1) * ground_speed.ew_sign.value());
        let v_ns: f64 = f64::from((ground_speed.ns_vel as i16 - 1) * ground_speed.ns_sign.value());
        let h: f64 = libm::atan2(v_ew, v_ns) * (360.0 / (2.0 * std::f64::consts::PI));
        let heading: f64 = if h < 0.0 { h + 360.0 } else { h };

        let vrate: Option<i16> = self
            .vrate_value
            .checked_sub(1)
            .and_then(|v: u16| v.checked_mul(64))
            .map(|v: u16| (v as i16) * self.vrate_sign.value());
        let Some(vrate) = vrate else {
            return None;
        };
        Some((heading as f32, libm::hypot(v_ew, v_ns), vrate))
    }
}
