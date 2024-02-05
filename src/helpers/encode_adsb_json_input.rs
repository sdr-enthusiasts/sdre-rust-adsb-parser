// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::error_handling::adsb_json_error::ADSBJSONError;

pub struct ADSBJSONFrames {
    pub frames: Vec<String>,
    pub left_over: String,
    pub errors: Vec<ADSBJSONError>,
}

impl ADSBJSONFrames {
    #[must_use]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}

/// Helper function to format ADSB JSON frames from a string.
/// Expected input is a &String of the JSON frame(s), including the control characters to start and end the frame.
/// Does not consume the input.
/// Returns a vector of strings, with each element of the array being a frame that can be passed in to the ADSB JSON parser.
#[must_use]
pub fn format_adsb_json_frames_from_string(string: &str) -> ADSBJSONFrames {
    // Split the string into a vector of strings, delimited by '\n' with each element being a frame.
    let frames: Vec<&str> = string.split('\n').collect();
    let mut output: Vec<String> = Vec::new();
    let mut errors: Vec<ADSBJSONError> = Vec::new();

    for (index, frame) in frames.iter().enumerate() {
        let frame = frame.trim(); // remove the trailing '\n' from the frame
                                  // If the frame is empty, skip it.
        if frame.is_empty() {
            continue;
        }
        // Check if the frame starts with '{' and ends with '}'.
        if !frame.starts_with('{') {
            // if this is the first frame, and the only element in the vector, return the frame as the left_over.
            if index == 0 && frames.len() == 1 {
                return ADSBJSONFrames {
                    frames: output,
                    left_over: frame.to_string(),
                    errors,
                };
            }
        }

        if !frame.ends_with('}') {
            // if this is the last frame, return the frame as the left_over.
            if index == frames.len() - 1 {
                return ADSBJSONFrames {
                    frames: output,
                    left_over: frame.to_string(),
                    errors,
                };
            }
        }

        // If the frame starts with '{' and ends with '}', push it to the output vector.
        if frame.starts_with('{') && frame.ends_with('}') {
            output.push(frame.to_string());
        } else {
            // we should never end up here but if we do, error out
            errors.push(ADSBJSONError::InvalidJSON {
                message: "Frame does not start with '{' and end with '}'".to_string(),
            });
        }
    }

    ADSBJSONFrames {
        frames: output,
        left_over: String::new(),
        errors,
    }
}

/// Helper function to format ADSB JSON frames from bytes.
/// Expected input is a &Vec<Vec<u8>>of the JSON frame(s), including the control characters to start and end the frame.
/// Does not consume the input.
/// Returns a vector of strings, with each element of the array being a frame that can be passed in to the ADSB JSON parser.

#[must_use]
pub fn format_adsb_json_frames_from_bytes(bytes: &[u8]) -> ADSBJSONFrames {
    format_adsb_json_frames_from_string(String::from_utf8_lossy(bytes).as_ref())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_adsb_json_parsing_input_single_frame() {
        let input = "{\"now\" : 1701103343.740,\"hex\":\"a40d4c\",\"type\":\"adsb_icao\",\"flight\":\"N360LF  \",\"r\":\"N360LF\",\"t\":\"GLF5\",\"dbFlags\":8,\"alt_baro\":45000,\"alt_geom\":45450,\"gs\":521.1,\"track\":68.85,\"baro_rate\":-64,\"squawk\":\"1416\",\"emergency\":\"none\",\"category\":\"A3\",\"nav_qnh\":1013.6,\"nav_altitude_mcp\":45024,\"nav_modes\":[\"autopilot\",\"althold\",\"tcas\"],\"lat\":37.491031,\"lon\":-107.526358,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":145.808,\"r_dir\":341.8,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":2,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":2,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":2657,\"seen\":0.0,\"rssi\":-19.0}\n";
        let output = format_adsb_json_frames_from_string(input);

        assert_eq!(
            output.frames.len(),
            1,
            "Expected 1 frame, got {}",
            output.frames.len()
        );
        assert_eq!(
            output.left_over, "",
            "Expected empty string, got {}",
            output.left_over
        );
    }

    #[test]
    fn test_adsb_json_parsing_input_multiple_frames() {
        let input = "{\"now\" : 1701103373.918,\"hex\":\"ac07dc\",\"type\":\"adsb_icao\",\"flight\":\"SWA3225 \",\"r\":\"N8742M\",\"t\":\"B38M\",\"alt_baro\":38000,\"alt_geom\":38525,\"gs\":391.7,\"track\":282.08,\"baro_rate\":64,\"squawk\":\"2564\",\"emergency\":\"none\",\"category\":\"A3\",\"nav_qnh\":1013.6,\"nav_altitude_mcp\":38016,\"nav_heading\":267.89,\"lat\":34.532543,\"lon\":-106.578712,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":39.361,\"r_dir\":180.7,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":2,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":2,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":3928,\"seen\":0.0,\"rssi\":-8.6}\n{\"now\" : 1701103373.933,\"hex\":\"a17bfe\",\"type\":\"adsb_icao\",\"flight\":\"AAY3184 \",\"r\":\"N195NV\",\"t\":\"A320\",\"alt_baro\":33975,\"alt_geom\":34350,\"gs\":367.1,\"track\":237.91,\"baro_rate\":-64,\"squawk\":\"3656\",\"emergency\":\"none\",\"category\":\"A3\",\"nav_qnh\":1012.8,\"nav_altitude_mcp\":34016,\"lat\":36.411301,\"lon\":-106.413288,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":73.836,\"r_dir\":5.9,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":4,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":3,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":4348,\"seen\":0.0,\"rssi\":-11.6}\n";
        let output = format_adsb_json_frames_from_string(input);

        assert_eq!(
            output.frames.len(),
            2,
            "Expected 2 frames, got {}",
            output.frames.len()
        );
        assert_eq!(
            output.left_over, "",
            "Expected empty string, got {}",
            output.left_over
        );
    }

    #[test]
    fn test_adsb_json_parsing_input_multiple_frames_with_incomplete_frame() {
        let input = "{\"now\" : 1701103373.918,\"hex\":\"ac07dc\",\"type\":\"adsb_icao\",\"flight\":\"SWA3225 \",\"r\":\"N8742M\",\"t\":\"B38M\",\"alt_baro\":38000,\"alt_geom\":38525,\"gs\":391.7,\"track\":282.08,\"baro_rate\":64,\"squawk\":\"2564\",\"emergency\":\"none\",\"category\":\"A3\",\"nav_qnh\":1013.6,\"nav_altitude_mcp\":38016,\"nav_heading\":267.89,\"lat\":34.532543,\"lon\":-106.578712,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":39.361,\"r_dir\":180.7,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":2,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":2,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":3928,\"seen\":0.0,\"rssi\":-8.6}\n{\"now\" : 1701103373.933,\"hex\":\"a17bfe\",\"type\":\"adsb_icao\",\"flight\":\"AAY3184 \",\"r\":\"N195NV\",\"t\":\"A320\",\"alt_baro\":33975,\"alt_geom\":34350,\"gs\":367.1,\"track\":237.91,\"baro_rate\":-64,\"squawk\":\"3656\",\"emergency\":\"none\",\"category\":\"A3\",\"nav_qnh\":1012.8,\"nav_altitude_mcp\":34016,\"lat\":36.411301,\"lon\":-106.413288,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":73.836,\"r_dir\":5.9,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":4,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":3,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":4348,\"seen\":0.0,\"rssi\":-11.6}\n{\"now\" : 1701103373.934,\"hex\":\"a12c18\",\"type\":\"adsb_icao\",\"flight\":\"DAL780  \",\"r\":\"N175DN\",\"t\":\"B763\",\"alt_baro\":37000,\"alt_geom\":37425,\"gs\":527.8,\"track\":86.85,\"baro_rate\":0,\"squawk\":\"1323\",\"emergency\":\"none\",\"category\":\"A5\",\"nav_qnh\":1013.6,\"nav_altitude_mcp\":36992,\"nav_heading\":78.05,\"lat\":35.112488,\"lon\":-106.604938,\"nic\":8,\"rc\":186,\"seen_pos\":0.000,\"r_dst\":4.860,\"r_dir\":201.0,\"version\":2,\"nic_baro\":1,\"nac_p\":10,\"nac_v\":2,\"sil\":3,\"sil_type\":\"perhour\",\"gva\":2,\"sda\":2,\"alert\":0,\"spi\":0,\"mlat\":[],\"tisb\":[],\"messages\":6896,\"seen\":0.0,\"rssi\":-3.8}\n{\"now\" : 1701103373.941,\"hex\":\"ab23c2\",\"type\":\"adsb_icao\",\"flight\":\"N817EA  \",\"r\":\"N817EA\",\"t\":\"C560\",\"alt_baro\":43000,\"alt_geom\":43550,\"gs\":335.2,\"track\":274.79,\"baro_rate\":0,\"squawk\":\"7064\"";

        let output = format_adsb_json_frames_from_string(input);

        assert_eq!(
            output.frames.len(),
            3,
            "Expected 3 frames, got {}",
            output.frames.len()
        );
        assert_eq!(
            output.left_over,
            "{\"now\" : 1701103373.941,\"hex\":\"ab23c2\",\"type\":\"adsb_icao\",\"flight\":\"N817EA  \",\"r\":\"N817EA\",\"t\":\"C560\",\"alt_baro\":43000,\"alt_geom\":43550,\"gs\":335.2,\"track\":274.79,\"baro_rate\":0,\"squawk\":\"7064\"",
            "Expected incomplete frame, got {}",
            output.left_over
        );
    }

    #[test]
    fn test_adsb_json_parsing_input_single_frame_from_bytes() {
        let input = [
            0x7b_u8, 0x22, 0x6e, 0x6f, 0x77, 0x22, 0x20, 0x3a, 0x20, 0x31, 0x37, 0x30, 0x31, 0x31,
            0x30, 0x33, 0x33, 0x34, 0x33, 0x2e, 0x37, 0x34, 0x30, 0x2c, 0x22, 0x68, 0x65, 0x78,
            0x22, 0x3a, 0x22, 0x61, 0x34, 0x30, 0x64, 0x34, 0x63, 0x22, 0x2c, 0x22, 0x74, 0x79,
            0x70, 0x65, 0x22, 0x3a, 0x22, 0x61, 0x64, 0x73, 0x62, 0x5f, 0x69, 0x63, 0x61, 0x6f,
            0x22, 0x2c, 0x22, 0x66, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x22, 0x3a, 0x22, 0x4e, 0x33,
            0x36, 0x30, 0x4c, 0x46, 0x20, 0x20, 0x22, 0x2c, 0x22, 0x72, 0x22, 0x3a, 0x22, 0x4e,
            0x33, 0x36, 0x30, 0x4c, 0x46, 0x22, 0x2c, 0x22, 0x74, 0x22, 0x3a, 0x22, 0x47, 0x4c,
            0x46, 0x35, 0x22, 0x2c, 0x22, 0x64, 0x62, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x22, 0x3a,
            0x38, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22, 0x3a, 0x34,
            0x35, 0x30, 0x30, 0x30, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x67, 0x65, 0x6f, 0x6d,
            0x22, 0x3a, 0x34, 0x35, 0x34, 0x35, 0x30, 0x2c, 0x22, 0x67, 0x73, 0x22, 0x3a, 0x35,
            0x32, 0x31, 0x2e, 0x31, 0x2c, 0x22, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x3a, 0x36,
            0x38, 0x2e, 0x38, 0x35, 0x2c, 0x22, 0x62, 0x61, 0x72, 0x6f, 0x5f, 0x72, 0x61, 0x74,
            0x65, 0x22, 0x3a, 0x2d, 0x36, 0x34, 0x2c, 0x22, 0x73, 0x71, 0x75, 0x61, 0x77, 0x6b,
            0x22, 0x3a, 0x22, 0x31, 0x34, 0x31, 0x36, 0x22, 0x2c, 0x22, 0x65, 0x6d, 0x65, 0x72,
            0x67, 0x65, 0x6e, 0x63, 0x79, 0x22, 0x3a, 0x22, 0x6e, 0x6f, 0x6e, 0x65, 0x22, 0x2c,
            0x22, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x22, 0x3a, 0x22, 0x41, 0x33,
            0x22, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x71, 0x6e, 0x68, 0x22, 0x3a, 0x31, 0x30,
            0x31, 0x33, 0x2e, 0x36, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x61, 0x6c, 0x74, 0x69,
            0x74, 0x75, 0x64, 0x65, 0x5f, 0x6d, 0x63, 0x70, 0x22, 0x3a, 0x34, 0x35, 0x30, 0x32,
            0x34, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x3a,
            0x5b, 0x22, 0x61, 0x75, 0x74, 0x6f, 0x70, 0x69, 0x6c, 0x6f, 0x74, 0x22, 0x2c, 0x22,
            0x61, 0x6c, 0x74, 0x68, 0x6f, 0x6c, 0x64, 0x22, 0x2c, 0x22, 0x74, 0x63, 0x61, 0x73,
            0x22, 0x5d, 0x2c, 0x22, 0x6c, 0x61, 0x74, 0x22, 0x3a, 0x33, 0x37, 0x2e, 0x34, 0x39,
            0x31, 0x30, 0x33, 0x31, 0x2c, 0x22, 0x6c, 0x6f, 0x6e, 0x22, 0x3a, 0x2d, 0x31, 0x30,
            0x37, 0x2e, 0x35, 0x32, 0x36, 0x33, 0x35, 0x38, 0x2c, 0x22, 0x6e, 0x69, 0x63, 0x22,
            0x3a, 0x38, 0x2c, 0x22, 0x72, 0x63, 0x22, 0x3a, 0x31, 0x38, 0x36, 0x2c, 0x22, 0x73,
            0x65, 0x65, 0x6e, 0x5f, 0x70, 0x6f, 0x73, 0x22, 0x3a, 0x30, 0x2e, 0x30, 0x30, 0x30,
            0x2c, 0x22, 0x72, 0x5f, 0x64, 0x73, 0x74, 0x22, 0x3a, 0x31, 0x34, 0x35, 0x2e, 0x38,
            0x30, 0x38, 0x2c, 0x22, 0x72, 0x5f, 0x64, 0x69, 0x72, 0x22, 0x3a, 0x33, 0x34, 0x31,
            0x2e, 0x38, 0x2c, 0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3a, 0x32,
            0x2c, 0x22, 0x6e, 0x69, 0x63, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22, 0x3a, 0x31, 0x2c,
            0x22, 0x6e, 0x61, 0x63, 0x5f, 0x70, 0x22, 0x3a, 0x31, 0x30, 0x2c, 0x22, 0x6e, 0x61,
            0x63, 0x5f, 0x76, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x73, 0x69, 0x6c, 0x22, 0x3a, 0x33,
            0x2c, 0x22, 0x73, 0x69, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x22, 0x70,
            0x65, 0x72, 0x68, 0x6f, 0x75, 0x72, 0x22, 0x2c, 0x22, 0x67, 0x76, 0x61, 0x22, 0x3a,
            0x32, 0x2c, 0x22, 0x73, 0x64, 0x61, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x61, 0x6c, 0x65,
            0x72, 0x74, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x73, 0x70, 0x69, 0x22, 0x3a, 0x30, 0x2c,
            0x22, 0x6d, 0x6c, 0x61, 0x74, 0x22, 0x3a, 0x5b, 0x5d, 0x2c, 0x22, 0x74, 0x69, 0x73,
            0x62, 0x22, 0x3a, 0x5b, 0x5d, 0x2c, 0x22, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
            0x73, 0x22, 0x3a, 0x32, 0x36, 0x35, 0x37, 0x2c, 0x22, 0x73, 0x65, 0x65, 0x6e, 0x22,
            0x3a, 0x30, 0x2e, 0x30, 0x2c, 0x22, 0x72, 0x73, 0x73, 0x69, 0x22, 0x3a, 0x2d, 0x31,
            0x39, 0x2e, 0x30, 0x7d, 0xa,
        ];

        let output = format_adsb_json_frames_from_bytes(&input);

        assert_eq!(
            output.frames.len(),
            1,
            "Expected 1 frame, got {}",
            output.frames.len()
        );

        assert_eq!(
            output.left_over, "",
            "Expected empty string, got {}",
            output.left_over
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_adsb_json_parsing_input_multiple_frames_with_leftover_from_bytes() {
        let input = [
            0x7b_u8, 0x22, 0x6e, 0x6f, 0x77, 0x22, 0x20, 0x3a, 0x20, 0x31, 0x37, 0x30, 0x31, 0x31,
            0x30, 0x33, 0x33, 0x37, 0x33, 0x2e, 0x39, 0x31, 0x38, 0x2c, 0x22, 0x68, 0x65, 0x78,
            0x22, 0x3a, 0x22, 0x61, 0x63, 0x30, 0x37, 0x64, 0x63, 0x22, 0x2c, 0x22, 0x74, 0x79,
            0x70, 0x65, 0x22, 0x3a, 0x22, 0x61, 0x64, 0x73, 0x62, 0x5f, 0x69, 0x63, 0x61, 0x6f,
            0x22, 0x2c, 0x22, 0x66, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x22, 0x3a, 0x22, 0x53, 0x57,
            0x41, 0x33, 0x32, 0x32, 0x35, 0x20, 0x22, 0x2c, 0x22, 0x72, 0x22, 0x3a, 0x22, 0x4e,
            0x38, 0x37, 0x34, 0x32, 0x4d, 0x22, 0x2c, 0x22, 0x74, 0x22, 0x3a, 0x22, 0x42, 0x33,
            0x38, 0x4d, 0x22, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22,
            0x3a, 0x33, 0x38, 0x30, 0x30, 0x30, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x67, 0x65,
            0x6f, 0x6d, 0x22, 0x3a, 0x33, 0x38, 0x35, 0x32, 0x35, 0x2c, 0x22, 0x67, 0x73, 0x22,
            0x3a, 0x33, 0x39, 0x31, 0x2e, 0x37, 0x2c, 0x22, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x22,
            0x3a, 0x32, 0x38, 0x32, 0x2e, 0x30, 0x38, 0x2c, 0x22, 0x62, 0x61, 0x72, 0x6f, 0x5f,
            0x72, 0x61, 0x74, 0x65, 0x22, 0x3a, 0x36, 0x34, 0x2c, 0x22, 0x73, 0x71, 0x75, 0x61,
            0x77, 0x6b, 0x22, 0x3a, 0x22, 0x32, 0x35, 0x36, 0x34, 0x22, 0x2c, 0x22, 0x65, 0x6d,
            0x65, 0x72, 0x67, 0x65, 0x6e, 0x63, 0x79, 0x22, 0x3a, 0x22, 0x6e, 0x6f, 0x6e, 0x65,
            0x22, 0x2c, 0x22, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x22, 0x3a, 0x22,
            0x41, 0x33, 0x22, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x71, 0x6e, 0x68, 0x22, 0x3a,
            0x31, 0x30, 0x31, 0x33, 0x2e, 0x36, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x61, 0x6c,
            0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x5f, 0x6d, 0x63, 0x70, 0x22, 0x3a, 0x33, 0x38,
            0x30, 0x31, 0x36, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x69,
            0x6e, 0x67, 0x22, 0x3a, 0x32, 0x36, 0x37, 0x2e, 0x38, 0x39, 0x2c, 0x22, 0x6c, 0x61,
            0x74, 0x22, 0x3a, 0x33, 0x34, 0x2e, 0x35, 0x33, 0x32, 0x35, 0x34, 0x33, 0x2c, 0x22,
            0x6c, 0x6f, 0x6e, 0x22, 0x3a, 0x2d, 0x31, 0x30, 0x36, 0x2e, 0x35, 0x37, 0x38, 0x37,
            0x31, 0x32, 0x2c, 0x22, 0x6e, 0x69, 0x63, 0x22, 0x3a, 0x38, 0x2c, 0x22, 0x72, 0x63,
            0x22, 0x3a, 0x31, 0x38, 0x36, 0x2c, 0x22, 0x73, 0x65, 0x65, 0x6e, 0x5f, 0x70, 0x6f,
            0x73, 0x22, 0x3a, 0x30, 0x2e, 0x30, 0x30, 0x30, 0x2c, 0x22, 0x72, 0x5f, 0x64, 0x73,
            0x74, 0x22, 0x3a, 0x33, 0x39, 0x2e, 0x33, 0x36, 0x31, 0x2c, 0x22, 0x72, 0x5f, 0x64,
            0x69, 0x72, 0x22, 0x3a, 0x31, 0x38, 0x30, 0x2e, 0x37, 0x2c, 0x22, 0x76, 0x65, 0x72,
            0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x6e, 0x69, 0x63, 0x5f, 0x62,
            0x61, 0x72, 0x6f, 0x22, 0x3a, 0x31, 0x2c, 0x22, 0x6e, 0x61, 0x63, 0x5f, 0x70, 0x22,
            0x3a, 0x31, 0x30, 0x2c, 0x22, 0x6e, 0x61, 0x63, 0x5f, 0x76, 0x22, 0x3a, 0x32, 0x2c,
            0x22, 0x73, 0x69, 0x6c, 0x22, 0x3a, 0x33, 0x2c, 0x22, 0x73, 0x69, 0x6c, 0x5f, 0x74,
            0x79, 0x70, 0x65, 0x22, 0x3a, 0x22, 0x70, 0x65, 0x72, 0x68, 0x6f, 0x75, 0x72, 0x22,
            0x2c, 0x22, 0x67, 0x76, 0x61, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x73, 0x64, 0x61, 0x22,
            0x3a, 0x32, 0x2c, 0x22, 0x61, 0x6c, 0x65, 0x72, 0x74, 0x22, 0x3a, 0x30, 0x2c, 0x22,
            0x73, 0x70, 0x69, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x6d, 0x6c, 0x61, 0x74, 0x22, 0x3a,
            0x5b, 0x5d, 0x2c, 0x22, 0x74, 0x69, 0x73, 0x62, 0x22, 0x3a, 0x5b, 0x5d, 0x2c, 0x22,
            0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x3a, 0x33, 0x39, 0x32, 0x38,
            0x2c, 0x22, 0x73, 0x65, 0x65, 0x6e, 0x22, 0x3a, 0x30, 0x2e, 0x30, 0x2c, 0x22, 0x72,
            0x73, 0x73, 0x69, 0x22, 0x3a, 0x2d, 0x38, 0x2e, 0x36, 0x7d, 0xa, 0x7b, 0x22, 0x6e,
            0x6f, 0x77, 0x22, 0x20, 0x3a, 0x20, 0x31, 0x37, 0x30, 0x31, 0x31, 0x30, 0x33, 0x33,
            0x37, 0x33, 0x2e, 0x39, 0x33, 0x33, 0x2c, 0x22, 0x68, 0x65, 0x78, 0x22, 0x3a, 0x22,
            0x61, 0x31, 0x37, 0x62, 0x66, 0x65, 0x22, 0x2c, 0x22, 0x74, 0x79, 0x70, 0x65, 0x22,
            0x3a, 0x22, 0x61, 0x64, 0x73, 0x62, 0x5f, 0x69, 0x63, 0x61, 0x6f, 0x22, 0x2c, 0x22,
            0x66, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x22, 0x3a, 0x22, 0x41, 0x41, 0x59, 0x33, 0x31,
            0x38, 0x34, 0x20, 0x22, 0x2c, 0x22, 0x72, 0x22, 0x3a, 0x22, 0x4e, 0x31, 0x39, 0x35,
            0x4e, 0x56, 0x22, 0x2c, 0x22, 0x74, 0x22, 0x3a, 0x22, 0x41, 0x33, 0x32, 0x30, 0x22,
            0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22, 0x3a, 0x33, 0x33,
            0x39, 0x37, 0x35, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x67, 0x65, 0x6f, 0x6d, 0x22,
            0x3a, 0x33, 0x34, 0x33, 0x35, 0x30, 0x2c, 0x22, 0x67, 0x73, 0x22, 0x3a, 0x33, 0x36,
            0x37, 0x2e, 0x31, 0x2c, 0x22, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x3a, 0x32, 0x33,
            0x37, 0x2e, 0x39, 0x31, 0x2c, 0x22, 0x62, 0x61, 0x72, 0x6f, 0x5f, 0x72, 0x61, 0x74,
            0x65, 0x22, 0x3a, 0x2d, 0x36, 0x34, 0x2c, 0x22, 0x73, 0x71, 0x75, 0x61, 0x77, 0x6b,
            0x22, 0x3a, 0x22, 0x33, 0x36, 0x35, 0x36, 0x22, 0x2c, 0x22, 0x65, 0x6d, 0x65, 0x72,
            0x67, 0x65, 0x6e, 0x63, 0x79, 0x22, 0x3a, 0x22, 0x6e, 0x6f, 0x6e, 0x65, 0x22, 0x2c,
            0x22, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x22, 0x3a, 0x22, 0x41, 0x33,
            0x22, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x71, 0x6e, 0x68, 0x22, 0x3a, 0x31, 0x30,
            0x31, 0x32, 0x2e, 0x38, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x61, 0x6c, 0x74, 0x69,
            0x74, 0x75, 0x64, 0x65, 0x5f, 0x6d, 0x63, 0x70, 0x22, 0x3a, 0x33, 0x34, 0x30, 0x31,
            0x36, 0x2c, 0x22, 0x6c, 0x61, 0x74, 0x22, 0x3a, 0x33, 0x36, 0x2e, 0x34, 0x31, 0x31,
            0x33, 0x30, 0x31, 0x2c, 0x22, 0x6c, 0x6f, 0x6e, 0x22, 0x3a, 0x2d, 0x31, 0x30, 0x36,
            0x2e, 0x34, 0x31, 0x33, 0x32, 0x38, 0x38, 0x2c, 0x22, 0x6e, 0x69, 0x63, 0x22, 0x3a,
            0x38, 0x2c, 0x22, 0x72, 0x63, 0x22, 0x3a, 0x31, 0x38, 0x36, 0x2c, 0x22, 0x73, 0x65,
            0x65, 0x6e, 0x5f, 0x70, 0x6f, 0x73, 0x22, 0x3a, 0x30, 0x2e, 0x30, 0x30, 0x30, 0x2c,
            0x22, 0x72, 0x5f, 0x64, 0x73, 0x74, 0x22, 0x3a, 0x37, 0x33, 0x2e, 0x38, 0x33, 0x36,
            0x2c, 0x22, 0x72, 0x5f, 0x64, 0x69, 0x72, 0x22, 0x3a, 0x35, 0x2e, 0x39, 0x2c, 0x22,
            0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x6e, 0x69,
            0x63, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22, 0x3a, 0x31, 0x2c, 0x22, 0x6e, 0x61, 0x63,
            0x5f, 0x70, 0x22, 0x3a, 0x31, 0x30, 0x2c, 0x22, 0x6e, 0x61, 0x63, 0x5f, 0x76, 0x22,
            0x3a, 0x34, 0x2c, 0x22, 0x73, 0x69, 0x6c, 0x22, 0x3a, 0x33, 0x2c, 0x22, 0x73, 0x69,
            0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x22, 0x70, 0x65, 0x72, 0x68, 0x6f,
            0x75, 0x72, 0x22, 0x2c, 0x22, 0x67, 0x76, 0x61, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x73,
            0x64, 0x61, 0x22, 0x3a, 0x33, 0x2c, 0x22, 0x61, 0x6c, 0x65, 0x72, 0x74, 0x22, 0x3a,
            0x30, 0x2c, 0x22, 0x73, 0x70, 0x69, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x6d, 0x6c, 0x61,
            0x74, 0x22, 0x3a, 0x5b, 0x5d, 0x2c, 0x22, 0x74, 0x69, 0x73, 0x62, 0x22, 0x3a, 0x5b,
            0x5d, 0x2c, 0x22, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x3a, 0x34,
            0x33, 0x34, 0x38, 0x2c, 0x22, 0x73, 0x65, 0x65, 0x6e, 0x22, 0x3a, 0x30, 0x2e, 0x30,
            0x2c, 0x22, 0x72, 0x73, 0x73, 0x69, 0x22, 0x3a, 0x2d, 0x31, 0x31, 0x2e, 0x36, 0x7d,
            0xa, 0x7b, 0x22, 0x6e, 0x6f, 0x77, 0x22, 0x20, 0x3a, 0x20, 0x31, 0x37, 0x30, 0x31,
            0x31, 0x30, 0x33, 0x33, 0x37, 0x33, 0x2e, 0x39, 0x33, 0x34, 0x2c, 0x22, 0x68, 0x65,
            0x78, 0x22, 0x3a, 0x22, 0x61, 0x31, 0x32, 0x63, 0x31, 0x38, 0x22, 0x2c, 0x22, 0x74,
            0x79, 0x70, 0x65, 0x22, 0x3a, 0x22, 0x61, 0x64, 0x73, 0x62, 0x5f, 0x69, 0x63, 0x61,
            0x6f, 0x22, 0x2c, 0x22, 0x66, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x22, 0x3a, 0x22, 0x44,
            0x41, 0x4c, 0x37, 0x38, 0x30, 0x20, 0x20, 0x22, 0x2c, 0x22, 0x72, 0x22, 0x3a, 0x22,
            0x4e, 0x31, 0x37, 0x35, 0x44, 0x4e, 0x22, 0x2c, 0x22, 0x74, 0x22, 0x3a, 0x22, 0x42,
            0x37, 0x36, 0x33, 0x22, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x62, 0x61, 0x72, 0x6f,
            0x22, 0x3a, 0x33, 0x37, 0x30, 0x30, 0x30, 0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x67,
            0x65, 0x6f, 0x6d, 0x22, 0x3a, 0x33, 0x37, 0x34, 0x32, 0x35, 0x2c, 0x22, 0x67, 0x73,
            0x22, 0x3a, 0x35, 0x32, 0x37, 0x2e, 0x38, 0x2c, 0x22, 0x74, 0x72, 0x61, 0x63, 0x6b,
            0x22, 0x3a, 0x38, 0x36, 0x2e, 0x38, 0x35, 0x2c, 0x22, 0x62, 0x61, 0x72, 0x6f, 0x5f,
            0x72, 0x61, 0x74, 0x65, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x73, 0x71, 0x75, 0x61, 0x77,
            0x6b, 0x22, 0x3a, 0x22, 0x31, 0x33, 0x32, 0x33, 0x22, 0x2c, 0x22, 0x65, 0x6d, 0x65,
            0x72, 0x67, 0x65, 0x6e, 0x63, 0x79, 0x22, 0x3a, 0x22, 0x6e, 0x6f, 0x6e, 0x65, 0x22,
            0x2c, 0x22, 0x63, 0x61, 0x74, 0x65, 0x67, 0x6f, 0x72, 0x79, 0x22, 0x3a, 0x22, 0x41,
            0x35, 0x22, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x71, 0x6e, 0x68, 0x22, 0x3a, 0x31,
            0x30, 0x31, 0x33, 0x2e, 0x36, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x61, 0x6c, 0x74,
            0x69, 0x74, 0x75, 0x64, 0x65, 0x5f, 0x6d, 0x63, 0x70, 0x22, 0x3a, 0x33, 0x36, 0x39,
            0x39, 0x32, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x69, 0x6e,
            0x67, 0x22, 0x3a, 0x37, 0x38, 0x2e, 0x30, 0x35, 0x2c, 0x22, 0x6c, 0x61, 0x74, 0x22,
            0x3a, 0x33, 0x35, 0x2e, 0x31, 0x31, 0x32, 0x34, 0x38, 0x38, 0x2c, 0x22, 0x6c, 0x6f,
            0x6e, 0x22, 0x3a, 0x2d, 0x31, 0x30, 0x36, 0x2e, 0x36, 0x30, 0x34, 0x39, 0x33, 0x38,
            0x2c, 0x22, 0x6e, 0x69, 0x63, 0x22, 0x3a, 0x38, 0x2c, 0x22, 0x72, 0x63, 0x22, 0x3a,
            0x31, 0x38, 0x36, 0x2c, 0x22, 0x73, 0x65, 0x65, 0x6e, 0x5f, 0x70, 0x6f, 0x73, 0x22,
            0x3a, 0x30, 0x2e, 0x30, 0x30, 0x30, 0x2c, 0x22, 0x72, 0x5f, 0x64, 0x73, 0x74, 0x22,
            0x3a, 0x34, 0x2e, 0x38, 0x36, 0x30, 0x2c, 0x22, 0x72, 0x5f, 0x64, 0x69, 0x72, 0x22,
            0x3a, 0x32, 0x30, 0x31, 0x2e, 0x30, 0x2c, 0x22, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
            0x6e, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x6e, 0x69, 0x63, 0x5f, 0x62, 0x61, 0x72, 0x6f,
            0x22, 0x3a, 0x31, 0x2c, 0x22, 0x6e, 0x61, 0x63, 0x5f, 0x70, 0x22, 0x3a, 0x31, 0x30,
            0x2c, 0x22, 0x6e, 0x61, 0x63, 0x5f, 0x76, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x73, 0x69,
            0x6c, 0x22, 0x3a, 0x33, 0x2c, 0x22, 0x73, 0x69, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65,
            0x22, 0x3a, 0x22, 0x70, 0x65, 0x72, 0x68, 0x6f, 0x75, 0x72, 0x22, 0x2c, 0x22, 0x67,
            0x76, 0x61, 0x22, 0x3a, 0x32, 0x2c, 0x22, 0x73, 0x64, 0x61, 0x22, 0x3a, 0x32, 0x2c,
            0x22, 0x61, 0x6c, 0x65, 0x72, 0x74, 0x22, 0x3a, 0x30, 0x2c, 0x22, 0x73, 0x70, 0x69,
            0x22, 0x3a, 0x30, 0x2c, 0x22, 0x6d, 0x6c, 0x61, 0x74, 0x22, 0x3a, 0x5b, 0x5d, 0x2c,
            0x22, 0x74, 0x69, 0x73, 0x62, 0x22, 0x3a, 0x5b, 0x5d, 0x2c, 0x22, 0x6d, 0x65, 0x73,
            0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x3a, 0x36, 0x38, 0x39, 0x36, 0x2c, 0x22, 0x73,
            0x65, 0x65, 0x6e, 0x22, 0x3a, 0x30, 0x2e, 0x30, 0x2c, 0x22, 0x72, 0x73, 0x73, 0x69,
            0x22, 0x3a, 0x2d, 0x33, 0x2e, 0x38, 0x7d, 0xa, 0x7b, 0x22, 0x6e, 0x6f, 0x77, 0x22,
            0x20, 0x3a, 0x20, 0x31, 0x37, 0x30, 0x31, 0x31, 0x30, 0x33, 0x33, 0x37, 0x33, 0x2e,
            0x39, 0x34, 0x31, 0x2c, 0x22, 0x68, 0x65, 0x78, 0x22, 0x3a, 0x22, 0x61, 0x62, 0x32,
            0x33, 0x63, 0x32, 0x22, 0x2c, 0x22, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x22, 0x61,
            0x64, 0x73, 0x62, 0x5f, 0x69, 0x63, 0x61, 0x6f, 0x22, 0x2c, 0x22, 0x66, 0x6c, 0x69,
            0x67, 0x68, 0x74, 0x22, 0x3a, 0x22, 0x4e, 0x38, 0x31, 0x37, 0x45, 0x41, 0x20, 0x20,
            0x22, 0x2c, 0x22, 0x72, 0x22, 0x3a, 0x22, 0x4e, 0x38, 0x31, 0x37, 0x45, 0x41, 0x22,
            0x2c, 0x22, 0x74, 0x22, 0x3a, 0x22, 0x43, 0x35, 0x36, 0x30, 0x22, 0x2c, 0x22, 0x61,
            0x6c, 0x74, 0x5f, 0x62, 0x61, 0x72, 0x6f, 0x22, 0x3a, 0x34, 0x33, 0x30, 0x30, 0x30,
            0x2c, 0x22, 0x61, 0x6c, 0x74, 0x5f, 0x67, 0x65, 0x6f, 0x6d, 0x22, 0x3a, 0x34, 0x33,
            0x35, 0x35, 0x30, 0x2c, 0x22, 0x67, 0x73, 0x22, 0x3a, 0x33, 0x33, 0x35, 0x2e, 0x32,
            0x2c, 0x22, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x22, 0x3a, 0x32, 0x37, 0x34, 0x2e, 0x37,
            0x39, 0x2c, 0x22, 0x62, 0x61, 0x72, 0x6f, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x22, 0x3a,
            0x30, 0x2c, 0x22, 0x73, 0x71, 0x75, 0x61, 0x77, 0x6b, 0x22, 0x3a, 0x22, 0x37, 0x30,
            0x36, 0x34, 0x22, 0x2c, 0x22, 0x65, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x6e, 0x63, 0x79,
            0x22, 0x3a, 0x22, 0x6e, 0x6f, 0x6e, 0x65, 0x22, 0x2c, 0x22, 0x63, 0x61, 0x74, 0x65,
            0x67, 0x6f, 0x72, 0x79, 0x22, 0x3a, 0x22, 0x41, 0x32, 0x22, 0x2c, 0x22, 0x6e, 0x61,
            0x76, 0x5f, 0x71, 0x6e, 0x68, 0x22, 0x3a, 0x31, 0x30, 0x31, 0x33, 0x2e, 0x36, 0x2c,
            0x22, 0x6e, 0x61, 0x76, 0x5f, 0x61, 0x6c, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x5f,
            0x6d, 0x63, 0x70, 0x22, 0x3a, 0x34, 0x33, 0x30, 0x30, 0x38, 0x2c, 0x22, 0x6e, 0x61,
            0x76, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x22, 0x3a, 0x32, 0x36, 0x32,
            0x2e, 0x32, 0x37, 0x2c, 0x22, 0x6e, 0x61, 0x76, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x73,
            0x22, 0x3a, 0x5b, 0x22, 0x61, 0x75, 0x74, 0x6f, 0x70, 0x69, 0x6c, 0x6f, 0x74, 0x22,
            0x2c, 0x22, 0x61, 0x6c, 0x74, 0x68, 0x6f, 0x6c, 0x64, 0x22, 0x2c, 0x22, 0x6c, 0x6e,
            0x61, 0x76, 0x22, 0x2c, 0x22, 0x74, 0x63, 0x61, 0x73, 0x22, 0x5d, 0x2c, 0x22, 0x6c,
            0x61, 0x74, 0x22, 0x3a, 0x33, 0x34, 0x2e, 0x33, 0x37, 0x35, 0x33, 0x35, 0x31, 0x2c,
            0x22, 0x6c, 0x6f,
        ];

        let output = format_adsb_json_frames_from_bytes(&input);

        assert_eq!(
            output.frames.len(),
            3,
            "Expected 3 frames, got {}",
            output.frames.len()
        );
        assert_eq!(
            output.left_over,
            "{\"now\" : 1701103373.941,\"hex\":\"ab23c2\",\"type\":\"adsb_icao\",\"flight\":\"N817EA  \",\"r\":\"N817EA\",\"t\":\"C560\",\"alt_baro\":43000,\"alt_geom\":43550,\"gs\":335.2,\"track\":274.79,\"baro_rate\":0,\"squawk\":\"7064\",\"emergency\":\"none\",\"category\":\"A2\",\"nav_qnh\":1013.6,\"nav_altitude_mcp\":43008,\"nav_heading\":262.27,\"nav_modes\":[\"autopilot\",\"althold\",\"lnav\",\"tcas\"],\"lat\":34.375351,\"lo",
            "Expected incomplete frame"
        );
    }
}
