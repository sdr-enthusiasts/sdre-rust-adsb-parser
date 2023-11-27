// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use custom_error::custom_error;

custom_error! {pub ADSBJSONError
    InvalidJSON{message: String}            = "Error converting to JSON: {message}",
}
