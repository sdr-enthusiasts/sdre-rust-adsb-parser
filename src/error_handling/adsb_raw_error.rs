// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use custom_error::custom_error;

custom_error! {pub ADSBRawError
    ByteSequenceWrong{size: usize}             = "Not enough bytes in the sequence to parse the message. ADSB Raw messages should be 14 or 28 bytes long. Found {size} bytes.",
    HexEncodingError{message: String}       = "Error converting the in input byte sequence to hex: {message}",
}

custom_error! {pub WrongType
    WrongType{message: String}              = "Wrong type: {message}",
}
