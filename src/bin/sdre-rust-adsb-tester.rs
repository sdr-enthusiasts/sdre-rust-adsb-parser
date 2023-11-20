// A small binary to read in a file of ADS-B messages and print them out from an inputted URL
#[macro_use]
extern crate log;
use generic_async_http_client::Request;
use sdre_rust_adsb_parser::DecodeMessage;
use std::env;
use std::process;
use std::time::Instant;

use sdre_rust_logging::SetupLogging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: sdre-rust-adsb-tester <url>");
        process::exit(1);
    }

    if args.len() == 3 {
        let log_level = &args[2];

        // match the input string to the log level
        match log_level.as_str() {
            "trace" => {
                3.enable_logging();
            }
            "debug" => {
                2.enable_logging();
            }
            "info" => {
                0.enable_logging();
            }
            "warn" => {
                0.enable_logging();
            }
            "error" => {
                3.enable_logging();
            }
            _ => {
                eprintln!("Invalid log level: {}. Setting to INFO", log_level);
                0.enable_logging();
            }
        }
    } else {
        0.enable_logging();
    }

    // loop and connect to the URL given
    let url_input = &args[1];

    loop {
        let req = Request::get(url_input);
        let mut planes_procesed = 0;
        let total_time: String;

        let mut resp = req.exec().await?;
        if resp.status_code() == 200 {
            let body = resp.text().await?;
            // for now we'll bust apart the response before parsing
            let now = Instant::now();
            for line in body.lines() {
                if line.starts_with("{")
                    && line.trim().len() > 0
                    && !line.starts_with("{ \"now\" : ")
                {
                    let final_message_to_process = line.trim().trim_end_matches(',');
                    debug!("Processing: {}", final_message_to_process);
                    let message = final_message_to_process.decode_message()?;
                    debug!("Decoded: {:?}", message);
                    planes_procesed += 1;
                }
            }
            let elapsed = now.elapsed();
            total_time = format!("{:.2?}", elapsed);
        } else {
            error!("Response status error: {:?}", resp.status());
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            continue;
        }
        info!("Processed {} planes in {}", planes_procesed, total_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
