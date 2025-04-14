// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(id_type = "u8", bits = "1")]
pub enum IsFMS {
    #[deku(id = "1")]
    FMS,
    #[deku(id = "0")]
    Autopilot,
}

impl fmt::Display for IsFMS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FMS => write!(f, "FMS"),
            Self::Autopilot => write!(f, "Autopilot"),
        }
    }
}
