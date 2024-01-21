// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

/// # Examples
/// This example program shows how to use the library to connect to a source of ADSB frames and generate state from them
/// To run this example to process tar1090 aircraft.json file individually, run the following command:
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
use log::{debug, error, info, trace};
use rocket::{get, routes, State};

use generic_async_http_client::{Request, Response};
use rocket::serde::json::Json;
use sdre_rust_adsb_parser::{
    decoders::{aircraftjson::AircraftJSON, json::JSONMessage},
    error_handling::deserialization_error::DeserializationError,
    helpers::{
        encode_adsb_beast_input::{format_adsb_beast_frames_from_bytes, ADSBBeastFrames},
        encode_adsb_json_input::format_adsb_json_frames_from_string,
        encode_adsb_raw_input::{format_adsb_raw_frames_from_bytes, ADSBRawFrames},
    },
    state_machine::state::{
        expire_planes, generate_aircraft_json, ProcessMessageType, StateMachine,
    },
    ADSBMessage, DecodeMessage,
};
use sdre_rust_logging::SetupLogging;
use sdre_stubborn_io::{config::DurationIterator, ReconnectOptions, StubbornTcpStream};
use std::str::FromStr;
use std::{collections::HashMap, net::SocketAddr};
use std::{fmt, time::Duration};
use std::{process::exit, sync::Arc};
use tokio::{io::AsyncReadExt, sync::Mutex, time::sleep};

#[derive(Debug, Default)]
enum Modes {
    #[default]
    JSONFromAircraftJSON,
    JSONFromTCP,
    Raw,
    Beast,
}

impl FromStr for Modes {
    type Err = ArgParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jsonfromaircraftjson" => Ok(Modes::JSONFromAircraftJSON),
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
            Modes::JSONFromAircraftJSON => write!(f, "JSON from aircraft.json"),
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
    print_state_interval_seconds: u64,
    print_json: bool,
    lat: f64,
    lon: f64,
}

impl Args {
    fn try_parse<It: Iterator<Item = String>>(mut arg_it: It) -> Result<Args, ArgParseError> {
        // Skip program name
        let _ = arg_it.next();

        let mut url: Option<String> = None;
        let mut log_verbosity_temp: Option<String> = None;
        let mut mode: Option<String> = None;
        let mut print_state_interval_seconds: u64 = 10;
        let mut print_json = false;
        let mut lat = None;
        let mut lon = None;

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
                "--print-json" => {
                    print_json = true;
                }
                "--print-state-interval" => {
                    print_state_interval_seconds = arg_it
                        .next()
                        .map(|s| s.parse::<u64>().unwrap_or(10))
                        .unwrap_or(10);
                }
                "--lat" => {
                    lat = arg_it.next().map(|s| s.parse::<f64>().unwrap_or(90.0));
                }
                "--lon" => {
                    lon = arg_it.next().map(|s| s.parse::<f64>().unwrap_or(360.0));
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
                    println!("Valid modes are: jsonfromurlindividual, jsonfromurlbulk, jsonfromtcp, raw, beast");
                    exit(1);
                }
            }
        } else {
            Modes::default()
        };

        // make sure lat/lon are both some
        if lat.is_none() {
            println!("Latitude not set.");
            exit(1);
        }

        if lon.is_none() {
            println!("Longitude not set.");
            exit(1);
        }

        Ok(Args {
            url,
            log_verbosity,
            mode,
            print_state_interval_seconds,
            print_json,
            lat: lat.unwrap(),
            lon: lon.unwrap(),
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
            --print-json: Print the JSON to stdout\n\
            --print-state-interval [seconds]: Set the interval to print state in seconds\n\
            --lat [latitude]: Set the latitude to use for distance calculations. Only used for raw/beast frames\n\
            --lon [longitude]: Set the longitude to use for distance calculations. Only used for raw/beast frames\n\
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
    let print_interval_in_seconds: u64 = args.print_state_interval_seconds;
    let print_json = &args.print_json;
    let lat = args.lat;
    let lon = args.lon;

    match mode {
        Modes::JSONFromAircraftJSON => {
            info!("Processing as Aircraft JSON");
            process_as_aircraft_json(url_input, print_interval_in_seconds, print_json, lat, lon)
                .await?;
        }
        Modes::JSONFromTCP => {
            info!("Processing as JSON from TCP");
            process_json_from_tcp(url_input, print_interval_in_seconds, print_json, lat, lon)
                .await?;
        }
        Modes::Raw => {
            info!("Processing as raw frames");
            process_raw_frames(url_input, print_interval_in_seconds, print_json, lat, lon).await?;
        }
        Modes::Beast => {
            info!("Processing as beast frames");
            process_beast_frames(url_input, print_interval_in_seconds, print_json, lat, lon)
                .await?;
        }
    }

    Ok(())
}

async fn process_beast_frames(
    ip: &str,
    print_interval_in_seconds: u64,
    print_json: &bool,
    lat: f64,
    lon: f64,
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

    let mut state_machine = StateMachine::new(90, 360, lat, lon);
    let sender_channel = state_machine.get_sender_channel();
    let print_mutex_context = state_machine.get_airplanes_mutex();
    let message_count_context = state_machine.get_messages_processed_mutex();
    let expire_mutex_context = state_machine.get_airplanes_mutex();
    let adsb_expire_timeout = state_machine.adsb_timeout_in_seconds;
    let adsc_expire_timeout = state_machine.adsc_timeout_in_seconds;

    // rocket state machine
    let rocket_print_mutex_context = state_machine.get_airplanes_mutex();
    let rocket_message_count_context = state_machine.get_messages_processed_mutex();

    // start the rocket server

    tokio::spawn(async move {
        rocket(rocket_print_mutex_context, rocket_message_count_context).await;
        // stop the program if the rocket server stops
        exit(0);
    });

    if *print_json {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(print_interval_in_seconds))
                    .await;
                match generate_aircraft_json(
                    print_mutex_context.clone(),
                    message_count_context.clone(),
                )
                .await
                {
                    Some(aircraft_json) => {
                        info!("Aircraft JSON: {}", aircraft_json.to_string().unwrap());
                    }
                    None => {
                        error!("Error generating aircraft JSON");
                    }
                }
            }
        });
    }

    tokio::spawn(async move {
        state_machine.process_adsb_message().await;
    });

    tokio::spawn(async move {
        expire_planes(
            expire_mutex_context,
            10,
            adsb_expire_timeout,
            adsc_expire_timeout,
        )
        .await;
    });

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            error!("No data read");
            continue;
        }
        trace!("Raw frame: {:02X?}", buffer[0..n].to_vec());

        // append the left over bytes to the buffer
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

        for frame in frames.frames {
            debug!("Decoding: {:02X?}", frame);

            sender_channel
                .send(ProcessMessageType::AsVecU8(frame))
                .await
                .unwrap();
        }
    }
    Ok(())
}

