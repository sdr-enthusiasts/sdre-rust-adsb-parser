// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

/// A small binary to read in a file of ADS-B messages and print them out from an inputted URL
///
/// # Examples
/// This example program shows how to use the library to read in a file of ADS-B messages and print them out
/// To run this example to process tar1090 aircraft.json file individually, run the following command:
/// ```bash
/// cargo run --example sdre-rust-adsb-tester -- --url http://localhost:8080/data/aircraft.json --mode jsonfromurlindividual
/// ```
///
/// To run this example to process readsb JSON, run the following command:
/// ```bash
/// cargo run --example sdre-rust-adsb-tester -- --url http://localhost:8080/data/aircraft.json --mode jsonfromurlbulk
/// ```
///
/// To run this example to process readsb JSON from a TCP connection, run the following command:
/// ```bash
/// cargo run --example sdre-rust-adsb-tester -- --url localhost:30047 --mode jsonfromtcp
/// ```
///
/// To run this example to process raw frames from a TCP connection, run the following command:
/// ```bash
/// cargo run --example sdre-rust-adsb-tester -- --url localhost:30002 --mode raw
/// ```
///
/// To run this example to process beast frames from a TCP connection, run the following command:
/// ```bash
/// cargo run --example sdre-rust-adsb-tester -- --url localhost:30005 --mode beast
/// ```
///
/// The program by default will print out the decoded messages to stdout. With each change in log level, more information will be printed out.

#[macro_use]
extern crate log;
use generic_async_http_client::Request;
use generic_async_http_client::Response;
use sdre_rust_adsb_parser::decoders::aircraftjson::AircraftJSON;
use sdre_rust_adsb_parser::decoders::aircraftjson::NewAircraftJSONMessage;
use sdre_rust_adsb_parser::decoders::beast::AdsbBeastMessage;
use sdre_rust_adsb_parser::decoders::beast::NewAdsbBeastMessage;
use sdre_rust_adsb_parser::decoders::json::JSONMessage;
use sdre_rust_adsb_parser::decoders::json::NewJSONMessage;
use sdre_rust_adsb_parser::decoders::raw::NewAdsbRawMessage;
use sdre_rust_adsb_parser::error_handling::deserialization_error::DeserializationError;
use sdre_rust_adsb_parser::helpers::encode_adsb_beast_input::format_adsb_beast_frames_from_bytes;
use sdre_rust_adsb_parser::helpers::encode_adsb_beast_input::ADSBBeastFrames;
use sdre_rust_adsb_parser::helpers::encode_adsb_json_input::format_adsb_json_frames_from_string;
use sdre_rust_adsb_parser::helpers::encode_adsb_raw_input::format_adsb_raw_frames_from_bytes;
use sdre_rust_adsb_parser::helpers::encode_adsb_raw_input::ADSBRawFrames;
use sdre_rust_adsb_parser::ADSBMessage;
use sdre_rust_adsb_parser::DecodeMessage;
use sdre_rust_logging::SetupLogging;
use std::fmt;
use std::process::exit;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Debug)]
enum Modes {
    JSONFromURLIndividual,
    JSONFromUrlBulk,
    JSONFromTCP,
    Raw,
    Beast,
}

impl Default for Modes {
    fn default() -> Self {
        Modes::JSONFromURLIndividual
    }
}

impl FromStr for Modes {
    type Err = ArgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jsonfromurlindividual" => Ok(Modes::JSONFromURLIndividual),
            "jsonfromurlbulk" => Ok(Modes::JSONFromUrlBulk),
            "jsonfromtcp" => Ok(Modes::JSONFromTCP),
            "raw" => Ok(Modes::Raw),
            "beast" => Ok(Modes::Beast),
            _ => Err(ArgParseError::InvalidMode),
        }
    }
}

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modes::JSONFromURLIndividual => write!(f, "JSON from URL, individual"),
            Modes::JSONFromUrlBulk => write!(f, "JSON from URL, bulk"),
            Modes::JSONFromTCP => write!(f, "JSON from tcp"),
            Modes::Raw => write!(f, "raw"),
            Modes::Beast => write!(f, "beast"),
        }
    }
}

#[derive(Debug)]
enum ArgParseError {
    UrlMissing,
    InvalidMode,
}

struct Args {
    url: String,
    log_verbosity: u8,
    mode: Modes,
    only_show_errors: bool,
    direct_decode: bool,
}

