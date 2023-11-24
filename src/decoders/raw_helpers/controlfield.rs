use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use crate::decoders::raw_helpers::capability::Capability;

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
