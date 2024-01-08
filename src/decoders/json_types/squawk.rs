// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(try_from = "String")]
pub struct Squawk {
    code_digit1: u8,
    code_digit2: u8,
    code_digit3: u8,
    code_digit4: u8,
}

impl Serialize for Squawk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let squawk = format!(
            "{}{}{}{}",
            self.code_digit1, self.code_digit2, self.code_digit3, self.code_digit4
        );
        serializer.serialize_str(&squawk)
    }
}

impl TryFrom<String> for Squawk {
    type Error = String;

    fn try_from(squawk: String) -> Result<Self, Self::Error> {
        let squawk_as_bytes = squawk.as_bytes();
        let code_digit1 = squawk_as_bytes[0] - 48;
        let code_digit2 = squawk_as_bytes[1] - 48;
        let code_digit3 = squawk_as_bytes[2] - 48;
        let code_digit4 = squawk_as_bytes[3] - 48;

        // verify all digits are between 0 and 7
        if code_digit1 > 7 || code_digit2 > 7 || code_digit3 > 7 || code_digit4 > 7 {
            return Err(format!("Invalid squawk code: {}", squawk));
        }

        Ok(Self {
            code_digit1,
            code_digit2,
            code_digit3,
            code_digit4,
        })
    }
}

impl fmt::Display for Squawk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.code_digit1, self.code_digit2, self.code_digit3, self.code_digit4
        )
    }
}
