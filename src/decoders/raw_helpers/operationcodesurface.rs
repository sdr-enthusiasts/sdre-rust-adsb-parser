use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct OperationCodeSurface {
    #[deku(bits = "1")]
    pub poe: u8,
    #[deku(bits = "1")]
    pub cdti: u8,
    #[deku(bits = "1")]
    pub b2_low: u8,
    #[deku(bits = "3")]
    #[deku(pad_bits_before = "6")]
    pub lw: u8,
}

impl fmt::Display for OperationCodeSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.poe.eq(&1) {
            write!(f, " POE")?;
        }
        if self.cdti.eq(&1) {
            write!(f, " CDTI")?;
        }
        if self.b2_low.eq(&1) {
            write!(f, " B2_LOW")?;
        }
        if self.lw != 0 {
            write!(f, " L/W={}", self.lw)?;
        }
        Ok(())
    }
}
