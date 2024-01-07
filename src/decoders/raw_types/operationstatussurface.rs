// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    adsbversion::ADSBVersion, capabilityclasssurface::CapabilityClassSurface,
    operationalmode::OperationalMode,
};

/// [`ME::AircraftOperationStatus`] && [`OperationStatus`] == 1
///
/// Version 2 support only
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct OperationStatusSurface {
    /// CC (14 bits)
    pub capability_class: CapabilityClassSurface,

    /// CC L/W codes
    #[deku(bits = "4")]
    pub lw_codes: u8,

    /// OM
    pub operational_mode: OperationalMode,

    /// OM last 8 bits (diff for airborne/surface)
    // TODO: parse:
    // http://www.anteni.net/adsb/Doc/1090-WP30-18-DRAFT_DO-260B-V42.pdf
    // 2.2.3.2.7.2.4.7 “GPS Antenna Offset” OM Code Subfield in Aircraft Operational Status Messages
    pub gps_antenna_offset: u8,

    pub version_number: ADSBVersion,

    #[deku(bits = "1")]
    pub nic_supplement_a: u8,

    #[deku(bits = "4")]
    #[deku(pad_bits_after = "2")] // reserved
    pub navigational_accuracy_category: u8,

    #[deku(bits = "2")]
    pub source_integrity_level: u8,

    #[deku(bits = "1")]
    pub barometric_altitude_integrity: u8,

    #[deku(bits = "1")]
    pub horizontal_reference_direction: u8,

    #[deku(bits = "1")]
    #[deku(pad_bits_after = "1")] // reserved
    pub sil_supplement: u8,
}

impl fmt::Display for OperationStatusSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Version:            {}", self.version_number)?;
        writeln!(f, "   NIC-A:              {}", self.nic_supplement_a)?;
        write!(f, "{}", self.capability_class)?;
        write!(f, "   Capability classes:")?;
        if self.lw_codes != 0 {
            writeln!(f, " L/W={}", self.lw_codes)?;
        } else {
            writeln!(f)?;
        }
        write!(f, "   Operational modes: {}", self.operational_mode)?;
        writeln!(f)?;
        writeln!(
            f,
            "   NACp:               {}",
            self.navigational_accuracy_category
        )?;
        writeln!(
            f,
            "   SIL:                {} (per hour)",
            self.source_integrity_level
        )?;
        writeln!(
            f,
            "   NICbaro:            {}",
            self.barometric_altitude_integrity
        )?;
        if self.horizontal_reference_direction == 1 {
            writeln!(f, "   Heading reference:  magnetic north")?;
        } else {
            writeln!(f, "   Heading reference:  true north")?;
        }
        Ok(())
    }
}
