use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
#[deku(type = "u8", bits = "8")]
pub enum MessageType {
    #[deku(id = "49")]
    ModeAC,
    #[deku(id = "50")]
    ShortFrame,
    #[deku(id = "51")]
    LongFrame,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::ModeAC => write!(f, "ModeAC"),
            MessageType::ShortFrame => write!(f, "ShortFrame"),
            MessageType::LongFrame => write!(f, "LongFrame"),
        }
    }
}
