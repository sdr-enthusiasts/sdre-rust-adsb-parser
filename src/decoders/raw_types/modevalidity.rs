// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq, Copy)]
#[deku(type = "u8", bits = "1")]
pub enum IsValidMode {
    #[deku(id = "1")]
    ValidMode,
    #[deku(id = "0")]
    InvalidMode,
}

impl fmt::Display for IsValidMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidMode => write!(f, "Valid Mode Validity"),
            Self::InvalidMode => write!(f, "Invalid Mode Validity"),
        }
    }
}
