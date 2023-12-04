// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum TiSB {
    #[default]
    None,
}

impl fmt::Display for TiSB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TiSB::None => write!(f, "None"),
        }
    }
}

impl TryFrom<String> for TiSB {
    type Error = String;

    fn try_from(field: String) -> Result<Self, Self::Error> {
        match field.as_str() {
            "None" => Ok(TiSB::None),
            _ => Err(format!("Invalid TiSB field: {}", field)),
        }
    }
}
