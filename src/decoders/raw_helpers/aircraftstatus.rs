use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    aircraftstatustype::AircraftStatusType, emergencystate::EmergencyState,
    helper_functions::decode_id13_field,
};

/// Table: A-2-97
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct AircraftStatus {
    pub sub_type: AircraftStatusType,
    pub emergency_state: EmergencyState,
    #[deku(
        bits = "13",
        endian = "big",
        map = "|squawk: u32| -> Result<_, DekuError> {Ok(decode_id13_field(squawk))}"
    )]
    pub squawk: u32,
}

impl fmt::Display for AircraftStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Subtype:        {}", self.sub_type)?;
        writeln!(f, "  Emergency:      {}", self.emergency_state)?;
        writeln!(f, "  Squawk:         {squawk:x?}", squawk = self.squawk)?;
        Ok(())
    }
}
