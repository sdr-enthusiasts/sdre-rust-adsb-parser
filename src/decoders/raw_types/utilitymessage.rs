// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::utilitymessagetype::UtilityMessageType;

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
pub struct UtilityMessage {
    #[deku(bits = "4")]
    pub iis: u8,
    pub ids: UtilityMessageType,
}
