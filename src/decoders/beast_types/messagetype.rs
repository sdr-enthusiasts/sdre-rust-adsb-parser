// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
#[deku(type = "u8", bits = "8")]
pub enum MessageType {
    #[deku(id = "49")]
    ModeAC,
    #[deku(id = "50")]
    ShortFrame,
    #[deku(id = "51")]
    LongFrame,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MessageType::ModeAC => write!(f, "ModeAC"),
            MessageType::ShortFrame => write!(f, "ShortFrame"),
            MessageType::LongFrame => write!(f, "LongFrame"),
        }
    }
}
