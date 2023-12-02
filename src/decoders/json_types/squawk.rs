// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
#[serde(from = "String")]
pub struct Squawk {
    code_digit1: u8,
    code_digit2: u8,
    code_digit3: u8,
    code_digit4: u8,
}

impl From<String> for Squawk {
    fn from(squawk: String) -> Self {
        let squawk_as_bytes = squawk.as_bytes();
        let code_digit1 = squawk_as_bytes[0] - 48;
        let code_digit2 = squawk_as_bytes[1] - 48;
        let code_digit3 = squawk_as_bytes[2] - 48;
        let code_digit4 = squawk_as_bytes[3] - 48;

        // verify all digits are between 0 and 7
        if code_digit1 > 7 || code_digit2 > 7 || code_digit3 > 7 || code_digit4 > 7 {
            panic!("Invalid squawk code: {}", squawk); // TODO: This should just return an Err
        }

        Self {
            code_digit1,
            code_digit2,
            code_digit3,
            code_digit4,
        }
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
