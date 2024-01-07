// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use crate::decoders::raw_types::capability::Capability;

use super::{controlfieldtype::ControlFieldType, icao::ICAO, me::ME};

/// Control Field (B.3) for [`crate::DF::TisB`]
///
/// reference: ICAO 9871
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
pub struct ControlField {
    t: ControlFieldType,
    /// AA: Address, Announced
    pub aa: ICAO,
    /// ME: message, extended quitter
    pub me: ME,
}

impl fmt::Display for ControlField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.me.to_string(
                self.aa,
                &format!("{}", self.t),
                Capability::AG_UNCERTAIN3,
                false,
            )?
        )
    }
}
