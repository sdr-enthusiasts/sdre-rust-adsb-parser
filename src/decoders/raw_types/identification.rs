// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{helper_functions::aircraft_identification_read, typecoding::TypeCoding};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
pub struct Identification {
    pub tc: TypeCoding,

    #[deku(bits = "3")]
    pub ca: u8,

    /// N-Number / Tail Number
    #[deku(reader = "aircraft_identification_read(deku::rest)")]
    pub cn: String,
}
