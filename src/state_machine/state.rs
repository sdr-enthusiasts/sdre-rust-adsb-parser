/// This module contains the implementation of the state machine for processing ADS-B messages.
/// The state machine handles various types of input messages, such as raw ADS-B data, JSON messages,
/// and AircraftJSON messages. It maintains a collection of airplanes and processes incoming messages
/// to update the state of the airplanes. The state machine also provides methods for retrieving and
/// printing airplane information.
///
/// # Examples
///
/// ```
/// use sdre_rust_adsb_parser::state_machine::state::StateMachine;
///
/// // Create a new state machine with a timeout of 10 seconds for ADS-B messages
/// let state_machine = StateMachine::new(10, 0, 37.7749, -122.4194);
///
/// // Get the sender channel to send messages to the state machine
/// let sender_channel = state_machine.get_sender_channel();
///
/// // Send a raw ADS-B message to the state machine
/// sender_channel.send(ProcessMessageType::Raw(raw_message)).await;
///
/// // Process the incoming messages in the state machine
/// state_machine.process_adsb_message().await;
///
/// // Print the airplanes in the state machine
/// state_machine.print_airplanes().await;
/// ```
///
/// The state machine processes different types of messages, such as raw ADS-B data, JSON messages,
/// and AircraftJSON messages. It maintains a collection of airplanes and updates their state based
/// on the incoming messages. The state machine provides methods for retrieving and printing airplane
/// information. It also allows sending messages to the state machine for processing.
///
/// The state machine can be created using the `new` method, which takes the timeout values for ADS-B
/// and ADS-C messages, as well as the latitude and longitude for decoding surface position messages.
/// The state machine provides a sender channel to send messages for processing, and the `process_adsb_message`
/// method can be used to process the incoming messages. The `print_airplanes` method prints the information
/// of all the airplanes in the state machine.
///
/// The state machine uses a mutex-protected hashmap to store the airplanes. The `get_airplane_by_hex` method
/// allows retrieving an airplane by its transponder hex code. The `print_airplane_by_hex` method prints the
/// information of a specific airplane. The `get_airplanes` method returns a vector of all the airplanes in
/// the state machine.
///
/// The state machine processes different types of messages, such as raw ADS-B data, JSON messages,
/// and AircraftJSON messages. The `process_adsb_message` method is responsible for processing these messages.
/// It uses pattern matching to handle different message types and calls the corresponding processing methods.
/// The processing methods update the state of the airplanes based on the incoming messages.
///
/// The state machine also keeps track of the number of messages processed using a mutex-protected counter.
/// The `get_messages_processed_mutex` method returns a mutex-protected reference to the counter.
///
/// Note: The state machine is designed to be used in a multi-threaded environment, where multiple threads
/// can send messages to the state machine for processing concurrently. The state machine ensures thread-safety
/// by using mutexes to protect shared data structures.
// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.
use core::fmt;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use crate::decoders::helpers::cpr_calculators::Position;
use crate::decoders::json_types::timestamp::TimeStamp;
use crate::decoders::raw_types::df::DF;
use crate::DecodeMessage;
use crate::{
    data_structures::airplane::Airplane,
    decoders::{
        aircraftjson::AircraftJSON, beast::AdsbBeastMessage, json::JSONMessage,
        json_types::messagetype::MessageType::ADSC, raw::AdsbRawMessage,
    },
    ADSBMessage,
};

#[derive(Debug, Clone)]
pub enum ProcessMessageType {
    Raw(AdsbRawMessage),
    Beast(AdsbBeastMessage),
    JSON(JSONMessage),
    AircraftJSON(AircraftJSON),
    ADSBMessage(ADSBMessage),
    AsVecU8(Vec<u8>),
    AsString(String),
}

impl fmt::Display for ProcessMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessMessageType::Raw(raw_message) => write!(f, "{}", raw_message),
            ProcessMessageType::Beast(beast_message) => write!(f, "{}", beast_message),
            ProcessMessageType::JSON(json_message) => write!(f, "{}", json_message),
            ProcessMessageType::AircraftJSON(aircraft_json) => write!(f, "{}", aircraft_json),
            ProcessMessageType::ADSBMessage(adsb_message) => write!(f, "{}", adsb_message),
            ProcessMessageType::AsVecU8(vec_u8) => {
                let mut output = "".to_string();
                for byte in vec_u8 {
                    output.push_str(&format!("{:02X?}", byte));
                }
                write!(f, "{}", output)
            }
            ProcessMessageType::AsString(string) => write!(f, "{}", string),
        }
    }
}

