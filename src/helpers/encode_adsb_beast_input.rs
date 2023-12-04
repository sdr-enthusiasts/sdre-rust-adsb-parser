// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

const ADSB_BEAST_START_CHARACTER: u8 = 0x1a; // The adsb beast end character sequence is is a '0x3b0a', start is '0x2a'
const ADSB_BEAST_LONG_FRAME_START_CHARACTER: u8 = 0x33;
const ADSB_BEAST_SHORT_FRAME_START_CHARACTER: u8 = 0x32;
const ADSB_BEAST_MODEAC_FRAME_START_CHARACTER: u8 = 0x31;
const ADSB_BEAST_SHORT_FRAME_LENGTH: usize = 15;
const ADSB_BEAST_LONG_FRAME_LENGTH: usize = 22;

pub struct ADSBBeastFrames {
    pub frames: Vec<Vec<u8>>,
    pub left_over: Vec<u8>,
}

impl ADSBBeastFrames {
    pub fn len(&self) -> usize {
        self.frames.len()
    }
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum FrameType {
    Short,
    Long,
    ModeAC,
    None,
}

/// Helper function to format ADSB Beast frames from bytes.
/// Expected input is a &Vec<Vec<u8>>of the beast frame(s), including the control characters to start and end the frame.
/// Does not consume the input.
/// Returns a vector of bytes, with each element of the array being a frame that can be passed in to the ADSB Beast parser.

pub fn format_adsb_beast_frames_from_bytes(bytes: &[u8]) -> ADSBBeastFrames {
    let mut formatted_frames: Vec<Vec<u8>> = Vec::new();
    let mut leftbytes: Vec<u8> = Vec::new();
    let mut frame_type: FrameType = FrameType::None;
    let mut frame_bytes: Vec<u8> = Vec::new();

    // https://github.com/junzis/pyModeS/blob/77273153cba6c2f282f672ea4078a62efcf716d7/pyModeS/extra/tcpclient.py#L65
    // example logic for iterating over this buffer

    let mut byte_iter = bytes.iter().peekable();

    while let Some(byte) = byte_iter.next() {
        let next_byte = byte_iter.peek();

        // if this is the start of a new frame, lets process the old one
        if *byte == ADSB_BEAST_START_CHARACTER && next_byte != Some(&&ADSB_BEAST_START_CHARACTER) {
            // if we have a frame, process it
            if !frame_bytes.is_empty() {
                // verify we have a valid frame length

                match frame_type {
                    FrameType::Short => {
                        if frame_bytes.len() != ADSB_BEAST_SHORT_FRAME_LENGTH {
                            error!(
                                "Frame is not the correct length. Expected {} got {}\n{:X?}",
                                ADSB_BEAST_SHORT_FRAME_LENGTH,
                                frame_bytes.len(),
                                frame_bytes
                            );
                            frame_bytes.clear();
                        }
                    }
                    FrameType::Long => {
                        if frame_bytes.len() != ADSB_BEAST_LONG_FRAME_LENGTH {
                            error!(
                                "Frame is not the correct length. Expected {} got {}\n{:X?}",
                                ADSB_BEAST_LONG_FRAME_LENGTH,
                                frame_bytes.len(),
                                frame_bytes
                            );
                            frame_bytes.clear();
                        }
                    }
                    FrameType::None => {
                        frame_bytes.clear();
                    }
                    FrameType::ModeAC => {
                        // Ignore the modeac frame
                        frame_bytes.clear();
                    }
                }

                // we have a valid frame, so lets add it to the list
                if !frame_bytes.is_empty() {
                    formatted_frames.push(frame_bytes.clone());
                    frame_bytes.clear();
                }
            }

            // determine the frame type
            match next_byte {
                Some(&&ADSB_BEAST_SHORT_FRAME_START_CHARACTER) => {
                    frame_type = FrameType::Short;
                    continue;
                }
                Some(&&ADSB_BEAST_LONG_FRAME_START_CHARACTER) => {
                    frame_type = FrameType::Long;
                    continue;
                }
                Some(&&ADSB_BEAST_MODEAC_FRAME_START_CHARACTER) => {
                    frame_type = FrameType::ModeAC;
                    continue;
                }
                _ => {
                    error!("Found a start character that wasn't a start sequence");
                    frame_type = FrameType::None;
                    continue;
                }
            }
        }
        // if we have a valid frame type, we should continue, otherwise, we continue
        match frame_type {
            FrameType::None => {
                error!("Frame type is None");
                continue;
            }
            _ => match *byte {
                ADSB_BEAST_START_CHARACTER => {
                    if next_byte == Some(&&ADSB_BEAST_START_CHARACTER) {
                        frame_bytes.push(*byte);
                        _ = byte_iter.next();
                        continue;
                    } else {
                        error!("Found a start character that wasn't a start sequence, or a double escape");
                        frame_type = FrameType::None;
                        continue;
                    }
                }
                _ => {
                    frame_bytes.push(*byte);
                }
            },
        }
    }

    // see if frame_bytes contains a valid frame
    if !frame_bytes.is_empty() {
        // verify we have a valid frame length
        match frame_type {
            FrameType::Short => {
                if frame_bytes.len() == ADSB_BEAST_SHORT_FRAME_LENGTH {
                    formatted_frames.push(frame_bytes.clone());
                    frame_bytes.clear();
                }
            }
            FrameType::Long => {
                if frame_bytes.len() == ADSB_BEAST_LONG_FRAME_LENGTH {
                    formatted_frames.push(frame_bytes.clone());
                    frame_bytes.clear();
                }
            }
            FrameType::None => (),
            FrameType::ModeAC => {
                // Ignore the modeac frame
                frame_bytes.clear();
            }
        }
    }

    if !frame_bytes.is_empty() {
        // we trimmed off the control characters, so we need to add them back if the frame starts with a start character
        if frame_bytes[0] == ADSB_BEAST_MODEAC_FRAME_START_CHARACTER
            || frame_bytes[0] == ADSB_BEAST_SHORT_FRAME_START_CHARACTER
            || frame_bytes[0] == ADSB_BEAST_LONG_FRAME_START_CHARACTER
        {
            frame_bytes.insert(0, ADSB_BEAST_START_CHARACTER);
        }

        match frame_bytes[0] {
            ADSB_BEAST_SHORT_FRAME_START_CHARACTER
            | ADSB_BEAST_LONG_FRAME_START_CHARACTER
            | ADSB_BEAST_MODEAC_FRAME_START_CHARACTER => {
                frame_bytes.insert(0, ADSB_BEAST_START_CHARACTER);
            }
            _ => (),
        }
        leftbytes = frame_bytes;
    }

    ADSBBeastFrames {
        frames: formatted_frames,
        left_over: leftbytes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adsb_beast_parsing_input() {
        // there are 33 frames in this input. 5 of the are MODEAC frames, which we don't want to decode
        let raw_frames = [
            0x1a as u8, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0x61, 0x57, 0x19,
            0x2, 0xe1, 0x94, 0x10, 0xfa, 0xf5, 0x48, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x68, 0x8c, 0x89,
            0x12, 0x8d, 0xa7, 0xc3, 0x36, 0xea, 0x54, 0x18, 0x60, 0x1, 0x5f, 0x48, 0x8a, 0x91,
            0xd7, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0xe7, 0xd0, 0x2a, 0x2, 0xe1, 0x93, 0x38, 0xd4,
            0x59, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0x9, 0xe9, 0x40, 0x2, 0x0, 0x5, 0xbc,
            0xcf, 0x8d, 0x9a, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x69, 0x70, 0x57, 0x18, 0x8d, 0xac,
            0xb1, 0x8a, 0x58, 0xbf, 0x3, 0x94, 0xf5, 0x24, 0xc7, 0x72, 0x74, 0xe5, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x69, 0x96, 0x6e, 0x20, 0x5d, 0xa7, 0xad, 0x84, 0x6c, 0x8f, 0xa, 0x1a,
            0x33, 0x0, 0x3e, 0x95, 0x69, 0xbd, 0x69, 0x18, 0x8d, 0xa4, 0x6f, 0x75, 0x99, 0x15,
            0x3a, 0x93, 0xb0, 0x8, 0x6, 0x79, 0xec, 0x70, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0xe4,
            0x11, 0x10, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6a, 0x23, 0x7e, 0x3d, 0x20, 0x0, 0x17, 0x18, 0xe7, 0x9e, 0x68, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6a, 0x42, 0x94, 0xf, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x37, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x6a, 0x46, 0xc7, 0x11, 0x2, 0xe1, 0x90, 0xb8, 0xc0, 0x5c, 0x65, 0x1a,
            0x32, 0x0, 0x3e, 0x95, 0x6a, 0x66, 0x1, 0x30, 0x2, 0x0, 0x4, 0xbf, 0x36, 0x2c, 0xc8,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x6, 0x43, 0xd, 0x8d, 0xa4, 0xfa, 0x78, 0xea, 0x4c,
            0x48, 0x5c, 0xed, 0x5c, 0x8, 0x61, 0x9f, 0x73, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x12,
            0x7e, 0xd4, 0x8d, 0xa0, 0x62, 0xef, 0x99, 0x9, 0xf1, 0x1a, 0x1a, 0x90, 0x4, 0x11, 0x3d,
            0xb8, 0x17, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x2a, 0x3d, 0x16, 0x8d, 0xa7, 0xd6,
            0xa8, 0x99, 0x15, 0x86, 0x86, 0xb8, 0x4, 0x13, 0x71, 0xd2, 0x9, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6b, 0x55, 0x75, 0x3e, 0x5d, 0xa1, 0x25, 0x95, 0x36, 0x1e, 0x78, 0x1a, 0x33,
            0x0, 0x3e, 0x95, 0x6b, 0x80, 0x8b, 0xd, 0x8f, 0xa6, 0x27, 0x4a, 0xe1, 0x10, 0x98, 0x0,
            0x0, 0x0, 0x0, 0x94, 0xb2, 0x49, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6b, 0xd1, 0x12, 0x15,
            0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6c, 0x4a, 0xd9,
            0x10, 0x8d, 0xac, 0x15, 0x47, 0x58, 0xb9, 0x80, 0xb8, 0x9b, 0x86, 0xae, 0xaf, 0xaa,
            0xd, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6d, 0x5, 0x46, 0xe, 0x2, 0xe1, 0x9c, 0xb0, 0x8a,
            0xf6, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xc, 0xce, 0x43, 0x5d, 0xa1, 0x25, 0x95,
            0x36, 0x1e, 0x78, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0x8d, 0x2c, 0xf, 0x5d, 0xa5, 0x59,
            0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xa0, 0xda, 0x33, 0x2, 0xe1,
            0x97, 0x18, 0xe0, 0xfd, 0xc8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xa9, 0xac, 0xa2,
            0x8d, 0xa4, 0x45, 0x85, 0x99, 0x9, 0xdd, 0x1a, 0x1a, 0xf8, 0x4, 0xe, 0xde, 0x3, 0x2,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xb6, 0xaa, 0x1d, 0x8d, 0xa7, 0xe2, 0xda, 0x99, 0xd,
            0x86, 0x7, 0x0, 0x4, 0x16, 0xca, 0x83, 0x69, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xcf,
            0xb8, 0x20, 0x0, 0x0, 0x12, 0x0, 0x5e, 0x95, 0x74, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e,
            0xf2, 0xf, 0x15, 0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6f, 0x49, 0x85, 0x41, 0x2, 0x5, 0x85, 0xbc, 0xc, 0x18, 0xfb, 0x1a, 0x33, 0x0, 0x3e,
            0x95, 0x6f, 0xae, 0x75, 0x25, 0x8d, 0xa1, 0x78, 0xde, 0x99, 0xd, 0x40, 0x9f, 0x10, 0x8,
            0xe, 0x84, 0x5, 0xe8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x5, 0xcc, 0x55, 0x8d, 0xad,
            0xd3, 0x76, 0x58, 0xab, 0x17, 0x7c, 0xab, 0xea, 0x6d, 0x7a, 0x9f, 0x5f, 0x1a, 0x32,
            0x0, 0x3e, 0x95, 0x70, 0x48, 0x90, 0x11, 0x5d, 0xa4, 0x41, 0x87, 0x76, 0x9d, 0xe4,
            0x1a, 0x32, 0x0, 0x3e, 0x95, 0x70, 0x82, 0x77, 0x2e, 0x2, 0xe1, 0x95, 0x31, 0xb, 0xa2,
            0x6d, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x9f, 0x31, 0x62, 0x8d, 0xa5, 0x5, 0xfe, 0xe1,
            0xa, 0x4, 0x0, 0x0, 0x0, 0x0, 0x11, 0xac, 0x6c, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70,
            0xae, 0x9c, 0x5a, 0x8d, 0xc0, 0x1e, 0x11, 0x99, 0x10, 0x5a, 0xb7, 0xf8, 0x4, 0x10,
            0x36, 0xf7, 0x67, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0x20, 0x67, 0x2b, 0x8d, 0xad,
            0x15, 0x60, 0x99, 0xa, 0xd, 0x4, 0x18, 0x4, 0xa, 0xe4, 0xfc, 0x20, 0x1a, 0x33, 0x0,
            0x3e, 0x95, 0x71, 0xdd, 0xb, 0xf, 0x8d, 0xc0, 0x1, 0xed, 0x99, 0x15, 0x40, 0x2, 0x90,
            0x4, 0x1f, 0x7d, 0xce, 0xe9, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x71, 0xe6, 0x1c, 0x22, 0x2,
            0xe1, 0x92, 0x19, 0x21, 0xcd, 0x85, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0xfa, 0x9a,
            0x3c, 0x8d, 0xa1, 0x25, 0x95, 0xe1, 0xe, 0x4, 0x0, 0x0, 0x0, 0x0, 0x86, 0xce, 0x8e,
        ];

        let frames = format_adsb_beast_frames_from_bytes(&raw_frames);
        for frame in frames.frames.iter() {
            println!("Frame: {:x?}", frame);
        }
        assert!(
            frames.frames.len() == 38,
            "Expected 38 frames, got {}",
            frames.frames.len()
        );

        // validate each frame is the correct length
        for frame in frames.frames.iter() {
            assert!(
                frame.len() == ADSB_BEAST_SHORT_FRAME_LENGTH
                    || frame.len() == ADSB_BEAST_LONG_FRAME_LENGTH,
                "Frame is not the correct length: {}",
                frame.len()
            );
        }

        // validate the leftover bytes are correct
        assert!(
            frames.left_over.len() == 0,
            "Expected 0 leftover bytes, got {}",
            frames.left_over.len()
        );
    }

    #[test]
    fn test_extra_bytes_in_input() {
        // there are 33 frames in this input. 5 of the are MODEAC frames, which we don't want to decode
        let raw_frames = [
            0x1a as u8, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0x61, 0x57, 0x19,
            0x2, 0xe1, 0x94, 0x10, 0xfa, 0xf5, 0x48, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x68, 0x8c, 0x89,
            0x12, 0x8d, 0xa7, 0xc3, 0x36, 0xea, 0x54, 0x18, 0x60, 0x1, 0x5f, 0x48, 0x8a, 0x91,
            0xd7, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0xe7, 0xd0, 0x2a, 0x2, 0xe1, 0x93, 0x38, 0xd4,
            0x59, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0x9, 0xe9, 0x40, 0x2, 0x0, 0x5, 0xbc,
            0xcf, 0x8d, 0x9a, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x69, 0x70, 0x57, 0x18, 0x8d, 0xac,
            0xb1, 0x8a, 0x58, 0xbf, 0x3, 0x94, 0xf5, 0x24, 0xc7, 0x72, 0x74, 0xe5, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x69, 0x96, 0x6e, 0x20, 0x5d, 0xa7, 0xad, 0x84, 0x6c, 0x8f, 0xa, 0x1a,
            0x33, 0x0, 0x3e, 0x95, 0x69, 0xbd, 0x69, 0x18, 0x8d, 0xa4, 0x6f, 0x75, 0x99, 0x15,
            0x3a, 0x93, 0xb0, 0x8, 0x6, 0x79, 0xec, 0x70, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0xe4,
            0x11, 0x10, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6a, 0x23, 0x7e, 0x3d, 0x20, 0x0, 0x17, 0x18, 0xe7, 0x9e, 0x68, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6a, 0x42, 0x94, 0xf, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x37, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x6a, 0x46, 0xc7, 0x11, 0x2, 0xe1, 0x90, 0xb8, 0xc0, 0x5c, 0x65, 0x1a,
            0x32, 0x0, 0x3e, 0x95, 0x6a, 0x66, 0x1, 0x30, 0x2, 0x0, 0x4, 0xbf, 0x36, 0x2c, 0xc8,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x6, 0x43, 0xd, 0x8d, 0xa4, 0xfa, 0x78, 0xea, 0x4c,
            0x48, 0x5c, 0xed, 0x5c, 0x8, 0x61, 0x9f, 0x73, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x12,
            0x7e, 0xd4, 0x8d, 0xa0, 0x62, 0xef, 0x99, 0x9, 0xf1, 0x1a, 0x1a, 0x90, 0x4, 0x11, 0x3d,
            0xb8, 0x17, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x2a, 0x3d, 0x16, 0x8d, 0xa7, 0xd6,
            0xa8, 0x99, 0x15, 0x86, 0x86, 0xb8, 0x4, 0x13, 0x71, 0xd2, 0x9, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6b, 0x55, 0x75, 0x3e, 0x5d, 0xa1, 0x25, 0x95, 0x36, 0x1e, 0x78, 0x1a, 0x33,
            0x0, 0x3e, 0x95, 0x6b, 0x80, 0x8b, 0xd, 0x8f, 0xa6, 0x27, 0x4a, 0xe1, 0x10, 0x98, 0x0,
            0x0, 0x0, 0x0, 0x94, 0xb2, 0x49, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6b, 0xd1, 0x12, 0x15,
            0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6c, 0x4a, 0xd9,
            0x10, 0x8d, 0xac, 0x15, 0x47, 0x58, 0xb9, 0x80, 0xb8, 0x9b, 0x86, 0xae, 0xaf, 0xaa,
            0xd, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6d, 0x5, 0x46, 0xe, 0x2, 0xe1, 0x9c, 0xb0, 0x8a,
            0xf6, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xc, 0xce, 0x43, 0x5d, 0xa1, 0x25, 0x95,
            0x36, 0x1e, 0x78, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0x8d, 0x2c, 0xf, 0x5d, 0xa5, 0x59,
            0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xa0, 0xda, 0x33, 0x2, 0xe1,
            0x97, 0x18, 0xe0, 0xfd, 0xc8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xa9, 0xac, 0xa2,
            0x8d, 0xa4, 0x45, 0x85, 0x99, 0x9, 0xdd, 0x1a, 0x1a, 0xf8, 0x4, 0xe, 0xde, 0x3, 0x2,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xb6, 0xaa, 0x1d, 0x8d, 0xa7, 0xe2, 0xda, 0x99, 0xd,
            0x86, 0x7, 0x0, 0x4, 0x16, 0xca, 0x83, 0x69, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xcf,
            0xb8, 0x20, 0x0, 0x0, 0x12, 0x0, 0x5e, 0x95, 0x74, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e,
            0xf2, 0xf, 0x15, 0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6f, 0x49, 0x85, 0x41, 0x2, 0x5, 0x85, 0xbc, 0xc, 0x18, 0xfb, 0x1a, 0x33, 0x0, 0x3e,
            0x95, 0x6f, 0xae, 0x75, 0x25, 0x8d, 0xa1, 0x78, 0xde, 0x99, 0xd, 0x40, 0x9f, 0x10, 0x8,
            0xe, 0x84, 0x5, 0xe8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x5, 0xcc, 0x55, 0x8d, 0xad,
            0xd3, 0x76, 0x58, 0xab, 0x17, 0x7c, 0xab, 0xea, 0x6d, 0x7a, 0x9f, 0x5f, 0x1a, 0x32,
            0x0, 0x3e, 0x95, 0x70, 0x48, 0x90, 0x11, 0x5d, 0xa4, 0x41, 0x87, 0x76, 0x9d, 0xe4,
            0x1a, 0x32, 0x0, 0x3e, 0x95, 0x70, 0x82, 0x77, 0x2e, 0x2, 0xe1, 0x95, 0x31, 0xb, 0xa2,
            0x6d, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x9f, 0x31, 0x62, 0x8d, 0xa5, 0x5, 0xfe, 0xe1,
            0xa, 0x4, 0x0, 0x0, 0x0, 0x0, 0x11, 0xac, 0x6c, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70,
            0xae, 0x9c, 0x5a, 0x8d, 0xc0, 0x1e, 0x11, 0x99, 0x10, 0x5a, 0xb7, 0xf8, 0x4, 0x10,
            0x36, 0xf7, 0x67, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0x20, 0x67, 0x2b, 0x8d, 0xad,
            0x15, 0x60, 0x99, 0xa, 0xd, 0x4, 0x18, 0x4, 0xa, 0xe4, 0xfc, 0x20, 0x1a, 0x33, 0x0,
            0x3e, 0x95, 0x71, 0xdd, 0xb, 0xf, 0x8d, 0xc0, 0x1, 0xed, 0x99, 0x15, 0x40, 0x2, 0x90,
            0x4, 0x1f, 0x7d, 0xce, 0xe9, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x71, 0xe6, 0x1c, 0x22, 0x2,
            0xe1, 0x92, 0x19, 0x21, 0xcd, 0x85, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0xfa, 0x9a,
            0x3c, 0x8d, 0xa1, 0x25, 0x95, 0xe1, 0xe, 0x4, 0x0, 0x0, 0x0, 0x0, 0x86, 0xce, 0x8e,
        ];

        let frames = format_adsb_beast_frames_from_bytes(&raw_frames);
        for frame in frames.frames.iter() {
            println!("Frame: {:x?}", frame);
        }
        assert!(
            frames.frames.len() == 38,
            "Expected 38 frames, got {}",
            frames.frames.len()
        );

        // validate each frame is the correct length
        for frame in frames.frames.iter() {
            assert!(
                frame.len() == ADSB_BEAST_SHORT_FRAME_LENGTH
                    || frame.len() == ADSB_BEAST_LONG_FRAME_LENGTH,
                "Frame is not the correct length: {}",
                frame.len()
            );
        }

        // validate the leftover bytes are correct
        assert!(
            frames.left_over.len() == 0,
            "Expected 0 leftover bytes, got {}",
            frames.left_over.len()
        );
    }

    #[test]
    fn test_extra_bytes_at_end_input() {
        // there are 33 frames in this input. 5 of the are MODEAC frames, which we don't want to decode
        let raw_frames = [
            0x1a as u8, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x1a, 0x31, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x31, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0x61, 0x57, 0x19,
            0x2, 0xe1, 0x94, 0x10, 0xfa, 0xf5, 0x48, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x68, 0x8c, 0x89,
            0x12, 0x8d, 0xa7, 0xc3, 0x36, 0xea, 0x54, 0x18, 0x60, 0x1, 0x5f, 0x48, 0x8a, 0x91,
            0xd7, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x68, 0xe7, 0xd0, 0x2a, 0x2, 0xe1, 0x93, 0x38, 0xd4,
            0x59, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0x9, 0xe9, 0x40, 0x2, 0x0, 0x5, 0xbc,
            0xcf, 0x8d, 0x9a, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x69, 0x70, 0x57, 0x18, 0x8d, 0xac,
            0xb1, 0x8a, 0x58, 0xbf, 0x3, 0x94, 0xf5, 0x24, 0xc7, 0x72, 0x74, 0xe5, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x69, 0x96, 0x6e, 0x20, 0x5d, 0xa7, 0xad, 0x84, 0x6c, 0x8f, 0xa, 0x1a,
            0x33, 0x0, 0x3e, 0x95, 0x69, 0xbd, 0x69, 0x18, 0x8d, 0xa4, 0x6f, 0x75, 0x99, 0x15,
            0x3a, 0x93, 0xb0, 0x8, 0x6, 0x79, 0xec, 0x70, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x69, 0xe4,
            0x11, 0x10, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6a, 0x23, 0x7e, 0x3d, 0x20, 0x0, 0x17, 0x18, 0xe7, 0x9e, 0x68, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6a, 0x42, 0x94, 0xf, 0x5d, 0xa5, 0x59, 0x7b, 0xee, 0x11, 0x37, 0x1a, 0x32, 0x0,
            0x3e, 0x95, 0x6a, 0x46, 0xc7, 0x11, 0x2, 0xe1, 0x90, 0xb8, 0xc0, 0x5c, 0x65, 0x1a,
            0x32, 0x0, 0x3e, 0x95, 0x6a, 0x66, 0x1, 0x30, 0x2, 0x0, 0x4, 0xbf, 0x36, 0x2c, 0xc8,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x6, 0x43, 0xd, 0x8d, 0xa4, 0xfa, 0x78, 0xea, 0x4c,
            0x48, 0x5c, 0xed, 0x5c, 0x8, 0x61, 0x9f, 0x73, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x12,
            0x7e, 0xd4, 0x8d, 0xa0, 0x62, 0xef, 0x99, 0x9, 0xf1, 0x1a, 0x1a, 0x90, 0x4, 0x11, 0x3d,
            0xb8, 0x17, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6b, 0x2a, 0x3d, 0x16, 0x8d, 0xa7, 0xd6,
            0xa8, 0x99, 0x15, 0x86, 0x86, 0xb8, 0x4, 0x13, 0x71, 0xd2, 0x9, 0x1a, 0x32, 0x0, 0x3e,
            0x95, 0x6b, 0x55, 0x75, 0x3e, 0x5d, 0xa1, 0x25, 0x95, 0x36, 0x1e, 0x78, 0x1a, 0x33,
            0x0, 0x3e, 0x95, 0x6b, 0x80, 0x8b, 0xd, 0x8f, 0xa6, 0x27, 0x4a, 0xe1, 0x10, 0x98, 0x0,
            0x0, 0x0, 0x0, 0x94, 0xb2, 0x49, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6b, 0xd1, 0x12, 0x15,
            0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6c, 0x4a, 0xd9,
            0x10, 0x8d, 0xac, 0x15, 0x47, 0x58, 0xb9, 0x80, 0xb8, 0x9b, 0x86, 0xae, 0xaf, 0xaa,
            0xd, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6d, 0x5, 0x46, 0xe, 0x2, 0xe1, 0x9c, 0xb0, 0x8a,
            0xf6, 0x4f, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xc, 0xce, 0x43, 0x5d, 0xa1, 0x25, 0x95,
            0x36, 0x1e, 0x78, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0x8d, 0x2c, 0xf, 0x5d, 0xa5, 0x59,
            0x7b, 0xee, 0x11, 0x36, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xa0, 0xda, 0x33, 0x2, 0xe1,
            0x97, 0x18, 0xe0, 0xfd, 0xc8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xa9, 0xac, 0xa2,
            0x8d, 0xa4, 0x45, 0x85, 0x99, 0x9, 0xdd, 0x1a, 0x1a, 0xf8, 0x4, 0xe, 0xde, 0x3, 0x2,
            0x1a, 0x33, 0x0, 0x3e, 0x95, 0x6e, 0xb6, 0xaa, 0x1d, 0x8d, 0xa7, 0xe2, 0xda, 0x99, 0xd,
            0x86, 0x7, 0x0, 0x4, 0x16, 0xca, 0x83, 0x69, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e, 0xcf,
            0xb8, 0x20, 0x0, 0x0, 0x12, 0x0, 0x5e, 0x95, 0x74, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x6e,
            0xf2, 0xf, 0x15, 0x2, 0x0, 0x4, 0x97, 0x3e, 0x1f, 0xe6, 0x1a, 0x32, 0x0, 0x3e, 0x95,
            0x6f, 0x49, 0x85, 0x41, 0x2, 0x5, 0x85, 0xbc, 0xc, 0x18, 0xfb, 0x1a, 0x33, 0x0, 0x3e,
            0x95, 0x6f, 0xae, 0x75, 0x25, 0x8d, 0xa1, 0x78, 0xde, 0x99, 0xd, 0x40, 0x9f, 0x10, 0x8,
            0xe, 0x84, 0x5, 0xe8, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x5, 0xcc, 0x55, 0x8d, 0xad,
            0xd3, 0x76, 0x58, 0xab, 0x17, 0x7c, 0xab, 0xea, 0x6d, 0x7a, 0x9f, 0x5f, 0x1a, 0x32,
            0x0, 0x3e, 0x95, 0x70, 0x48, 0x90, 0x11, 0x5d, 0xa4, 0x41, 0x87, 0x76, 0x9d, 0xe4,
            0x1a, 0x32, 0x0, 0x3e, 0x95, 0x70, 0x82, 0x77, 0x2e, 0x2, 0xe1, 0x95, 0x31, 0xb, 0xa2,
            0x6d, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70, 0x9f, 0x31, 0x62, 0x8d, 0xa5, 0x5, 0xfe, 0xe1,
            0xa, 0x4, 0x0, 0x0, 0x0, 0x0, 0x11, 0xac, 0x6c, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x70,
            0xae, 0x9c, 0x5a, 0x8d, 0xc0, 0x1e, 0x11, 0x99, 0x10, 0x5a, 0xb7, 0xf8, 0x4, 0x10,
            0x36, 0xf7, 0x67, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0x20, 0x67, 0x2b, 0x8d, 0xad,
            0x15, 0x60, 0x99, 0xa, 0xd, 0x4, 0x18, 0x4, 0xa, 0xe4, 0xfc, 0x20, 0x1a, 0x33, 0x0,
            0x3e, 0x95, 0x71, 0xdd, 0xb, 0xf, 0x8d, 0xc0, 0x1, 0xed, 0x99, 0x15, 0x40, 0x2, 0x90,
            0x4, 0x1f, 0x7d, 0xce, 0xe9, 0x1a, 0x32, 0x0, 0x3e, 0x95, 0x71, 0xe6, 0x1c, 0x22, 0x2,
            0xe1, 0x92, 0x19, 0x21, 0xcd, 0x85, 0x1a, 0x33, 0x0, 0x3e, 0x95, 0x71, 0xfa, 0x9a,
            0x3c, 0x8d, 0xa1, 0x25, 0x95, 0xe1, 0xe, 0x4, 0x0, 0x0, 0x0, 0x0, 0x86, 0xce, 0x8e,
            0x1a, 0x33,
        ];

        let frames = format_adsb_beast_frames_from_bytes(&raw_frames);
        for frame in frames.frames.iter() {
            println!("Frame: {:x?}", frame);
        }
        assert!(
            frames.frames.len() == 38,
            "Expected 38 frames, got {}",
            frames.frames.len()
        );

        // validate each frame is the correct length
        for frame in frames.frames.iter() {
            assert!(
                frame.len() == ADSB_BEAST_SHORT_FRAME_LENGTH
                    || frame.len() == ADSB_BEAST_LONG_FRAME_LENGTH,
                "Frame is not the correct length: {}",
                frame.len()
            );
        }

        // validate the leftover bytes are correct
        assert!(
            frames.left_over.len() == 2,
            "Expected 2 leftover bytes, got {}",
            frames.left_over.len()
        );
    }
}
