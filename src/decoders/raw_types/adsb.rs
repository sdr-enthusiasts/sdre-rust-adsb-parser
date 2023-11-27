// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{capability::Capability, icao::ICAO, me::ME};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Adsb {
    // Transponder Capability
    pub capability: Capability,
    // ICAO aircraft address
    pub icao: ICAO,
    // // Message, extended Squitter
    pub me: ME,
    // // Parity/Interrogator ID
    pub pi: ICAO,
}

impl fmt::Display for Adsb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self.me.to_string(self.icao, "ADS-B", self.capability, true)
        )
    }
}
