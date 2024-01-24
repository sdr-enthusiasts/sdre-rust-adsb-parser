// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use crate::decoders::common_types::sda::SystemDesignAssurance;

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
    pub reserved_recv_atc_service: u8,

    #[deku(bits = "1")]
    pub single_antenna_flag: bool,

    pub system_design_assurance: SystemDesignAssurance,
}

impl OperationalMode {
    #[must_use]
    pub const fn is_reserved_zero(&self) -> bool {
        self.reserved == 0 && self.reserved_recv_atc_service == 0
    }
}

impl fmt::Display for OperationalMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.tcas_ra_active {
            write!(f, " TCAS")?;
        }
        if self.ident_switch_active {
            write!(f, " IDENT_SWITCH_ACTIVE")?;
        }
        if self.reserved_recv_atc_service != 0 {
            write!(f, " ATC")?;
        }
        if self.single_antenna_flag {
            write!(f, " SAF")?;
        }

        write!(f, " SDA={}", self.system_design_assurance)?;

        Ok(())
    }
}
