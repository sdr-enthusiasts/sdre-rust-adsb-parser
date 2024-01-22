// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Squawk {
    String(String),
}

impl From<&str> for Squawk {
    fn from(s: &str) -> Self {
        Squawk::String(ensure_at_least_four_digits(s))
    }
}

impl From<String> for Squawk {
    fn from(s: String) -> Self {
        Squawk::String(ensure_at_least_four_digits(&s))
    }
}

impl fmt::Display for Squawk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Squawk::String(s) => write!(f, "{}", s),
        }
    }
}

fn ensure_at_least_four_digits(s: &str) -> String {
    let mut s = s.to_string();

    while s.len() < 4 {
        s.insert(0, '0');
    }

    s
}
