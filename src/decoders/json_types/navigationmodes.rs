// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum NavigationModes {
    autopilot,
    vnav,
    althold,
    approach,
    lnav,
    tcas,
    none,
}

impl Default for NavigationModes {
    fn default() -> Self {
        Self::none
    }
}
