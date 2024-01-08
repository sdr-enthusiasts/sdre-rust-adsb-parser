// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::collections::HashMap;

use crate::{
    data_structures::airplane::Airplane,
    decoders::{
        aircraftjson::AircraftJSON, beast::AdsbBeastMessage, json::JSONMessage, raw::AdsbRawMessage,
    },
    ADSBMessage,
};

pub struct StateMachine {
    pub airplanes: HashMap<String, Airplane>,
    timeout_in_seconds: u64,
}

impl StateMachine {
    pub fn new(timeout_in_seconds: u32) -> StateMachine {
        StateMachine {
            airplanes: HashMap::new(),
            timeout_in_seconds: timeout_in_seconds as u64,
        }
    }

    pub async fn process_adsb_message(&mut self, message: ADSBMessage) {
        match message {
            ADSBMessage::AdsbRawMessage(raw_message) => self.process_aircraft_raw(raw_message),
            ADSBMessage::AdsbBeastMessage(adsb_beast_message) => {
                self.process_aircraft_beast(adsb_beast_message)
            }
            ADSBMessage::AircraftJSON(aircraft_json) => self.process_aircraft_json(aircraft_json),
            ADSBMessage::JSONMessage(json_message) => self.process_json_message(json_message),
        }
    }

    pub fn process_json_message(&mut self, message: JSONMessage) {
        if self
            .airplanes
            .contains_key(&message.transponder_hex.get_transponder_hex_as_string())
        {
            let airplane = self
                .airplanes
                .get_mut(&message.transponder_hex.get_transponder_hex_as_string())
                .unwrap();
            //airplane.update(message);
        } else {
            // let mut airplane = Airplane::new(message);
            // airplane.timeout_in_seconds = self.timeout_in_seconds;
            // self.airplanes.insert(message.transponder_hex.get_transponder_hex_as_string(), airplane);
        }
    }

    pub fn process_aircraft_json(&mut self, message: AircraftJSON) {
        for aircraft in message.aircraft {
            self.process_json_message(aircraft);
        }
    }

    pub fn process_aircraft_raw(&mut self, message: AdsbRawMessage) {
        unimplemented!("RawMessage")
    }

    pub fn process_aircraft_beast(&mut self, message: AdsbBeastMessage) {
        self.process_aircraft_raw(message.raw_message)
    }
}
