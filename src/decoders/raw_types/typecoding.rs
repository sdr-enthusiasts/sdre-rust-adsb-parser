// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(id_type = "u8", bits = "5")]
#[repr(u8)]
pub enum TypeCoding {
    D = 1,
    C = 2,
    B = 3,
    A = 4,
}

impl From<u8> for TypeCoding {
    /// Used to reconstruct a `TypeCoding` from the raw ADS-B Type Code value
    /// that [`ME`](super::me::ME) already had to read (and thus consume from
    /// the bitstream) to select the `AircraftIdentification` variant.
    fn from(v: u8) -> Self {
        match v {
            1 => TypeCoding::D,
            2 => TypeCoding::C,
            3 => TypeCoding::B,
            _ => TypeCoding::A,
        }
    }
}

impl fmt::Display for TypeCoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::D => write!(f, "D"),
            Self::C => write!(f, "C"),
            Self::B => write!(f, "B"),
            Self::A => write!(f, "A"),
        }
    }
}
