// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// [`ME::AircraftOperationStatus`]
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct CapabilityClassAirborne {
    #[deku(bits = "2", assert_eq = "0")]
    pub reserved0: u8,

    /// TCAS Operational
    #[deku(bits = "1")]
    pub acas: u8,

    /// 1090ES IN
    ///
    /// bit 12
    #[deku(bits = "1")]
    pub cdti: u8,

    #[deku(bits = "2", assert_eq = "0")]
    // FIXME???
    // This SHOULD be 0, but it's not always
    // The best I can tell the military will broadcast ADSB
    // but mangle some fields they shouldn't play with
    // so that "normal" ADSB receivers will ignore them
    //#[deku(bits = "2")]
    pub reserved1: u8,

    #[deku(bits = "1")]
    pub arv: u8,
    #[deku(bits = "1")]
    pub ts: u8,
    #[deku(bits = "2")]
    #[deku(pad_bits_after = "6")] //reserved
    pub tc: u8,
}

impl fmt::Display for CapabilityClassAirborne {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.acas.eq(&1) {
            write!(f, " ACAS")?;
        }
        if self.cdti.eq(&1) {
            write!(f, " CDTI")?;
        }
        if self.arv.eq(&1) {
            write!(f, " ARV")?;
        }
        if self.ts.eq(&1) {
            write!(f, " TS")?;
        }
        if self.tc.eq(&1) {
            write!(f, " TC")?;
        }
        Ok(())
    }
}