pub struct StateMachine {
    pub airplanes: Arc<Mutex<HashMap<String, Airplane>>>,
    pub adsb_timeout_in_seconds: u64,
    pub adsc_timeout_in_seconds: u64,
    input_channel: Sender<ProcessMessageType>,
    output_channel: Receiver<ProcessMessageType>,
    messages_processed: Arc<Mutex<u64>>,
    position: Position,
}

// Note: Input to the state machine is a single frame of ADS-B data (beast/raw), AircraftJSON, or JSON
/// Create the state machine. The state machine will enable the user to set the timeout for ADS-B and ADS-C messages.
/// The state machine needs a user-defined lat/lon for decoding Surface Position messages. This position is also used
/// for airborne aircraft positions if the aircraft position cannot be derived from the available messages received.
impl StateMachine {
    pub fn new(
        adsb_timeout_in_seconds: u32,
        adsc_timeout_in_seconds: u32,
        lat: f64,
        lon: f64,
    ) -> StateMachine {
        let (sender_channel, receiver_channel): (
            Sender<ProcessMessageType>,
            Receiver<ProcessMessageType>,
        ) = tokio::sync::mpsc::channel(100);
        StateMachine {
            airplanes: Arc::new(Mutex::new(HashMap::new())),
            adsb_timeout_in_seconds: adsb_timeout_in_seconds as u64,
            adsc_timeout_in_seconds: adsc_timeout_in_seconds as u64,
            input_channel: sender_channel,
            output_channel: receiver_channel,
            messages_processed: Arc::new(Mutex::new(0)),
            position: Position {
                latitude: lat,
                longitude: lon,
            },
        }
    }

    pub fn get_sender_channel(&self) -> Sender<ProcessMessageType> {
        self.input_channel.clone()
    }

    pub fn get_airplanes_mutex(&self) -> Arc<Mutex<HashMap<String, Airplane>>> {
        self.airplanes.clone()
    }

    pub fn get_messages_processed_mutex(&self) -> Arc<Mutex<u64>> {
        self.messages_processed.clone()
    }

    pub async fn get_airplane_by_hex(&self, transponder_hex: &str) -> Option<Airplane> {
        let airplanes = self.airplanes.lock().await;

        airplanes.get(transponder_hex).cloned()
    }

    pub async fn print_airplane_by_hex(&self, transponder_hex: &str) {
        match self.get_airplane_by_hex(transponder_hex).await {
            Some(airplane) => println!("{}", airplane),
            None => println!("No airplane found with transponder hex {}", transponder_hex),
        }
    }

    pub async fn print_airplanes(&self) {
        let airplanes = self.airplanes.lock().await;

        for (_, airplane) in airplanes.iter() {
            println!("{}", airplane);
        }
    }

    pub async fn get_airplanes(&self) -> Vec<Airplane> {
        let mut airplanes = self.airplanes.lock().await;
        let mut airplanes_vec = Vec::new();

        for (_, airplane) in airplanes.iter_mut() {
            airplanes_vec.push(airplane.clone());
        }

        airplanes_vec
    }

    pub async fn process_adsb_message(&mut self) {
        while let Some(message) = self.output_channel.recv().await {
            let mut result: Result<(), String> = Ok(());

            match message.clone() {
                ProcessMessageType::Raw(raw_message) => {
                    result = self.process_aircraft_raw(raw_message).await
                }
                ProcessMessageType::Beast(beast_message) => {
                    result = self.process_aircraft_beast(beast_message).await
                }
                ProcessMessageType::JSON(json_message) => {
                    self.process_json_message(json_message).await
                }
                ProcessMessageType::AircraftJSON(aircraft_json) => {
                    self.process_aircraft_json(aircraft_json).await
                }
                ProcessMessageType::ADSBMessage(adsb_message) => match adsb_message {
                    ADSBMessage::AdsbRawMessage(raw_message) => {
                        result = self.process_aircraft_raw(raw_message).await
                    }
                    ADSBMessage::AdsbBeastMessage(beast_message) => {
                        result = self.process_aircraft_beast(beast_message).await
                    }
                    ADSBMessage::AircraftJSON(json_message) => {
                        self.process_aircraft_json(json_message).await
                    }
                    ADSBMessage::JSONMessage(json_message) => {
                        self.process_json_message(json_message).await
                    }
                },
                ProcessMessageType::AsVecU8(vec_u8) => {
                    if let Ok(message) = vec_u8.decode_message() {
                        match message {
                            ADSBMessage::AdsbRawMessage(raw_message) => {
                                result = self.process_aircraft_raw(raw_message).await
                            }
                            ADSBMessage::AdsbBeastMessage(beast_message) => {
                                result = self.process_aircraft_beast(beast_message).await
                            }
                            ADSBMessage::AircraftJSON(json_message) => {
                                self.process_aircraft_json(json_message).await
                            }
                            ADSBMessage::JSONMessage(json_message) => {
                                self.process_json_message(json_message).await
                            }
                        }
                    }
                }
                ProcessMessageType::AsString(string) => {
                    if let Ok(message) = string.decode_message() {
                        match message {
                            ADSBMessage::AdsbRawMessage(raw_message) => {
                                result = self.process_aircraft_raw(raw_message).await
                            }
                            ADSBMessage::AdsbBeastMessage(beast_message) => {
                                result = self.process_aircraft_beast(beast_message).await
                            }
                            ADSBMessage::AircraftJSON(json_message) => {
                                self.process_aircraft_json(json_message).await
                            }
                            ADSBMessage::JSONMessage(json_message) => {
                                self.process_json_message(json_message).await
                            }
                        }
                    }
                }
            }

            let mut messages_processed = self.messages_processed.lock().await;
            *messages_processed += 1;

            if let Err(e) = result {
                error!("{}", e);
                error!("Message: {}", message);
            }
        }
    }

