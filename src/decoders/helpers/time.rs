// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::time::SystemTime;

use crate::decoders::json_types::timestamp::TimeStamp;

// Not all messages have a timestamp, so we'll use the current time if one isn't provided.
#[must_use]
pub fn get_time_as_timestamp() -> TimeStamp {
    match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => TimeStamp::from(n.as_secs_f64()),
        Err(_) => TimeStamp::default(),
    }
}

#[must_use]
pub fn get_time_as_f64() -> f64 {
    match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => n.as_secs_f64(),
        Err(_) => 0.0,
    }
}
