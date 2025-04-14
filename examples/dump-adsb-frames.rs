// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

/// # Examples
/// This example program shows how to use the library to connect to a source of ADSB frames and print the raw frames
/// To run this example to process tar1090 aircraft.json file individually, run the following command:
/// ```bash
/// cargo run --example dump-adsb-frames -- --url http://localhost:8080/data/aircraft.json --mode jsonfromurlindividual
/// ```
///
/// To run this example to process readsb JSON, run the following command:
/// ```bash
/// cargo run --example dump-adsb-frames -- --url http://localhost:8080/data/aircraft.json --mode jsonfromurlbulk
/// ```
///
/// To run this example to process readsb JSON from a TCP connection, run the following command:
/// ```bash
/// cargo run --example dump-adsb-frames -- --url localhost:30047 --mode jsonfromtcp
/// ```
///
/// To run this example to process raw frames from a TCP connection, run the following command:
/// ```bash
/// cargo run --example dump-adsb-frames -- --url localhost:30002 --mode raw
/// ```
///
/// To run this example to process beast frames from a TCP connection, run the following command:
/// ```bash
/// cargo run --example dump-adsb-frames -- --url localhost:30005 --mode beast
/// ```
///
/// The program by default will print out the decoded messages to stdout. With each change in log level, more information will be printed out.

#[macro_use]
extern crate log;
use generic_async_http_client::{Request, Response};
use sdre_rust_adsb_parser::{
    ADSBMessage, DecodeMessage,
    decoders::{
        aircraftjson::{AircraftJSON, NewAircraftJSONMessage},
        beast::{AdsbBeastMessage, NewAdsbBeastMessage},
        json::{JSONMessage, NewJSONMessage},
        raw::NewAdsbRawMessage,
    },
    error_handling::deserialization_error::DeserializationError,
    helpers::{
        encode_adsb_beast_input::{ADSBBeastFrames, format_adsb_beast_frames_from_bytes},
        encode_adsb_json_input::format_adsb_json_frames_from_string,
        encode_adsb_raw_input::{ADSBRawFrames, format_adsb_raw_frames_from_bytes},
    },
};
use sdre_rust_logging::SetupLogging;
use sdre_stubborn_io::{ReconnectOptions, StubbornTcpStream, config::DurationIterator};
use std::fmt;
use std::net::SocketAddr;
use std::process::exit;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::{io::AsyncReadExt, time::sleep};

#[derive(Debug, Default)]
enum Modes {
    #[default]
    JSONFromURLIndividual,
    JSONFromUrlBulk,
    JSONFromTCP,
    Raw,
    Beast,
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
    log_verbosity: String,
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

