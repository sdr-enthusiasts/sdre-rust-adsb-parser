// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// ADS-B Defined from different ICAO documents
///
/// reference: ICAO 9871 (5.3.2.3)
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(id_type = "u8", bits = "3")]
pub enum ADSBVersion {
    #[deku(id = "0")]
    ADSBVersion0,
    #[deku(id = "1")]
    ADSBVersion1,
    #[deku(id = "2")]
    ADSBVersion2,
    #[deku(id = "3")]
    ADSBVersion3,
    #[deku(id_pat = "_")]
    Unknown,
}

impl fmt::Display for ADSBVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deku_id().unwrap())
    }
}
