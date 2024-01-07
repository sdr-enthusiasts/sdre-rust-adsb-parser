// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::error_handling::adsb_raw_error::ADSBRawError;
use hex;
const ADSB_RAW_START_CHARACTER: u8 = 0x2a; // The adsb raw end character sequence is is a '0x3b0a', start is '0x2a'
const ADSB_RAW_END_SEQUENCE_FINISH_CHARACTER: u8 = 0x0a;
const ADSB_RAW_END_SEQUENCE_INIT_CHARACTER: u8 = 0x3b;
const ADSB_RAW_FRAME_SMALL: usize = 14;
const ADSB_RAW_FRAME_LARGE: usize = 28;
const ADSB_RAW_MODEAC_FRAME: usize = 4;

pub struct ADSBRawFrames {
    pub frames: Vec<Vec<u8>>,
    pub left_over: Vec<u8>,
    pub errors: Vec<ADSBRawError>,
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
    let mut errors_found: Vec<ADSBRawError> = Vec::new();

    let mut byte_iter = bytes.iter().peekable();

    while let Some(byte) = byte_iter.next() {
        // if the byte, and the next one, are the end sequence, we should have a frame
        // verify the frame length is correct, and if so, add it to the list of frames
        if *byte == ADSB_RAW_END_SEQUENCE_INIT_CHARACTER
            && byte_iter.peek() == Some(&&ADSB_RAW_END_SEQUENCE_FINISH_CHARACTER)
        {
            // verify we have a valid frame length
            if current_frame.len() == ADSB_RAW_MODEAC_FRAME {
                // we will ignore the modeac frame
                current_frame.clear();
                _ = byte_iter.next();
                continue;
            }

            if current_frame.len() != ADSB_RAW_FRAME_SMALL
                && current_frame.len() != ADSB_RAW_FRAME_LARGE
            {
                errors_found.push(ADSBRawError::ByteSequenceWrong {
                    size: current_frame.len() as u8,
                });
                current_frame.clear();
                _ = byte_iter.next();
                continue;
            }

            // we've ended up here, the frame size should be valid
            if let Ok(frame_bytes) = hex::decode(&current_frame) {
                formatted_frames.push(frame_bytes);
            } else {
                errors_found.push(ADSBRawError::HexEncodingError {
                    message: "Could not convert the {frame_string} string to bytes".to_string(),
                });
            }

            current_frame.clear();
            _ = byte_iter.next();
            continue;
        }

        // If we've encountered the start character, we will just continue to the next loop iteration
        if *byte == ADSB_RAW_START_CHARACTER {
            continue;
        }

        // if we've ended up here we should just append the byte to the current frame
        current_frame.push(*byte);
    }

    // current frame should be clear, but just in case, we will log it
    if !current_frame.is_empty() {
        debug!("Left over frame: {:?}", current_frame);
    }

    ADSBRawFrames {
        frames: formatted_frames,
        left_over: current_frame,
        errors: errors_found,
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
