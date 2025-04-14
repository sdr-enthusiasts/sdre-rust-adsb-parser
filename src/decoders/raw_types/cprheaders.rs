// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Even / Odd
#[derive(
    Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Default,
)]
#[deku(id_type = "u8", bits = "1")]
pub enum CPRFormat {
    #[default]
    Even = 0,
    Odd = 1,
}

impl fmt::Display for CPRFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CPRFormat::Even => write!(f, "even"),
            CPRFormat::Odd => write!(f, "odd"),
        }
    }
}
