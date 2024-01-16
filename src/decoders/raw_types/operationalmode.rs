// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// `OperationMode` field not including the last 8 bits that are different for Surface/Airborne
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct OperationalMode {
    /// (0, 0) in Version 2, reserved for other values
    #[deku(bits = "2", assert_eq = "0")]
    pub reserved: u8,

    #[deku(bits = "1")]
    pub tcas_ra_active: bool,

    #[deku(bits = "1")]
    pub ident_switch_active: bool,

    #[deku(bits = "1")]
    pub reserved_recv_atc_service: bool,

    #[deku(bits = "1")]
    pub single_antenna_flag: bool,

    #[deku(bits = "2")]
    pub system_design_assurance: u8,
}

impl fmt::Display for OperationalMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.tcas_ra_active {
            write!(f, " TCAS")?;
        }
        if self.ident_switch_active {
            write!(f, " IDENT_SWITCH_ACTIVE")?;
        }
        if self.reserved_recv_atc_service {
            write!(f, " ATC")?;
        }
        if self.single_antenna_flag {
            write!(f, " SAF")?;
        }
        if self.system_design_assurance != 0 {
            write!(f, " SDA={}", self.system_design_assurance)?;
        }
        Ok(())
    }
}
