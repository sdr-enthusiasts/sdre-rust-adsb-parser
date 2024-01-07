// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use custom_error::custom_error;

custom_error! {pub ADSBBeastError
    ShortFrameTooShort{message: usize}                      = "Found a short frame but not enough bytes ({message}) to decode it",
    LongFrameTooShort {message: usize}                      = "Found a long frame but not enough bytes  ({message}) to decode it",
    ModeACFrameTooShort {message: usize}                    = "Found a Mode A/C frame but not enough bytes  ({message}) to decode it",
    StartSequenceError {message: String}                    = "Found a start character ({message}) that wasn't a start sequence",
    FrameTypeNone                                           = "We should be working on a frame but the frame type is None",
}
