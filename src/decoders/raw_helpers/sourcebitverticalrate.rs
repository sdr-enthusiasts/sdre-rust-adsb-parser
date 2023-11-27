// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum SourceBitVerticalRate {
    GNSS = 0,
    Barometer = 1,
}

impl fmt::Display for SourceBitVerticalRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SourceBitVerticalRate::GNSS => write!(f, "GNSS"),
            SourceBitVerticalRate::Barometer => write!(f, "barometer"),
        }
    }
}
