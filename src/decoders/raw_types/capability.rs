// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Transponder level and additional information (3.1.2.5.2.2.1)
#[derive(
    Serialize, Deserialize, DekuRead, DekuWrite, Debug, Clone, Copy, Eq, PartialEq, Default,
)]
#[allow(non_camel_case_types)]
#[deku(id_type = "u8", bits = "3")]
pub enum Capability {
    /// Level 1 transponder (surveillance only), and either airborne or on the ground
    #[default]
    AG_UNCERTAIN = 0x00,
    #[deku(id_pat = "0x01..=0x03")]
    Reserved,
    /// Level 2 or above transponder, on ground
    AG_GROUND = 0x04,
    /// Level 2 or above transponder, airborne
    AG_AIRBORNE = 0x05,
    /// Level 2 or above transponder, either airborne or on ground
    AG_UNCERTAIN2 = 0x06,
    /// DR field is not equal to 0, or fs field equal 2, 3, 4, or 5, and either airborne or on
    /// ground
    AG_UNCERTAIN3 = 0x07,
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Capability::AG_UNCERTAIN => write!(f, "uncertain1"),
            Capability::Reserved => write!(f, "reserved"),
            Capability::AG_GROUND => write!(f, "ground"),
            Capability::AG_AIRBORNE => write!(f, "airborne"),
            Capability::AG_UNCERTAIN2 => write!(f, "uncertain2"),
            Capability::AG_UNCERTAIN3 => write!(f, "airborne?"),
        }
    }
}
