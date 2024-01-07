// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Airborne / Ground and SPI
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "3")]
pub enum FlightStatus {
    NoAlertNoSPIAirborne = 0b000,
    NoAlertNoSPIOnGround = 0b001,
    AlertNoSPIAirborne = 0b010,
    AlertNoSPIOnGround = 0b011,
    AlertSPIAirborneGround = 0b100,
    NoAlertSPIAirborneGround = 0b101,
    Reserved = 0b110,
    NotAssigned = 0b111,
}

impl fmt::Display for FlightStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FlightStatus::NoAlertNoSPIAirborne
            | FlightStatus::AlertSPIAirborneGround
            | FlightStatus::NoAlertSPIAirborneGround => write!(f, "airborne?"),
            FlightStatus::NoAlertNoSPIOnGround => write!(f, "ground?"),
            FlightStatus::AlertNoSPIAirborne => write!(f, "airborne"),
            FlightStatus::AlertNoSPIOnGround => write!(f, "ground"),
            FlightStatus::Reserved | FlightStatus::NotAssigned => write!(f, "reserved"),
        }
    }
}
