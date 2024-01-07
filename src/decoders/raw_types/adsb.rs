// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Adsb {
    /// `to_string` with DF.id() input
    pub fn to_string(&self, address_type: &str) -> String {
        self.me
            .to_string(self.icao, address_type, self.capability, true)
            .unwrap()
    }
}
