// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

// emitter category https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize_enum_str, Serialize_enum_str)]
pub enum EmitterCategory {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
}

impl Default for EmitterCategory {
    fn default() -> Self {
        Self::A0
    }
}
