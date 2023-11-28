// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::error_handling::adsb_raw_error::ADSBRawError;
use crate::error_handling::deserialization_error::DeserializationError;
use hex;
const ADSB_RAW_START_CHARACTER: u8 = 0x2a; // The adsb raw end character sequence is is a '0x3b0a', start is '0x2a'
const ADSB_RAW_END_SEQUENCE_FINISH_CHARACTER: u8 = 0x3b;
const ADSB_RAW_END_SEQUENCE_INIT_CHARACTER: u8 = 0x0a;
const ADSB_RAW_FRAME_SMALL: usize = 14;
const ADSB_RAW_FRAME_LARGE: usize = 28;
const ADSB_RAW_MODEAC_FRAME: usize = 4;

pub struct ADSBRawFrames {
    pub frames: Vec<Vec<u8>>,
    pub left_over: Vec<u8>,
}

impl ADSBRawFrames {
    pub fn len(&self) -> usize {
        self.frames.len()
    }
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

/// Helper function to format ADSB Raw frames from bytes.
/// Expected input is a &Vec<Vec<u8>>of the raw frame(s), including the control characters to start and end the frame.
/// Does not consume the input.
/// Returns a vector of bytes, with each element of the array being a frame that can be passed in to the ADSB Raw parser.

pub fn format_adsb_raw_frames_from_bytes(bytes: &[u8]) -> ADSBRawFrames {
    let mut formatted_frames: Vec<Vec<u8>> = Vec::new();
    let mut current_frame: Vec<u8> = Vec::new();
    let mut errors_found: Vec<DeserializationError> = Vec::new();

    for (position, byte) in bytes.iter().enumerate() {
        if byte == &ADSB_RAW_END_SEQUENCE_INIT_CHARACTER && position != 0 {
            match current_frame.len() {
                ADSB_RAW_MODEAC_FRAME => {
                    // this is a valid frame size, but it's NOT one we want to decode
                    debug!("Detected a MODEAC frame, skipping");
                }
                // The frame size is valid
                ADSB_RAW_FRAME_SMALL | ADSB_RAW_FRAME_LARGE => {
                    if let Ok(frame_bytes) = hex::decode(&current_frame) {
                        formatted_frames.push(frame_bytes);
                        current_frame = Vec::new();
                    } else {
                        errors_found.push(DeserializationError::ADSBRawError(
                            ADSBRawError::HexEncodingError {
                                message: "Could not convert the {frame_string} string to bytes"
                                    .to_string(),
                            },
                        ));
                    }
                }
                // The frame size is invalid
                _ => {
                    errors_found.push(DeserializationError::ADSBRawError(
                        ADSBRawError::ByteSequenceWrong {
                            size: current_frame.len() as u8,
                        },
                    ));
                }
            }
        } else if byte == &ADSB_RAW_START_CHARACTER {
            current_frame = Vec::new();
        } else if byte != &ADSB_RAW_END_SEQUENCE_FINISH_CHARACTER
            && byte != &ADSB_RAW_END_SEQUENCE_INIT_CHARACTER
        {
            current_frame.push(*byte);
        }
        // if it's the last character, see if we have a valid frame
        if position == bytes.len() - 1 && !current_frame.is_empty() {
            debug!("Reached the end of the input, checking for a valid frame");
            match current_frame.len() {
                ADSB_RAW_MODEAC_FRAME => {
                    // this is a valid frame size, but it's NOT one we want to decode
                    debug!("Detected a MODEAC frame, skipping");
                    current_frame = Vec::new();
                }
                ADSB_RAW_FRAME_SMALL | ADSB_RAW_FRAME_LARGE => {
                    debug!("Detected a valid frame at end of input, converting to bytes");

                    if let Ok(frame_bytes) = hex::decode(&current_frame) {
                        formatted_frames.push(frame_bytes);
                        current_frame = Vec::new();
                    } else {
                        errors_found.push(DeserializationError::ADSBRawError(
                            ADSBRawError::HexEncodingError {
                                message: "Could not convert the bytes {current_frame} to a string"
                                    .to_string(),
                            },
                        ));
                    }
                }
                _ => {
                    // append the control character init to the start of the frame
                    current_frame.insert(0, ADSB_RAW_START_CHARACTER);
                    debug!("Detected unused bits at end of input, skipping and sending back")
                }
            }
        }
    }

    // log any errors in decoding

    for error in errors_found {
        error!("Error with frame: {}", error);
    }

    ADSBRawFrames {
        frames: formatted_frames,
        left_over: current_frame,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_adsb_raw_parsing_input() {
        let mut input = vec![
            0x2a, 0x35, 0x44, 0x41, 0x42, 0x45, 0x36, 0x35, 0x41, 0x32, 0x46, 0x42, 0x46, 0x41,
            0x46, 0x3b, 0x0a, 0x2a, 0x38, 0x44, 0x41, 0x31, 0x41, 0x33, 0x43, 0x43, 0x39, 0x39,
            0x30, 0x39, 0x42, 0x38, 0x31, 0x34, 0x46, 0x30, 0x30, 0x34, 0x31, 0x32, 0x37, 0x46,
            0x31, 0x31, 0x30, 0x37, 0x3b, 0x0a,
        ];

        assert_eq!(
            format_adsb_raw_frames_from_bytes(&input).len(),
            2,
            "There should be two frames in the input"
        );
        assert_eq!(
            format_adsb_raw_frames_from_bytes(&input).frames,
            [
                hex::decode("5DABE65A2FBFAF").unwrap(),
                hex::decode("8DA1A3CC9909B814F004127F1107").unwrap()
            ]
        );

        input.push(0x2a);
        input.push(0x35);
        input.push(0x34);
        input.push(0x32);
        input.push(0x34);
        input.push(0x3b);
        input.push(0x0a);
        assert_eq!(
            format_adsb_raw_frames_from_bytes(&input).len(),
            2,
            "There should be two frames in the input"
        );
        assert_eq!(
            format_adsb_raw_frames_from_bytes(&input).frames,
            [
                hex::decode("5DABE65A2FBFAF").unwrap(),
                hex::decode("8DA1A3CC9909B814F004127F1107").unwrap()
            ]
        );
    }
}
