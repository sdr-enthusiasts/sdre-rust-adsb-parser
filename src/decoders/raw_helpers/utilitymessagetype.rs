use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Message Type
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "2")]
pub enum UtilityMessageType {
    NoInformation = 0b00,
    CommB = 0b01,
    CommC = 0b10,
    CommD = 0b11,
}

impl fmt::Display for UtilityMessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UtilityMessageType::NoInformation => write!(f, "no information"),
            UtilityMessageType::CommB => write!(f, "Comm-B"),
            UtilityMessageType::CommC => write!(f, "Comm-C"),
            UtilityMessageType::CommD => write!(f, "Comm-D"),
        }
    }
}