impl Args {
    fn try_parse<It: Iterator<Item = String>>(mut arg_it: It) -> Result<Args, ArgParseError> {
        // Skip program name
        let _ = arg_it.next();

        let mut url: Option<String> = None;
        let mut log_verbosity_temp: Option<String> = None;
        let mut mode: Option<String> = None;
        let mut only_show_errors: bool = false;
        let mut direct_decode: bool = false;

        while let Some(arg) = arg_it.next() {
            match arg.as_str() {
                "--url" => {
                    url = arg_it.next().map(Into::into);
                }
                "--log-verbosity" => {
                    log_verbosity_temp = arg_it.next().map(Into::into);
                }
                "--mode" => {
                    mode = arg_it.next().map(Into::into);
                }
                "--help" => {
                    println!("{}", Args::help());
                    exit(0);
                }
                "--only-show-errors" => {
                    only_show_errors = true;
                }
                "--direct-decode" => {
                    direct_decode = true;
                }
                s => {
                    println!("Invalid argument: {s}");
                    println!("{}", Args::help());
                    exit(1);
                }
            }
        }

        let url: String = url.ok_or(ArgParseError::UrlMissing)?;

        let log_verbosity: u8 = if let Some(log_verbosity_temp) = log_verbosity_temp {
            match log_verbosity_temp.parse::<u8>() {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid log verbosity: {e:?}");
                    println!("Defaulting to 0");
                    0
                }
            }
        } else {
            0
        };

        let mode: Modes = if let Some(mode) = mode {
            mode.parse::<Modes>().unwrap()
        } else {
            Modes::default()
        };

        Ok(Args {
            url: url,
            log_verbosity: log_verbosity,
            mode: mode,
            only_show_errors: only_show_errors,
            direct_decode: direct_decode,
        })
    }

    fn parse<It: Iterator<Item = String>>(arg_it: It) -> Args {
        match Self::try_parse(arg_it) {
            Ok(v) => v,
            Err(e) => {
                println!("Argument parsing failed: {e:?}");
                println!("{}", Args::help());
                exit(1);
            }
        }
    }

    fn help() -> String {
        "\n\
            sdre-rust-adsb-tester: Decodes readsb JSON, readsb airplanes.json, ADSB Raw or ADSB packets and prints the results to stdout\n\
\n\
            Args:\n\
            --url [url:[port]]: URL and optional port to get ADSB data from\n\
            --log-verbosity [0-5]: Set the log verbosity\n\
            --mode [jsonfromurlindividual, jsonfromurlbulk, jsonfromtcp, raw, beast]: Set the mode to use\n\
            --pretty-print [standard, usa, metric]: Set the pretty print mode\n\
            --only-show-errors: Only show errors\n\
            --direct-decode: Directly decode the message using the appropriate message type. Otherwise, will use generic decoder to infer type\n\
            --help: Show this help and exit\n\
        "
        .to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Args = Args::parse(std::env::args());

    args.log_verbosity.enable_logging();

    // loop and connect to the URL given
    let url_input: &String = &args.url;
    let mode: &Modes = &args.mode;
    let only_show_errors: &bool = &args.only_show_errors;
    let direct_decode: &bool = &args.direct_decode;

    match mode {
        Modes::JSONFromURLIndividual => {
            info!("Processing as individual messages");
            process_as_individual_messages(url_input, only_show_errors, direct_decode).await?;
        }
        Modes::JSONFromUrlBulk => {
            info!("Processing as bulk messages");
            process_as_bulk_messages(url_input, only_show_errors, direct_decode).await?;
        }
        Modes::JSONFromTCP => {
            info!("Processing as JSON from TCP");
            process_json_from_tcp(url_input, only_show_errors, direct_decode).await?;
        }
        Modes::Raw => {
            info!("Processing as raw frames");
            process_raw_frames(url_input, only_show_errors, direct_decode).await?;
        }
        Modes::Beast => {
            info!("Processing as beast frames");
            process_beast_frames(url_input, only_show_errors, direct_decode).await?;
        }
    }

    Ok(())
}

async fn process_beast_frames(
    ip: &str,
    only_show_errors: &bool,
    direct_decode: &bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as raw
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    let mut buffer: [u8; 4096] = [0u8; 4096];
    let mut left_over: Vec<u8> = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:x?}", buffer[0..n].to_vec());
        let processed_buffer: Vec<u8> = [&left_over[..], &buffer[0..n]].concat();
        let frames: ADSBBeastFrames = format_adsb_beast_frames_from_bytes(&processed_buffer);
        left_over = frames.left_over;

        trace!("Pre-processed: {:x?}", frames.frames);
        for frame in &frames.frames {
            debug!("Decoding: {:x?}", frame);

            if !direct_decode {
                let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:x?}", frame);
                }
            } else {
                let message: Result<AdsbBeastMessage, DeserializationError> = frame.to_adsb_beast();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                }
            }
        }
    }
    Ok(())
}

async fn process_raw_frames(
    ip: &str,
    only_show_errors: &bool,
    direct_decode: &bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as raw
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    let mut buffer: [u8; 4096] = [0u8; 4096];
    let mut left_over: Vec<u8> = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:x?}", buffer[0..n].to_vec());

        // append the left over bytes to the buffer
        let processed_buffer: Vec<u8> = [&left_over[..], &buffer[0..n]].concat();
        let frames: ADSBRawFrames = format_adsb_raw_frames_from_bytes(&processed_buffer);
        left_over = frames.left_over;

        trace!("Pre-processed: {:?}", frames.frames);

        for frame in &frames.frames {
            debug!("Decoding: {:x?}", frame);
            if !direct_decode {
                let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:x?}", frame);
                }
            } else {
                let message = frame.to_adsb_raw();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:x?}", frame);
                }
            }
        }
    }
    Ok(())
}

