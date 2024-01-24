// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "String")]
pub enum TransponderHex {
    TransponderHexAsString(String),
    None,
}

impl Serialize for TransponderHex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::TransponderHexAsString(transponder_hex) => {
                serializer.serialize_str(transponder_hex)
            }
            Self::None => serializer.serialize_none(),
        }
    }
}

impl Default for TransponderHex {
    fn default() -> Self {
        Self::None
    }
}

impl From<String> for TransponderHex {
    fn from(transponder_hex: String) -> Self {
        Self::TransponderHexAsString(transponder_hex.to_ascii_uppercase())
    }
}

impl fmt::Display for TransponderHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransponderHexAsString(transponder_hex) => write!(f, "{transponder_hex}"),
            Self::None => write!(f, "None"),
        }
    }
}

impl TransponderHex {
    #[must_use]
    pub fn get_transponder_hex_as_string(&self) -> String {
        match self {
            Self::TransponderHexAsString(transponder_hex) => transponder_hex.clone(),
            Self::None => String::new(),
        }
    }
}
