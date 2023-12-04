// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

// 000 : no alert, no SPI, aircraft is airborne
// 001 : no alert, no SPI, aircraft is on-ground
// 010 : alert, no SPI, aircraft is airborne
// 011 : alert, no SPI, aircraft is on-ground
// 100 : alert, SPI, aircraft is airborne or on-ground
// 101 : no alert, SPI, aircraft is airborne or on-ground
// 110 : reserved
// 111 : not assigned
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(try_from = "u8")]
pub enum FlightStatusAlertBit {
    NoAlertNoSPIAirborne,
    NoAlertNoSPIOnGround,
    AlertNoSPIAirborne,
    AlertNoSPIOnGround,
    AlertSPIAirborneOrOnGround,
    NoAlertSPIAirborneOrOnGround,
    Reserved,
    NotAssigned,
}

impl TryFrom<u8> for FlightStatusAlertBit {
    type Error = String;

    fn try_from(flight_status_alert_bit: u8) -> Result<Self, Self::Error> {
        match flight_status_alert_bit {
            0b000 => Ok(FlightStatusAlertBit::NoAlertNoSPIAirborne),
            0b001 => Ok(FlightStatusAlertBit::NoAlertNoSPIOnGround),
            0b010 => Ok(FlightStatusAlertBit::AlertNoSPIAirborne),
            0b011 => Ok(FlightStatusAlertBit::AlertNoSPIOnGround),
            0b100 => Ok(FlightStatusAlertBit::AlertSPIAirborneOrOnGround),
            0b101 => Ok(FlightStatusAlertBit::NoAlertSPIAirborneOrOnGround),
            0b110 => Ok(FlightStatusAlertBit::Reserved),
            0b111 => Ok(FlightStatusAlertBit::NotAssigned),
            _ => Err(format!(
                "FlightStatusAlertBit should be a value between 0 and 7, inclusive. Found: {}",
                flight_status_alert_bit
            )),
        }
    }
}

impl fmt::Display for FlightStatusAlertBit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlightStatusAlertBit::NoAlertNoSPIAirborne => {
                write!(f, "No alert, no SPI, aircraft is airborne")
            }
            FlightStatusAlertBit::NoAlertNoSPIOnGround => {
                write!(f, "No alert, no SPI, aircraft is on-ground")
            }
            FlightStatusAlertBit::AlertNoSPIAirborne => {
                write!(f, "Alert, no SPI, aircraft is airborne")
            }
            FlightStatusAlertBit::AlertNoSPIOnGround => {
                write!(f, "Alert, no SPI, aircraft is on-ground")
            }
            FlightStatusAlertBit::AlertSPIAirborneOrOnGround => {
                write!(f, "Alert, SPI, aircraft is airborne or on-ground")
            }
            FlightStatusAlertBit::NoAlertSPIAirborneOrOnGround => {
                write!(f, "No alert, SPI, aircraft is airborne or on-ground")
            }
            FlightStatusAlertBit::Reserved => write!(f, "Reserved"),
            FlightStatusAlertBit::NotAssigned => {
                write!(f, "Not assigned")
            }
        }
    }
}
