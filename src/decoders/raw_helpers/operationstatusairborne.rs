use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    adsbversion::ADSBVersion, capabilityclassairborne::CapabilityClassAirborne,
    operationalmode::OperationalMode,
};

/// [`ME::AircraftOperationStatus`] && [`OperationStatus`] == 0
///
/// Version 2 support only
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct OperationStatusAirborne {
    /// CC (16 bits)
    pub capability_class: CapabilityClassAirborne,

    /// OM
    pub operational_mode: OperationalMode,

    #[deku(pad_bytes_before = "1")] // reserved: OM last 8 bits (diff for airborne/surface)
    pub version_number: ADSBVersion,

    #[deku(bits = "1")]
    pub nic_supplement_a: u8,

    #[deku(bits = "4")]
    pub navigational_accuracy_category: u8,

    #[deku(bits = "2")]
    pub geometric_vertical_accuracy: u8,

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

impl fmt::Display for OperationStatusAirborne {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "   Version:            {}", self.version_number)?;
        writeln!(f, "   Capability classes:{}", self.capability_class)?;
        writeln!(f, "   Operational modes: {}", self.operational_mode)?;
        writeln!(f, "   NIC-A:              {}", self.nic_supplement_a)?;
        writeln!(
            f,
            "   NACp:               {}",
            self.navigational_accuracy_category
        )?;
        writeln!(
            f,
            "   GVA:                {}",
            self.geometric_vertical_accuracy
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
