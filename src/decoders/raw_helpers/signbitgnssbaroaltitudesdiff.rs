use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum SignBitGNSSBaroAltitudesDiff {
    Above = 0,
    Below = 1,
}

impl fmt::Display for SignBitGNSSBaroAltitudesDiff {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SignBitGNSSBaroAltitudesDiff::Above => write!(f, "above"),
            SignBitGNSSBaroAltitudesDiff::Below => write!(f, "below"),
        }
    }
}