        let log_verbosity: String = if let Some(log_verbosity_temp) = log_verbosity_temp {
            match log_verbosity_temp.parse::<String>() {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid log verbosity: {e:?}");
                    println!("Defaulting to info");
                    "info".to_string()
                }
            }
        } else {
            "info".to_string()
        };

        let mode: Modes = if let Some(mode) = mode {
            match mode.parse::<Modes>() {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid mode: {e:?}");
                    println!(
                        "Valid modes are: jsonfromurlindividual, jsonfromurlbulk, jsonfromtcp, raw, beast"
                    );
                    exit(1);
                }
            }
        } else {
            Modes::default()
        };

        Ok(Args {
            url,
            log_verbosity,
            mode,
            only_show_errors,
            direct_decode,
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
            dump-adsb-frames: Decodes readsb JSON, readsb airplanes.json, ADSB Raw or ADSB packets and prints the results to stdout\n\
\n\
            Args:\n\
            --url [url:[port]]: URL and optional port to get ADSB data from\n\
            --log-verbosity [0-5]: Set the log verbosity\n\
            --mode [jsonfromurlindividual, jsonfromurlbulk, jsonfromtcp, raw, beast]: Set the mode to use\n\
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
    // open a TCP connection to ip. Grab the frames and process them as beast
    let addr = match ip.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error parsing host: {}", e);
            return Ok(());
        }
    };

    let mut stream =
        match StubbornTcpStream::connect_with_options(addr, reconnect_options(ip)).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error connecting to {}: {}", ip, e);
                Err(e)?
            }
        };

    info!("Connected to {}", ip);
    let mut buffer: [u8; 4096] = [0u8; 4096];
    let mut left_over: Vec<u8> = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:02X?}", buffer[0..n].to_vec());
        let processed_buffer: Vec<u8> = [&left_over[..], &buffer[0..n]].concat();
        let frames: ADSBBeastFrames = format_adsb_beast_frames_from_bytes(&processed_buffer);

        if !frames.errors.is_empty() {
            for error in frames.errors {
                error!("Error decoding: {}", error);
            }

            info!("Full buffer: {:02X?}", processed_buffer);
            info!("Left over before: {:02X?}", left_over);
            info!("Left over after: {:02X?}", frames.left_over);
            info!("Frames: {:02X?}", frames.frames);
        }

        left_over = frames.left_over;

        trace!("Pre-processed: {:02X?}", frames.frames);
        for frame in &frames.frames {
            debug!("Decoding: {:02X?}", frame);

            if !direct_decode {
                let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:02X?}", frame);
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
    let addr = match ip.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error parsing host: {}", e);
            return Ok(());
        }
    };

    let mut stream =
        match StubbornTcpStream::connect_with_options(addr, reconnect_options(ip)).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error connecting to {}: {}", ip, e);
                Err(e)?
            }
        };

    info!("Connected to {}", ip);
    let mut buffer: [u8; 4096] = [0u8; 4096];
    let mut left_over: Vec<u8> = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:02X?}", buffer[0..n].to_vec());

        // append the left over bytes to the buffer
        let processed_buffer: Vec<u8> = [&left_over[..], &buffer[0..n]].concat();
        let frames: ADSBRawFrames = format_adsb_raw_frames_from_bytes(&processed_buffer);

        if !frames.errors.is_empty() {
            for error in frames.errors {
                error!("Error decoding: {}", error);
            }

            info!("Full buffer: {:02X?}", processed_buffer);
            info!("Left over before: {:02X?}", left_over);
            info!("Left over after: {:02X?}", frames.left_over);
            info!("Frames: {:02X?}", frames.frames);
        }

        left_over = frames.left_over;

        trace!("Pre-processed: {:02X?}", frames.frames);

        for frame in &frames.frames {
            debug!("Decoding: {:02X?}", frame);
            if !direct_decode {
                let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:02X?}", frame);
                }
            } else {
                let message = frame.to_adsb_raw();
                if let Ok(message) = message {
                    if !only_show_errors {
                        info!("Decoded: {}", message.pretty_print());
                    }
                } else {
                    error!("Error decoding: {}", message.unwrap_err());
                    error!("Message input: {:02X?}", frame);
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
    let addr = match ip.parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error parsing host: {}", e);
            return Ok(());
        }
    };

    let mut stream =
        match StubbornTcpStream::connect_with_options(addr, reconnect_options(ip)).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Error connecting to {}: {}", ip, e);
                Err(e)?
            }
        };

    info!("Connected to {}", ip);
    info!("Any error frames will be printed out once per hex");
    let mut buffer: [u8; 8000] = [0u8; 8000];
    let mut left_over = String::new();

    let mut error_frames = Vec::new();

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:02X?}", buffer[0..n].to_vec());
        // convert the bytes to a string
        let mut json_string: String = String::from_utf8_lossy(&buffer[0..n]).to_string();
        trace!("Pre-processed: {}", json_string);

        // if we have a left over string, prepend it to the json_string
        if !left_over.is_empty() {
            json_string = format!("{}{}", left_over, json_string);
        }

        let frames = format_adsb_json_frames_from_string(&json_string);

        trace!("Pre-processed with left overs: {:02X?}", frames.frames);

        left_over = frames.left_over;

        if !frames.errors.is_empty() {
            for error in frames.errors {
                error!("Error decoding: {}", error);
            }

            info!("Full buffer: {}", json_string);
            info!("Left over: {}", left_over);
            info!("Frames: {:?}", frames.frames);
        }

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

// create ReconnectOptions. We want the TCP stuff that goes out and connects to clients
// to attempt to reconnect
// See: https://docs.rs/stubborn-io/latest/src/stubborn_io/config.rs.html#93

pub fn reconnect_options(host: &str) -> ReconnectOptions {
    ReconnectOptions::new()
        .with_exit_if_first_connect_fails(false)
        .with_retries_generator(get_our_standard_reconnect_strategy)
        .with_connection_name(host)
}

fn get_our_standard_reconnect_strategy() -> DurationIterator {
    let initial_attempts = vec![
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(5),
        Duration::from_secs(10),
        Duration::from_secs(20),
        Duration::from_secs(30),
        Duration::from_secs(40),
        Duration::from_secs(50),
        Duration::from_secs(60),
    ];

    let repeat = std::iter::repeat(Duration::from_secs(60));

    let forever_iterator = initial_attempts.into_iter().chain(repeat);

    Box::new(forever_iterator)
}
