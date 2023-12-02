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
use core::fmt;
use generic_async_http_client::Request;
use generic_async_http_client::Response;
use sdre_rust_adsb_parser::decoders::json::NewJSONMessage;
use sdre_rust_adsb_parser::error_handling::deserialization_error::DeserializationError;
use sdre_rust_adsb_parser::helpers::encode_adsb_beast_input::format_adsb_beast_frames_from_bytes;
use sdre_rust_adsb_parser::helpers::encode_adsb_beast_input::ADSBBeastFrames;
use sdre_rust_adsb_parser::helpers::encode_adsb_json_input::format_adsb_json_frames_from_string;
use sdre_rust_adsb_parser::helpers::encode_adsb_raw_input::format_adsb_raw_frames_from_bytes;
use sdre_rust_adsb_parser::helpers::encode_adsb_raw_input::ADSBRawFrames;
use sdre_rust_adsb_parser::ADSBMessage;
use sdre_rust_adsb_parser::DecodeMessage;
use sdre_rust_logging::SetupLogging;
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
}

impl Args {
    fn try_parse<It: Iterator<Item = String>>(mut arg_it: It) -> Result<Args, ArgParseError> {
        // Skip program name
        let _ = arg_it.next();

        let mut url: Option<String> = None;
        let mut log_verbosity_temp: Option<String> = None;
        let mut mode: Option<String> = None;

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

    match mode {
        Modes::JSONFromURLIndividual => {
            info!("Processing as individual messages");
            process_as_individual_messages(url_input).await?;
        }
        Modes::JSONFromUrlBulk => {
            info!("Processing as bulk messages");
            process_as_bulk_messages(url_input).await?;
        }
        Modes::JSONFromTCP => {
            info!("Processing as JSON from TCP");
            process_json_from_tcp(url_input).await?;
        }
        Modes::Raw => {
            info!("Processing as raw frames");
            process_raw_frames(url_input).await?;
        }
        Modes::Beast => {
            info!("Processing as beast frames");
            process_beast_frames(url_input).await?;
        }
    }

    Ok(())
}

async fn process_json_from_tcp(ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as JSON
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    let mut buffer: [u8; 8000] = [0u8; 8000];
    let mut left_over = String::new();

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
            //let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
            let message = NewJSONMessage::to_json(&frame);
            if let Ok(message_done) = message {
                info!("Decoded:\n{}", message_done.pretty_print());
            } else {
                error!("Error decoding: {}", message.unwrap_err());
            }
        }
    }
    Ok(())
}

async fn process_beast_frames(ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as raw
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    let mut buffer: [u8; 1024] = [0u8; 1024];

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:x?}", buffer[0..n].to_vec());
        let frames: ADSBBeastFrames = format_adsb_beast_frames_from_bytes(&buffer[0..n]);
        trace!("Pre-processed: {:x?}", frames.frames);
        for frame in frames.frames {
            debug!("Decoding: {:x?}", frame);
            let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
            if let Ok(message_done) = message {
                info!("Decoded {:x?}: {}", frame, message_done);
            } else {
                error!("Error decoding: {}", message.unwrap_err());
            }
        }
    }
    Ok(())
}

async fn process_raw_frames(ip: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // open a TCP connection to ip. Grab the frames and process them as raw
    let mut stream: BufReader<TcpStream> = BufReader::new(TcpStream::connect(ip).await?);
    info!("Connected to {:?}", stream);
    let mut buffer: [u8; 1024] = [0u8; 1024];

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:x?}", buffer[0..n].to_vec());

        let frames: ADSBRawFrames = format_adsb_raw_frames_from_bytes(&buffer[0..n]);

        trace!("Pre-processed: {:?}", frames.frames);

        for frame in frames.frames {
            debug!("Decoding: {:x?}", frame);
            let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
            if let Ok(message_done) = message {
                info!("Decoded {:?}: {}", frame, message_done);
            } else {
                error!("Error decoding: {}", message.unwrap_err());
            }
        }
    }
    Ok(())
}

async fn process_as_bulk_messages(
    url: &str,
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
            let message: Result<ADSBMessage, DeserializationError> = body.decode_message();
            if let Ok(message) = message {
                info!("Decoded: {}", message);
                planes_procesed = message.len();
            } else {
                error!("Error decoding: {}", message.unwrap_err());
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
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                    let message: Result<ADSBMessage, DeserializationError> =
                        final_message_to_process.decode_message();

                    if let Ok(message) = message {
                        info!("Decoded: {:?}", message);
                        planes_procesed += 1;
                    } else {
                        error!("Error decoding: {}", message.unwrap_err());
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
