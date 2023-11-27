// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use custom_error::custom_error;

custom_error! {pub ADSBBeastError
    StringError{message: String}            = "Error converting the byte sequence to a string: {message}",
    ShortFrameTooShort                      = "Found a short frame but not enough bytes to decode it",
    LongFrameTooShort                       = "Found a long frame but not enough bytes to decode it",
    HexEncodingError{message: String}       = "Could not convert the bytes {message} to a string: {message}",
}