    pub async fn process_json_message(&mut self, message: JSONMessage) {
        // lock the mutex and get a mutable reference to the hashmap
        let mut airplanes = self.airplanes.lock().await;

        // get the airplane from the hashmap
        match airplanes.entry(
            message
                .transponder_hex
                .get_transponder_hex_as_string()
                .clone(),
        ) {
            // if the airplane exists, update it
            Entry::Occupied(mut airplane) => {
                debug!("Updating airplane {}", airplane.get().transponder_hex);
                airplane.get_mut().update_from_json(&message);
            }

            // if the airplane doesn't exist, create it
            Entry::Vacant(airplane) => {
                debug!("Creating airplane {}", message.transponder_hex);
                airplane.insert(message);
            }
        }
    }

    pub async fn process_aircraft_json(&mut self, message: AircraftJSON) {
        for aircraft in message.aircraft {
            self.process_json_message(aircraft).await;
        }
    }

    pub async fn process_aircraft_raw(&mut self, message: AdsbRawMessage) -> Result<(), String> {
        if let DF::ADSB(adsb) = &message.df {
            let mut airplanes = self.airplanes.lock().await;

            let transponderhex = adsb.icao.to_string();

            match airplanes.entry(transponderhex.clone()) {
                Entry::Occupied(mut airplane) => {
                    return airplane
                        .get_mut()
                        .update_from_df(&message.df, &self.position);
                }
                Entry::Vacant(airplane) => {
                    let mut new_airplane = Airplane::new(transponderhex);
                    match new_airplane.update_from_df(&message.df, &self.position) {
                        Ok(_) => {
                            airplane.insert(new_airplane);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn process_aircraft_beast(
        &mut self,
        message: AdsbBeastMessage,
    ) -> Result<(), String> {
        self.process_aircraft_raw(message.raw_message).await
    }
}

pub async fn generate_aircraft_json(
    planes: Arc<Mutex<HashMap<String, Airplane>>>,
    messages: Arc<Mutex<u64>>,
) -> Option<AircraftJSON> {
    let airplanes = planes.lock().await;
    let total_messages = messages.lock().await;

    let vec_of_planes = airplanes.values().cloned().collect();

    Some(AircraftJSON::new(vec_of_planes, *total_messages))
}

pub async fn expire_planes(
    planes: Arc<Mutex<HashMap<String, Airplane>>>,
    check_interval_in_seconds: u64,
    adsb_timeout_in_seconds: u64,
    adsc_timeout_in_seconds: u64,
) {
    let adsb_timeout_in_seconds = adsb_timeout_in_seconds as f64;
    let adsc_timeout_in_seconds = adsc_timeout_in_seconds as f64;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(check_interval_in_seconds)).await;
        // current unix timestamp
        let current_time = chrono::Utc::now().timestamp() as f64;
        let mut airplanes = planes.lock().await;
        let mut planes_removed = 0;

        airplanes.retain(|_, value| match value.timestamp {
            TimeStamp::TimeStampAsF64(timestamp) => match &value.message_type {
                ADSC => {
                    if current_time - timestamp > adsc_timeout_in_seconds {
                        planes_removed += 1;
                        info!("Removing ADSC");
                        false
                    } else {
                        true
                    }
                }
                _ => {
                    if current_time - timestamp > adsb_timeout_in_seconds {
                        planes_removed += 1;
                        false
                    } else {
                        true
                    }
                }
            },
            TimeStamp::None => {
                planes_removed += 1;
                false
            }
        });

        info!(
            "Tracking {} airplane{}. Removing {} for a new total of {}",
            airplanes.len() + planes_removed,
            if airplanes.len() + planes_removed == 1 {
                ""
            } else {
                "s"
            },
            planes_removed,
            airplanes.len()
        );
    }
}
