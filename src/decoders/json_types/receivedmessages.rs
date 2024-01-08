// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "i32")]
pub struct ReceivedMessages {
    received_messages: i32,
}

impl Serialize for ReceivedMessages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.received_messages)
    }
}

impl From<i32> for ReceivedMessages {
    fn from(received_messages: i32) -> Self {
        Self { received_messages }
    }
}

impl fmt::Display for ReceivedMessages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.received_messages)
    }
}
