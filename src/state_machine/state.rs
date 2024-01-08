// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use core::time;
use std::collections::{hash_map::Entry, HashMap};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use crate::decoders::json_types::timestamp::TimeStamp;
use crate::decoders::raw_types::ke;
use crate::{
    data_structures::airplane::Airplane,
    decoders::{
        aircraftjson::AircraftJSON, beast::AdsbBeastMessage, json::JSONMessage, raw::AdsbRawMessage,
    },
    ADSBMessage,
};

pub struct StateMachine {
    pub airplanes: Arc<Mutex<HashMap<String, Airplane>>>,
    pub timeout_in_seconds: u64,
    input_channel: Sender<ADSBMessage>,
    output_channel: Receiver<ADSBMessage>,
    messages_processed: Arc<Mutex<u64>>,
}

impl StateMachine {
    pub fn new(timeout_in_seconds: u32) -> StateMachine {
        let (sender_channel, receiver_channel) = tokio::sync::mpsc::channel(100);
        StateMachine {
            airplanes: Arc::new(Mutex::new(HashMap::new())),
            timeout_in_seconds: timeout_in_seconds as u64,
            input_channel: sender_channel,
            output_channel: receiver_channel,
            messages_processed: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_sender_channel(&self) -> Sender<ADSBMessage> {
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
            match message {
                ADSBMessage::AdsbRawMessage(raw_message) => {
                    self.process_aircraft_raw(raw_message).await
                }
                ADSBMessage::AdsbBeastMessage(adsb_beast_message) => {
                    self.process_aircraft_beast(adsb_beast_message).await
                }
                ADSBMessage::AircraftJSON(aircraft_json) => {
                    self.process_aircraft_json(aircraft_json).await
                }
                ADSBMessage::JSONMessage(json_message) => {
                    self.process_json_message(json_message).await
                }
            }

            let mut messages_processed = self.messages_processed.lock().await;
            *messages_processed += 1;
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

    pub async fn process_aircraft_raw(&mut self, message: AdsbRawMessage) {
        unimplemented!("RawMessage")
    }

    pub async fn process_aircraft_beast(&mut self, message: AdsbBeastMessage) {
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
    timeout_in_seconds: u64,
) {
    let timeout_in_seconds = timeout_in_seconds as f64;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(check_interval_in_seconds)).await;
        // current unix timestamp
        let current_time = chrono::Utc::now().timestamp() as f64;
        let mut airplanes = planes.lock().await;

        let mut planes_to_remove = Vec::new();

        for (key, airplane) in airplanes.iter() {
            match airplane.timestamp {
                TimeStamp::TimeStampAsF64(timestamp) => {
                    if current_time - timestamp > timeout_in_seconds {
                        planes_to_remove.push(key.clone());
                    }
                }
                TimeStamp::None => {
                    planes_to_remove.push(key.clone());
                }
            }
        }

        info!(
            "Tracking {} airplane{}. Removing {} for a new total of {}",
            airplanes.len(),
            if airplanes.len() == 1 { "" } else { "s" },
            planes_to_remove.len(),
            airplanes.len() - planes_to_remove.len()
        );

        for key in planes_to_remove {
            airplanes.remove(&key);
        }
    }
}