async fn process_as_bulk_messages(
    url: &str,
    only_show_errors: &bool,
    direct_decode: &bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let req: Request = Request::get(url);
        let total_time: String;
        let mut planes_procesed: usize = 0;

        let mut resp: Response = req.exec().await?;
        if resp.status_code() == 200 {
            let body: String = resp.text().await?;
            // for now we'll bust apart the response before parsing
            let now: Instant = Instant::now();

            trace!("Processing: {}", body);
            if !direct_decode {
                let message: Result<ADSBMessage, DeserializationError> = body.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                    planes_procesed = message.len();
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                }
            } else {
                let message: Result<AircraftJSON, DeserializationError> = body.to_aircraft_json();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                    planes_procesed = message.len();
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                }
            }

            let elapsed: Duration = now.elapsed();
            total_time = format!("{:.2?}", elapsed);
        } else {
            error!("Response status error: {}", resp.status());
            sleep(Duration::from_secs(10)).await;
            continue;
        }
        info!("Processed {} planes in {}", planes_procesed, total_time);
        sleep(Duration::from_secs(10)).await;
    }
}

async fn process_as_individual_messages(
    url: &str,
    only_show_errors: &bool,
    direct_decode: &bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Error frames will be printed out for every error encountered");
    loop {
        let req: Request = Request::get(url);
        let mut planes_procesed = 0;
        let total_time: String;

        let mut resp: Response = req.exec().await?;
        if resp.status_code() == 200 {
            let body: String = resp.text().await?;
            // for now we'll bust apart the response before parsing
            let now: Instant = Instant::now();
            for line in body.lines() {
                if line.starts_with('{') && !line.is_empty() && !line.starts_with("{ \"now\" : ") {
                    let final_message_to_process: &str = line.trim().trim_end_matches(',');
                    debug!("Decoding: {}", final_message_to_process);
                    if !direct_decode {
                        let message: Result<ADSBMessage, DeserializationError> =
                            final_message_to_process.decode_message();

                        if let Ok(message) = message {
                            if !only_show_errors {
                                info!("Decoded: {}", message.pretty_print());
                            }

                            planes_procesed += 1;
                        } else {
                            error!("Error decoding: {}", message.unwrap_err());
                        }
                    } else {
                        let message: Result<JSONMessage, DeserializationError> =
                            final_message_to_process.to_json();

                        if let Ok(message) = message {
                            if !only_show_errors {
                                info!("Decoded: {}", message.pretty_print());
                            }

                            planes_procesed += 1;
                        } else {
                            error!("Error decoding: {}", message.unwrap_err());
                        }
                    }
                }
            }
            let elapsed: Duration = now.elapsed();
            total_time = format!("{:.2?}", elapsed);
        } else {
            error!("Response status error: {}", resp.status());
            sleep(Duration::from_secs(10)).await;
            continue;
        }
        info!("Processed {} planes in {}", planes_procesed, total_time);
        sleep(Duration::from_secs(10)).await;
    }
}

async fn process_json_from_tcp(
    ip: &str,
    only_show_errors: &bool,
    direct_decode: &bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as JSON
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    info!("Any error frames will be printed out once per hex");
    let mut buffer: [u8; 8000] = [0u8; 8000];
    let mut left_over = String::new();

    let mut error_frames = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:x?}", buffer[0..n].to_vec());
        // convert the bytes to a string
        let mut json_string: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
        trace!("Pre-processed: {}", json_string);

        // if we have a left over string, prepend it to the json_string
        if !left_over.is_empty() {
            json_string = format!("{}{}", left_over, json_string);
        }

        let frames = format_adsb_json_frames_from_string(&json_string);

        trace!("Pre-processed with left overs: {:?}", frames.frames);

        left_over = frames.left_over;

        for frame in frames.frames {
            debug!("Decoding: {}", frame);
            if *direct_decode {
                let message = frame.to_json();

                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    // split the string on the comma, iterate over the strings, find the hex field
                    // store it in the error_frames vector
                    let mut split_string: Vec<&str> = frame.split(',').collect();

                    for split in split_string.iter_mut() {
                        if split.starts_with("\"hex\":") {
                            let hex = split.replace("\"hex\":", "");
                            // check if the hex is already in the error_frames vector
                            if !error_frames.contains(&hex) {
                                error_frames.push(hex);
                                error!("Error decoding: {}", message.unwrap_err());
                                error!("Message input: {}", frame);
                            } else {
                                continue;
                            }
                            break;
                        }
                    }
                }
            } else {
                let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    // split the string on the comma, iterate over the strings, find the hex field
                    // store it in the error_frames vector
                    let mut split_string: Vec<&str> = frame.split(',').collect();

                    for split in split_string.iter_mut() {
                        if split.starts_with("\"hex\":") {
                            let hex = split.replace("\"hex\":", "");
                            // check if the hex is already in the error_frames vector
                            if !error_frames.contains(&hex) {
                                error_frames.push(hex);
                                error!("Error decoding: {}", message.unwrap_err());
                                error!("Message input: {}", frame);
                            } else {
                                continue;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
