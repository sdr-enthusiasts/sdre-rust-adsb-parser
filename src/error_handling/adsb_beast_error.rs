use custom_error::custom_error;

custom_error! {pub ADSBBeastError
    StringError{message: String}            = "Error converting the byte sequence to a string: {message}",
    ShortFrameTooShort                      = "Found a short frame but not enough bytes to decode it",
    LongFrameTooShort                       = "Found a long frame but not enough bytes to decode it",
    HexEncodingError{message: String}       = "Could not convert the bytes {message} to a string: {message}",
}
