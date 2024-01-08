// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::decoders::json::JSONMessage;

// FIXME: does this need to exist?
pub trait ConvertToJSON {
    fn convert_to_json(&self) -> JSONMessage;
}

pub trait UpdateFromJSON {
    fn update_from_json(self, json_message: &JSONMessage);
}
