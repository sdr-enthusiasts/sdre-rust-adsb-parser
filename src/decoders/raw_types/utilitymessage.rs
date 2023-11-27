// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::utilitymessagetype::UtilityMessageType;

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct UtilityMessage {
    #[deku(bits = "4")]
    pub iis: u8,
    pub ids: UtilityMessageType,
}

impl fmt::Display for UtilityMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  IIS:           {}", self.iis)?;
        writeln!(f, "  IDS:           {}", self.ids)?;
        Ok(())
    }
}
