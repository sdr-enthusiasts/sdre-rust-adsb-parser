// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

/// SPI Condition
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq, Default)]
#[deku(type = "u8", bits = "2")]
pub enum SurveillanceStatus {
    #[default]
    NoCondition = 0,
    PermanentAlert = 1,
    TemporaryAlert = 2,
    SPICondition = 3,
}
