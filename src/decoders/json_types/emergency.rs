// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize_enum_str, Serialize_enum_str)]
// #[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Emergency {
    none,
    general,
    lifeguard,
    minfuel,
    nordo,
    unlawful,
    downed,
    reserved,
}

impl Default for Emergency {
    fn default() -> Self {
        Self::none
    }
}
