use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{
    operationstatusairborne::OperationStatusAirborne,
    operationstatussurface::OperationStatusSurface,
};

/// Aircraft Operational Status Subtype
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "3")]
pub enum OperationStatus {
    #[deku(id = "0")]
    Airborne(OperationStatusAirborne),

    #[deku(id = "1")]
    Surface(OperationStatusSurface),

    #[deku(id_pat = "2..=7")]
    Reserved(#[deku(bits = "5")] u8, [u8; 5]),
}

impl fmt::Display for OperationStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OperationStatus::Airborne(opstatus_airborne) => {
                write!(f, "{}", opstatus_airborne)
            }
            OperationStatus::Surface(opstatus_surface) => {
                write!(f, "{}", opstatus_surface)
            }
            OperationStatus::Reserved(..) => {
                write!(f, "Reserved")
            }
        }
    }
}
