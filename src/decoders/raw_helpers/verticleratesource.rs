use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum VerticalRateSource {
    BarometricPressureAltitude = 0,
    GeometricAltitude = 1,
}

impl fmt::Display for VerticalRateSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            VerticalRateSource::BarometricPressureAltitude => write!(f, "barometric"),
            VerticalRateSource::GeometricAltitude => write!(f, "GNSS"),
        }
    }
}
