use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum StatusForGroundTrack {
    Invalid = 0,
    Valid = 1,
}

impl fmt::Display for StatusForGroundTrack {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StatusForGroundTrack::Invalid => write!(f, "invalid"),
            StatusForGroundTrack::Valid => write!(f, "valid"),
        }
    }
}
