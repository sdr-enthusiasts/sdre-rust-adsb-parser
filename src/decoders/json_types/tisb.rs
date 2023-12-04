// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]
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

impl From<String> for TiSB {
    fn from(s: String) -> Self {
        match s.as_str() {
            "None" => TiSB::None,
            _ => panic!("Invalid TiSB: {}", s),
        }
    }
}
