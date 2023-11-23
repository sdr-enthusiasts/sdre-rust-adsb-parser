use crate::error_handling::adsb_raw_error::ADSBRawError;
use crate::error_handling::deserialization_error::DeserializationError;
use hex;
const ADSB_RAW_START_CHARACTER: u8 = 0x2a; // The adsb raw end character sequence is is a '0x3b0a', start is '0x2a'
const ADSB_RAW_END_SEQUENCE_FINISH_CHARACTER: u8 = 0x3b;
const ADSB_RAW_END_SEQUENCE_INIT_CHARACTER: u8 = 0x0a;
const ADSB_RAW_FRAME_SMALL: usize = 14;
const ADSB_RAW_FRAME_LARGE: usize = 28;
const ADSB_RAW_MODEAC_FRAME: usize = 4;

/// Helper function to format ADSB Raw frames from a single line of String.
/// Expected input is a &str slice.
/// Does not consume the input.
/// Returns a Result<String, ADSBRawError> with control characters split from the input.

pub fn format_adsb_raw_frame_from_str(line: &str) -> Result<String, ADSBRawError> {
    // remove * from the start of the line, and ; \n from the end and return
    let formatted_line = line
        .trim_start_matches('*')
        .trim_end_matches(|c| c == ';' || c == '\n')
        .to_string();

    if formatted_line.len() == ADSB_RAW_FRAME_LARGE || formatted_line.len() == ADSB_RAW_FRAME_SMALL
    {
        return Ok(formatted_line);
    }

    Err(ADSBRawError::ByteSequenceWrong {
        size: formatted_line.len() as u8,
    })
}

/// Helper function to format ADSB Raw frames from a &Vec<String>.
/// Expected input is &Vec<String>.
/// Does not consume the input.
/// Returns a new Vec<String> with control characters split from the input.

pub fn format_adsb_raw_frames_from_vec_string(frames: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for line in frames {
        if !line.is_empty() {
            if let Ok(formatted_line) = format_adsb_raw_frame_from_str(line) {
                output.push(formatted_line);
            }
        }
    }

    output
}

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
    let mut entry = 0;

    for (position, byte) in bytes.iter().enumerate() {
        if byte == &ADSB_RAW_END_SEQUENCE_INIT_CHARACTER && entry != 0 {
            match current_frame.len() {
                ADSB_RAW_MODEAC_FRAME => {
                    // this is a valid frame size, but it's NOT one we want to decode
                    debug!("Detected a MODEAC frame, skipping");
                }
                // The frame size is valid
                ADSB_RAW_FRAME_SMALL | ADSB_RAW_FRAME_LARGE => {
                    // FIXME: there is some kind of stupid read-in issue with the data where I need to do this round-robin
                    // nonsense to convert the data to a string and then back in to a vector of u8s
                    // Maybe I should chunk the input?
                    if let Ok(frame_string) = String::from_utf8(current_frame.clone()) {
                        debug!("Valid frame: {}", frame_string);
                        if let Ok(frame_bytes) = hex::decode(frame_string) {
                            formatted_frames.push(frame_bytes);
                            current_frame = Vec::new();
                        } else {
                            errors_found.push(DeserializationError::ADSBRawError(
                                ADSBRawError::StringError {
                                    message: "Could not convert the {frame_string} string to bytes"
                                        .to_string(),
                                },
                            ));
                        }
                    } else {
                        errors_found.push(DeserializationError::ADSBRawError(
                            ADSBRawError::StringError {
                                message: "Could not convert the bytes {current_frame} to a string"
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
            entry += 1;
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
                    if let Ok(frame_string) = String::from_utf8(current_frame.clone()) {
                        if let Ok(frame_bytes) = hex::decode(frame_string) {
                            formatted_frames.push(frame_bytes);
                        } else {
                            errors_found.push(DeserializationError::ADSBRawError(
                                ADSBRawError::StringError {
                                    message: "Could not convert the {frame_string} string to bytes"
                                        .to_string(),
                                },
                            ));
                        }
                        current_frame = Vec::new();
                    } else {
                        errors_found.push(DeserializationError::ADSBRawError(
                            ADSBRawError::StringError {
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

#[test]
fn test_adsb_raw_parsing_from_str() {
    let line_one = "*5DABE65A2FBFAF;\n";
    let line_two = "*8DA1A3CC9909B814F004127F1107;\n";
    let vec_of_lines = vec![line_one.to_string(), line_two.to_string()];

    assert_eq!(
        format_adsb_raw_frame_from_str(line_one).unwrap(),
        "5DABE65A2FBFAF"
    );
    assert_eq!(
        format_adsb_raw_frame_from_str(line_two).unwrap(),
        "8DA1A3CC9909B814F004127F1107"
    );

    assert_eq!(
        format_adsb_raw_frames_from_vec_string(&vec_of_lines),
        ["5DABE65A2FBFAF", "8DA1A3CC9909B814F004127F1107"]
    );
}

// #[test]
// fn test_adsb_raw_parsing_input() {
//     let mut input = vec![
//         0x2a, 0x35, 0x44, 0x41, 0x42, 0x45, 0x36, 0x35, 0x41, 0x32, 0x46, 0x42, 0x46, 0x41, 0x46,
//         0x3b, 0x0a, 0x2a, 0x38, 0x44, 0x41, 0x31, 0x41, 0x33, 0x43, 0x43, 0x39, 0x39, 0x30, 0x39,
//         0x42, 0x38, 0x31, 0x34, 0x46, 0x30, 0x30, 0x34, 0x31, 0x32, 0x37, 0x46, 0x31, 0x31, 0x30,
//         0x37, 0x3b, 0x0a,
//     ];

//     assert_eq!(
//         format_adsb_raw_frames_from_bytes(&input).len(),
//         2,
//         "There should be two frames in the input"
//     );
//     assert_eq!(
//         format_adsb_raw_frames_from_bytes(&input),
//         [
//             hex::decode("5DABE65A2FBFAF").unwrap(),
//             hex::decode("8DA1A3CC9909B814F004127F1107").unwrap()
//         ]
//     );

//     input.push(0x2a);
//     input.push(0x35);
//     input.push(0x34);
//     input.push(0x32);
//     input.push(0x34);
//     input.push(0x3b);
//     input.push(0x0a);
//     assert_eq!(
//         format_adsb_raw_frames_from_bytes(&input).len(),
//         2,
//         "There should be two frames in the input"
//     );
//     assert_eq!(
//         format_adsb_raw_frames_from_bytes(&input),
//         [
//             hex::decode("5DABE65A2FBFAF").unwrap(),
//             hex::decode("8DA1A3CC9909B814F004127F1107").unwrap()
//         ]
//     );
// }
