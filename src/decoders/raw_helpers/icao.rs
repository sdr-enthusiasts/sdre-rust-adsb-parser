// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, num};

/// ICAO Address; Mode S transponder code
#[derive(
    Deserialize,
    Serialize,
    DekuRead,
    DekuWrite,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct ICAO(pub [u8; 3]);

impl fmt::Display for ICAO {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.0[0])?;
        write!(f, "{:02x}", self.0[1])?;
        write!(f, "{:02x}", self.0[2])?;
        Ok(())
    }
}

impl FromStr for ICAO {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u32 = u32::from_str_radix(s, 16)?;
        let bytes = num.to_be_bytes();
        let num: [u8; 3] = [bytes[1], bytes[2], bytes[3]];
        Ok(Self(num))
    }
}