async fn process_raw_frames(
    ip: &str,
    print_interval_in_seconds: u64,
    print_json: &bool,
    lat: f64,
    lon: f64,
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

    let mut state_machine = StateMachine::new(90, 360, lat, lon);
    let sender_channel = state_machine.get_sender_channel();
    let print_mutex_context = state_machine.get_airplanes_mutex();
    let message_count_context = state_machine.get_messages_processed_mutex();
    let expire_mutex_context = state_machine.get_airplanes_mutex();
    let adsb_expire_timeout = state_machine.adsb_timeout_in_seconds;
    let adsc_expire_timeout = state_machine.adsc_timeout_in_seconds;

    // rocket state machine
    let rocket_print_mutex_context = state_machine.get_airplanes_mutex();
    let rocket_message_count_context = state_machine.get_messages_processed_mutex();

    // start the rocket server

    tokio::spawn(async move {
        rocket(rocket_print_mutex_context, rocket_message_count_context).await;
        // stop the program if the rocket server stops
        exit(0);
    });

    if *print_json {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(print_interval_in_seconds))
                    .await;
                match generate_aircraft_json(
                    print_mutex_context.clone(),
                    message_count_context.clone(),
                )
                .await
                {
                    Some(aircraft_json) => {
                        info!("Aircraft JSON: {}", aircraft_json.to_string().unwrap());
                    }
                    None => {
                        error!("Error generating aircraft JSON");
                    }
                }
            }
        });
    }

    tokio::spawn(async move {
        state_machine.process_adsb_message().await;
    });

    tokio::spawn(async move {
        expire_planes(
            expire_mutex_context,
            10,
            adsb_expire_timeout,
            adsc_expire_timeout,
        )
        .await;
    });

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

        for frame in frames.frames {
            debug!("Decoding: {:02X?}", frame);

            sender_channel
                .send(ProcessMessageType::AsVecU8(frame))
                .await
                .unwrap();
        }
    }
    Ok(())
}

