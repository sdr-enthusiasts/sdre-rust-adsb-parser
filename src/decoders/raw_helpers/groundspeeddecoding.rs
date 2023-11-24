use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::sign::Sign;

/// [`ME::AirborneVelocity`] && [`AirborneVelocitySubType::GroundSpeedDecoding`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct GroundSpeedDecoding {
    pub ew_sign: Sign,
    #[deku(endian = "big", bits = "10")]
    pub ew_vel: u16,
    pub ns_sign: Sign,
    #[deku(endian = "big", bits = "10")]
    pub ns_vel: u16,
}

impl fmt::Display for GroundSpeedDecoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  EW velocity:    {}{} kt", self.ew_sign, self.ew_vel)?;
        writeln!(f, "  NS velocity:    {}{} kt", self.ns_sign, self.ns_vel)?;
        Ok(())
    }
}
