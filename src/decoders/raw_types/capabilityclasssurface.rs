// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// [`ME::AircraftOperationStatus`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct CapabilityClassSurface {
    /// 0, 0 in current version, reserved as id for later versions
    #[deku(bits = "2", assert_eq = "0")]
    pub reserved0: u8,

    /// Position Offset Applied
    #[deku(bits = "1")]
    pub poa: u8,

    /// Aircraft has ADS-B 1090ES Receive Capability
    #[deku(bits = "1")]
    pub es1090: u8,

    #[deku(bits = "2")]
    pub reserved1: u8,

    /// Class B2 Ground Vehicle transmitting with less than 70 watts
    #[deku(bits = "1")]
    pub b2_low: u8,

    /// Aircraft has ADS-B UAT Receive Capability
    #[deku(bits = "1")]
    pub uat_in: u8,

    /// Navigation Accuracy Category for Velocity
    #[deku(bits = "3")]
    pub nac_v: u8,

    /// NIC Supplement used on the Surface
    #[deku(bits = "1")]
    pub nic_supplement_c: u8,
}

impl CapabilityClassSurface {
    #[must_use]
    pub const fn is_reserved_zero(&self) -> bool {
        self.reserved0 == 0 && self.reserved1 == 0
    }
}

impl fmt::Display for CapabilityClassSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "   NIC-C:              {}", self.nic_supplement_c)?;
        writeln!(f, "   NACv:               {}", self.nac_v)?;
        Ok(())
    }
}