async fn process_as_aircraft_json(
    url: &str,
    print_interval_in_seconds: u64,
    print_json: &bool,
    lat: f64,
    lon: f64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut state_machine = StateMachine::new(90, 360, lat, lon);
    let sender_channel = state_machine.get_sender_channel();
    let print_mutex_context = state_machine.get_airplanes_mutex();
    let message_count_context = state_machine.get_messages_processed_mutex();
    let expire_mutex_context = state_machine.get_airplanes_mutex();
    let adsb_expire_timeout = state_machine.adsb_timeout_in_seconds;
    let adsc_expire_timeout = state_machine.adsc_timeout_in_seconds;

    // rocket state machine
    let rocket_print_mutex_context = state_machine.get_airplanes_mutex();
    let rocket_message_count_context = state_machine.get_messages_processed_mutex();

    // start the rocket server

    tokio::spawn(async move {
        rocket(rocket_print_mutex_context, rocket_message_count_context).await;
        // stop the program if the rocket server stops
        exit(0);
    });

    if *print_json {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(print_interval_in_seconds))
                    .await;
                match generate_aircraft_json(
                    print_mutex_context.clone(),
                    message_count_context.clone(),
                )
                .await
                {
                    Some(aircraft_json) => {
                        info!("Aircraft JSON: {}", aircraft_json.to_string().unwrap());
                    }
                    None => {
                        error!("Error generating aircraft JSON");
                    }
                }
            }
        });
    }

    tokio::spawn(async move {
        state_machine.process_adsb_message().await;
    });

    tokio::spawn(async move {
        expire_planes(
            expire_mutex_context,
            10,
            adsb_expire_timeout,
            adsc_expire_timeout,
        )
        .await;
    });

    loop {
        let req: Request = Request::get(url);

        let mut resp: Response = req.exec().await?;
        if resp.status_code() == 200 {
            let body: String = resp.text().await?;
            // for now we'll bust apart the response before parsing
            for line in body.lines() {
                if line.starts_with('{') && !line.is_empty() && !line.starts_with("{ \"now\" : ") {
                    let final_message_to_process: &str = line.trim().trim_end_matches(',');
                    debug!("Decoding: {}", final_message_to_process);

                    let message: Result<ADSBMessage, DeserializationError> =
                        final_message_to_process.decode_message();
                    if let Ok(message) = message {
                        sender_channel
                            .send(ProcessMessageType::ADSBMessage(message))
                            .await
                            .unwrap();
                    } else {
                        error!("Error decoding: {}", message.unwrap_err());
                        error!("Message input: {}", final_message_to_process);
                    }
                }
            }
        } else {
            error!("Response status error: {}", resp.status());
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        sleep(Duration::from_secs(10)).await;
    }
}

async fn process_json_from_tcp(
    ip: &str,
    print_interval_in_seconds: u64,
    print_json: &bool,
    lat: f64,
    lon: f64,
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

    let mut buffer: [u8; 8000] = [0u8; 8000];
    let mut left_over = String::new();

    let mut state_machine = StateMachine::new(90, 360, lat, lon);
    let sender_channel = state_machine.get_sender_channel();
    let print_mutex_context = state_machine.get_airplanes_mutex();
    let message_count_context = state_machine.get_messages_processed_mutex();
    let expire_mutex_context = state_machine.get_airplanes_mutex();
    let adsb_expire_timeout = state_machine.adsb_timeout_in_seconds;
    let adsc_expire_timeout = state_machine.adsc_timeout_in_seconds;

    // rocket state machine
    let rocket_print_mutex_context = state_machine.get_airplanes_mutex();
    let rocket_message_count_context = state_machine.get_messages_processed_mutex();

    // start the rocket server

    tokio::spawn(async move {
        rocket(rocket_print_mutex_context, rocket_message_count_context).await;
        // stop the program if the rocket server stops
        exit(0);
    });

    if *print_json {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(print_interval_in_seconds))
                    .await;
                match generate_aircraft_json(
                    print_mutex_context.clone(),
                    message_count_context.clone(),
                )
                .await
                {
                    Some(aircraft_json) => {
                        info!("Aircraft JSON: {}", aircraft_json.to_string().unwrap());
                    }
                    None => {
                        error!("Error generating aircraft JSON");
                    }
                }
            }
        });
    }

    tokio::spawn(async move {
        state_machine.process_adsb_message().await;
    });

    tokio::spawn(async move {
        expire_planes(
            expire_mutex_context,
            10,
            adsb_expire_timeout,
            adsc_expire_timeout,
        )
        .await;
    });

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

            let message: Result<ADSBMessage, DeserializationError> = frame.decode_message();
            if let Ok(message) = message {
                sender_channel
                    .send(ProcessMessageType::ADSBMessage(message))
                    .await
                    .unwrap();
            } else {
                error!("Error decoding: {}", message.unwrap_err());
                error!("Message input: {}", frame);
            }
        }
    }
    Ok(())
}

struct Model {
    print_context: Arc<Mutex<HashMap<String, JSONMessage>>>,
    message_count_context: Arc<Mutex<u64>>,
}

#[get("/data/aircraft.json")]
async fn aircraft_json(model: &State<Model>) -> Json<AircraftJSON> {
    let print_context = model.print_context.clone();
    let message_count_context = model.message_count_context.clone();

    let aircraft_json = generate_aircraft_json(print_context, message_count_context).await;
    if let Some(aircraft_json) = aircraft_json {
        Json(aircraft_json)
    } else {
        Json(AircraftJSON::default())
    }
}

async fn rocket(
    print_context: Arc<Mutex<HashMap<String, JSONMessage>>>,
    message_count_context: Arc<Mutex<u64>>,
) {
    let model = Model {
        print_context,
        message_count_context,
    };

    match rocket::build()
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("log_level", rocket::config::LogLevel::Critical)),
        )
        .manage(model)
        .mount("/", routes![aircraft_json])
        .launch()
        .await
    {
        Ok(_) => {
            println!("Rocket exited");
        }
        Err(e) => {
            println!("Error launching rocket: {}", e);
        }
    }
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
