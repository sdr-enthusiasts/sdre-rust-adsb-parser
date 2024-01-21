// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::{decoders::helpers::cpr_calculators::Position, MessageResult};

use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

use super::{
    helpers::prettyprint::{
        pretty_print_field, pretty_print_field_from_option, pretty_print_label,
    },
    json_types::{
        adsbversion::ADSBVersion,
        altimeter::Altimeter,
        altitude::Altitude,
        barorate::BaroRate,
        calculatedbestflightid::CalculatedBestFlightID,
        dbflags::DBFlags,
        emergency::Emergency,
        emmittercategory::EmitterCategory,
        heading::Heading,
        lastknownposition::LastKnownPosition,
        latitude::Latitude,
        longitude::Longitude,
        messagetype::MessageType,
        meters::{Meters, NauticalMiles},
        mlat::MLATFields,
        nacp::NavigationIntegrityCategory,
        nacv::NavigationAccuracyVelocity,
        navigationmodes::NavigationModes,
        receivedmessages::ReceivedMessages,
        secondsago::SecondsAgo,
        signalpower::SignalPower,
        sil::SourceIntegrityLevel,
        sourceintegritylevel::SourceIntegrityLevelType,
        speed::Speed,
        timestamp::TimeStamp,
        tisb::TiSB,
        transponderhex::TransponderHex,
    },
    raw_types::{df::DF, me::ME},
    rawtojson::{
        update_airborne_velocity, update_aircraft_identification,
        update_aircraft_position_airborne, update_aircraft_position_surface,
        update_aircraft_status, update_from_no_position, update_operational_status,
        update_target_state_and_status_information,
    },
};

/// Trait for performing a decode if you wish to apply it to types other than the defaults done in this library.
///
/// The originating data must be in JSON format and have support for providing a `str`, and will not consume the source.
///
/// This is intended for specifically decoding to `JSONMessage`.
pub trait NewJSONMessage {
    fn to_json(&self) -> MessageResult<JSONMessage>;
}

/// Implementing `.to_json()` for the type `String`.
///
/// This does not consume the `String`.
impl NewJSONMessage for String {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_json()` for the type `str`.
///
/// This does not consume the `str`.
impl NewJSONMessage for str {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_str(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_json()` for the type `[u8]`.
///
/// This does not consume the `[u8]`.

impl NewJSONMessage for [u8] {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_slice(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Supporting `.to_json()` for the type `Vec<u8>`.
///
/// This does not consume the `Vec<u8>`.

impl NewJSONMessage for Vec<u8> {
    fn to_json(&self) -> MessageResult<JSONMessage> {
        match serde_json::from_slice(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl fmt::Display for JSONMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string().unwrap())
    }
}

impl JSONMessage {
    pub fn new(icao: String) -> JSONMessage {
        JSONMessage {
            transponder_hex: icao.into(),
            timestamp: get_timestamp(),
            last_time_seen: (0.0).into(),
            message_type: MessageType::ADSBICAO, // FIXME: this feels wrong. How do we handle data that is UAT?
            ..Default::default()
        }
    }
    /// Converts `JSONMessage` to `String`.
    pub fn to_string(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    /// Function to pretty print the JSONMessage.
    /// Units will be in Feet, Nautical Miles, and hPa.
    /// The units are not translated from the default units from the original data.
    ///
    /// return type is a String
    pub fn pretty_print(&self) -> String {
        self.pretty_print_with_options()
    }

    fn pretty_print_with_options(&self) -> String {
        // Go through each field and print it out
        let mut output: String = String::new();
        pretty_print_label("JSON Message", &mut output);
        pretty_print_field("Timestamp", &self.timestamp, &mut output);
        pretty_print_field("Message Type", &self.message_type, &mut output);
        pretty_print_field("Messages", &self.number_of_received_messages, &mut output);
        pretty_print_label("Aircraft Identification", &mut output);
        pretty_print_field("Transponder Hex:", &self.transponder_hex, &mut output);
        pretty_print_field_from_option(
            "Transponder Squawk Code",
            &self.transponder_squawk_code,
            &mut output,
        );
        pretty_print_field_from_option(
            "Calculated Best Flight ID",
            &self.calculated_best_flight_id,
            &mut output,
        );
        pretty_print_field_from_option(
            "Aircraft Registration from Database",
            &self.aircraft_registration_from_database,
            &mut output,
        );
        pretty_print_field_from_option(
            "Aircraft Type from Database",
            &self.aircraft_type_from_database,
            &mut output,
        );
        pretty_print_field_from_option(
            "Aircraft Type from Database Long Name",
            &self.aircraft_type_from_database_long_name,
            &mut output,
        );
        pretty_print_label("Aircraft Position, Altitude and Speed", &mut output);
        pretty_print_field_from_option("Latitude", &self.latitude, &mut output);
        pretty_print_field_from_option("Longitude", &self.longitude, &mut output);
        pretty_print_field_from_option("Ground Speed", &self.ground_speed, &mut output);
        pretty_print_field_from_option(
            "Indicator Air Speed",
            &self.indicated_air_speed,
            &mut output,
        );
        pretty_print_field_from_option("True Air Speed", &self.true_air_speed, &mut output);
        pretty_print_field_from_option(
            "True Track Over Ground",
            &self.true_track_over_ground,
            &mut output,
        );
        pretty_print_field_from_option("True Heading", &self.true_heading, &mut output);
        pretty_print_field_from_option("Magnetic Heading", &self.magnetic_heading, &mut output);
        pretty_print_field_from_option(
            "Last Known Position",
            &self.last_known_position,
            &mut output,
        );
        pretty_print_field_from_option("Calculated Track", &self.calculated_track, &mut output);
        pretty_print_field("Last Time Seen", &self.last_time_seen, &mut output);
        pretty_print_field_from_option(
            "Last Time Seen Position and Altitude",
            &self.last_time_seen_pos_and_alt,
            &mut output,
        );
        pretty_print_field_from_option(
            "Barometric Altitude",
            &self.barometric_altitude,
            &mut output,
        );
        pretty_print_field_from_option(
            "Barometric Altitude Rate",
            &self.barometric_altitude_rate,
            &mut output,
        );
        pretty_print_field_from_option("Geometric Altitude", &self.geometric_altitude, &mut output);
        pretty_print_field_from_option(
            "Geometric Altitude Rate",
            &self.geometric_altitude_rate,
            &mut output,
        );

        pretty_print_label("Autopilot Settings", &mut output);
        pretty_print_field_from_option(
            "Flight Management System Selected Altitude",
            &self.flight_management_system_selected_altitude,
            &mut output,
        );
        pretty_print_field_from_option(
            "Autopilot Selected Altitude",
            &self.autopilot_selected_altitude,
            &mut output,
        );

        pretty_print_field_from_option(
            "Autopilot Selected Heading",
            &self.autopilot_selected_heading,
            &mut output,
        );
        // loop through all of the autopilot modes and print them out
        if let Some(autopilot_modes) = &self.autopilot_modes {
            for autopilot_mode in autopilot_modes {
                pretty_print_field("Autopilot Mode", &autopilot_mode, &mut output);
            }
        }
        pretty_print_field_from_option("Selected Altimeter", &self.selected_altimeter, &mut output);

        pretty_print_label("ADSB Information", &mut output);

        pretty_print_field_from_option("Category", &self.category, &mut output);
        pretty_print_field_from_option("DB Flags", &self.db_flags, &mut output);
        pretty_print_field_from_option("Emergency", &self.emergency, &mut output);
        pretty_print_field_from_option(
            "Geometric Vertical Accuracy",
            &self.geometric_verticle_accuracy,
            &mut output,
        );
        pretty_print_field_from_option(
            "Navigation Accuracy Position",
            &self.navigation_accuracy_position,
            &mut output,
        );
        pretty_print_field_from_option(
            "Navigation Accuracy Velocity",
            &self.navigation_accuracy_velocity,
            &mut output,
        );
        pretty_print_field_from_option(
            "Navigation Integrity Category",
            &self.naviation_integrity_category,
            &mut output,
        );
        pretty_print_field_from_option(
            "Barometric Altitude Integrity Category",
            &self.barometeric_altitude_integrity_category,
            &mut output,
        );
        pretty_print_field_from_option(
            "Aircraft Direction from Receiving Station",
            &self.aircraft_direction_from_receiving_station,
            &mut output,
        );

        pretty_print_field_from_option(
            "Aircraft Distance from Receiving Station",
            &self.aircract_distance_from_receiving_station,
            &mut output,
        );
        pretty_print_field_from_option(
            "Radius of Containment",
            &self.radius_of_containment,
            &mut output,
        );
        pretty_print_field_from_option("RSSI", &self.rssi, &mut output);
        pretty_print_field_from_option(
            "System Design Assurance",
            &self.system_design_assurance,
            &mut output,
        );

        pretty_print_field_from_option(
            "Source Integrity Level",
            &self.source_integrity_level,
            &mut output,
        );
        pretty_print_field_from_option("Source Integrity Level Type", &self.sil_type, &mut output);
        pretty_print_field_from_option(
            "Flight Status Special Position ID Bit",
            &self.flight_status_special_position_id_bit,
            &mut output,
        );
        pretty_print_field_from_option("Flight Status", &self.flight_status, &mut output);

        pretty_print_field_from_option("Version", &self.version, &mut output);
        // loop through TISB and print

        for tisb_message in &self.tisb {
            pretty_print_field("TISB Message", tisb_message, &mut output);
        }

        // loop through MLAT and print

        for mlat_message in &self.mlat {
            pretty_print_field("MLAT Message", mlat_message, &mut output);
        }

        output
    }

    /// Converts `JSONMessage` to `String` and appends a `\n` to the end.
    pub fn to_string_newline(&self) -> MessageResult<String> {
        match serde_json::to_string(self) {
            Err(to_string_error) => Err(to_string_error.into()),
            Ok(string) => Ok(format!("{}\n", string)),
        }
    }

    /// Converts `JSONMessage` to a `String` encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes(&self) -> MessageResult<Vec<u8>> {
        match self.to_string() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    /// Converts `JSONMessage` to a `String` terminated with a `\n` and encoded as bytes.
    ///
    /// The output is returned as a `Vec<u8>`.
    pub fn to_bytes_newline(&self) -> MessageResult<Vec<u8>> {
        match self.to_string_newline() {
            Err(conversion_failed) => Err(conversion_failed),
            Ok(string) => Ok(string.into_bytes()),
        }
    }

    pub fn update_from_json(&mut self, json_message: &JSONMessage) {
        // update only if the time stamp on json_message is newer than the one we have
        if json_message.timestamp > self.timestamp {
            *self = json_message.clone();
        } else {
            warn!(
                "Not updating JSONMessage because the timestamp is older than the one we have. {} < {}",
                json_message.timestamp, self.timestamp
            );
        }
    }

    pub fn update_from_df(
        &mut self,
        raw_adsb: &DF,
        reference_positon: &Position,
    ) -> Result<(), String> {
        if let DF::ADSB(adsb) = raw_adsb {
            match &adsb.me {
                ME::AirborneVelocity(velocity) => update_airborne_velocity(self, velocity),
                ME::NoPosition(no_position) => {
                    update_from_no_position(self, no_position);
                }
                ME::AircraftIdentification(id) => {
                    update_aircraft_identification(self, id);
                }
                ME::SurfacePosition(surfaceposition) => {
                    return update_aircraft_position_surface(
                        self,
                        surfaceposition,
                        reference_positon,
                    )
                }
                ME::AirbornePositionGNSSAltitude(altitude)
                | ME::AirbornePositionBaroAltitude(altitude) => {
                    let baro_altitude = matches!(adsb.me, ME::AirbornePositionBaroAltitude(_));
                    return update_aircraft_position_airborne(
                        self,
                        altitude,
                        baro_altitude,
                        reference_positon,
                    );
                }
                ME::Reserved0(_) => return Err("Reserved0 is not implemented....".into()),
                ME::SurfaceSystemStatus(_) => {
                    return Err("SurfaceSystemStatus is not implemented....".into())
                }
                ME::Reserved1(_) => return Err("Reserved1 is not implemented....".into()),
                ME::AircraftStatus(status) => update_aircraft_status(self, status),
                ME::TargetStateAndStatusInformation(target_state_and_status_information) => {
                    update_target_state_and_status_information(
                        self,
                        target_state_and_status_information,
                    );
                }
                ME::AircraftOperationalCoordination(_) => {
                    return Err("AircraftOperationalCoordination is not implemented....".into())
                }
                ME::AircraftOperationStatus(operation_status) => {
                    return update_operational_status(self, operation_status);
                }
            }
        }

        // Reset the last time seen to "now". When the serializer is fixed properly
        self.last_time_seen = (0.0).into();
        self.timestamp = get_timestamp();

        Ok(())
    }
}

// Not all messages have a timestamp, so we'll use the current time if one isn't provided.
pub fn get_timestamp() -> TimeStamp {
    match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => TimeStamp::from(n.as_secs_f64()),
        Err(_) => TimeStamp::default(),
    }
}

// https://github.com/wiedehopf/readsb/blob/dev/README-json.md

/// The JSON message format.
/// This is for a single aircraft of JSON data.
/// TODO: There is a metric load of "Option" types here. 99.9% of the time they are present in
/// the payload. It may be well worth it to remove the Option types and just use the default,
/// or see if the message structure is consistent if certain fields are missing and create a different
/// struct for those messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Default)]
#[serde(deny_unknown_fields)]
pub struct JSONMessage {
    /// The timestamp of the message in seconds since the epoch.
    #[serde(rename = "now", default = "get_timestamp")]
    pub timestamp: TimeStamp,
    /// The Flight Status bit field. 2.2.3.2.3.2
    #[serde(skip_serializing_if = "Option::is_none", rename = "alert")]
    pub flight_status: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_baro")]
    /// Aircraft altitude reported from the barometric altimeter.
    pub barometric_altitude: Option<Altitude>,
    /// Aircraft altitude reported from the GNSS/INS system on the aircraft
    #[serde(skip_serializing_if = "Option::is_none", rename = "alt_geom")]
    pub geometric_altitude: Option<Altitude>,
    /// Rate of change in the barometric altitude in feet per minute.
    #[serde(skip_serializing_if = "Option::is_none", rename = "baro_rate")]
    pub barometric_altitude_rate: Option<BaroRate>,
    /// Emitter category to identify the aircraft or vehicle class. 2.2.3.2.5.2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<EmitterCategory>,
    /// Wiedehopf's aircraft.json indicator for interesting aircraft.
    /// Possible Values are military, interesting, PIA and LADD.
    #[serde(skip_serializing, rename = "dbFlags")]
    pub db_flags: Option<DBFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ADS-B emergency/priority status, a superset of the 7x00 squawks (2.2.3.2.7.8.1.1)
    /// (none, general, lifeguard, minfuel, nordo, unlawful, downed, reserved)
    pub emergency: Option<Emergency>,
    /// The aircraft callsign, Flight Name, or Tail Number. Most likely the id used by air traffic control.
    /// to interact with the flight. (2.2.8.2.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "flight")]
    pub calculated_best_flight_id: Option<CalculatedBestFlightID>,
    /// Rate of change of geometric (GNSS / INS) altitude, feet/minute
    #[serde(skip_serializing_if = "Option::is_none", rename = "geom_rate")]
    pub geometric_altitude_rate: Option<BaroRate>,
    /// Ground speed in knots.
    #[serde(skip_serializing_if = "Option::is_none", rename = "gs")]
    pub ground_speed: Option<Speed>,
    /// Indicated Air speed.
    // TODO: what is the source of this?
    #[serde(skip_serializing_if = "Option::is_none", rename = "ias")]
    pub indicated_air_speed: Option<Speed>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "gva")]
    pub geometric_verticle_accuracy: Option<u8>, // FIXME: I doubt this is right
    /// The transponder hex identifier of the aircraft.
    #[serde(rename = "hex")]
    pub transponder_hex: TransponderHex,
    /// {lat, lon, nic, rc, seen_pos} when the regular lat and lon are older than 60 seconds they are no longer considered valid,
    /// this will provide the last position and show the age for the last position. aircraft will only be in the aircraft json
    /// if a position has been received in the last 60 seconds or if any message has been received in the last 30 seconds.
    /// TODO: set this during pruning of data
    #[serde(skip_serializing_if = "Option::is_none", rename = "lastPosition")]
    pub last_known_position: Option<LastKnownPosition>,
    /// The aircraft latitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "lat")]
    pub latitude: Option<Latitude>,
    /// The aircraft longitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "lon")]
    pub longitude: Option<Longitude>,
    /// The number of messages received for this aircraft.
    #[serde(rename = "messages")]
    pub number_of_received_messages: ReceivedMessages,
    /// list of fields derived from MLAT data
    pub mlat: Vec<MLATFields>,
    /// Navigation Accuracy for Position (2.2.5.1.35)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_p")]
    pub navigation_accuracy_position: Option<NavigationIntegrityCategory>, // FIXME: I doubt this is right
    /// Navigation Accuracy for Velocity (2.2.5.1.19)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nac_v")]
    pub navigation_accuracy_velocity: Option<NavigationAccuracyVelocity>, // FIXME: I doubt this is right
    /// selected altitude from the Mode Control Panel / Flight Control Unit (MCP/FCU) or equivalent equipment
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_mcp")]
    pub autopilot_selected_altitude: Option<Altitude>,
    /// selected heading (True or Magnetic is not defined in DO-260B, mostly Magnetic as that is the de facto standard) (2.2.3.2.7.1.3.7)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_heading")]
    pub autopilot_selected_heading: Option<Heading>,
    /// selected altitude from the Flight Manaagement System (FMS) (2.2.3.2.7.1.3.3)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_altitude_fms")]
    pub flight_management_system_selected_altitude: Option<Altitude>,
    /// set of engaged automation modes: 'autopilot', 'vnav', 'althold', 'approach', 'lnav', 'tcas'
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_modes")]
    pub autopilot_modes: Option<Vec<NavigationModes>>,
    /// altimeter setting (QFE or QNH/QNE), hPa
    #[serde(skip_serializing_if = "Option::is_none", rename = "nav_qnh")]
    pub selected_altimeter: Option<Altimeter>,
    /// Navigation Integrity Category (2.2.3.2.7.2.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic")]
    pub naviation_integrity_category: Option<NavigationIntegrityCategory>, // FIXME: I doubt this is right
    /// Navigation Integrity Category for Barometric Altitude (2.2.5.1.35)
    #[serde(skip_serializing_if = "Option::is_none", rename = "nic_baro")]
    pub barometeric_altitude_integrity_category: Option<u8>, // FIXME: I doubt this is right
    #[serde(skip_serializing_if = "Option::is_none", rename = "r")]
    /// Wiedehopf's aircraft.json aircraft registration pulled from database
    pub aircraft_registration_from_database: Option<String>,
    /// distance from supplied center point in nmi
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dir")]
    pub aircraft_direction_from_receiving_station: Option<Heading>,
    /// true direction of the aircraft from the supplied center point (degrees)
    #[serde(skip_serializing_if = "Option::is_none", rename = "r_dst")]
    pub aircract_distance_from_receiving_station: Option<NauticalMiles>,
    /// Radius of Containment, meters; a measure of position integrity derived from NIC & supplementary bits. (2.2.3.2.7.2.6, Table 2-69)
    #[serde(skip_serializing_if = "Option::is_none", rename = "rc")]
    pub radius_of_containment: Option<Meters>,
    /// recent average RSSI (signal power), in dbFS; this will always be negative.
    /// This value is always (?) present in payloads from readsb, but will be missing in payloads we generate
    /// from raw/beast data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<SignalPower>,
    /// System Design Assurance (2.2.3.2.7.2.4.6)
    #[serde(skip_serializing_if = "Option::is_none", rename = "sda")]
    pub system_design_assurance: Option<i32>, // FIXME: I doubt this is right
    /// how long ago (in seconds before "now") a message was last received from this aircraft
    #[serde(rename = "seen")]
    pub last_time_seen: SecondsAgo, // FIXME: when doing any serialization this value needs to be referenced to the current time
    /// how long ago (in seconds before "now") the position was last updated
    #[serde(skip_serializing_if = "Option::is_none", rename = "seen_pos")]
    pub last_time_seen_pos_and_alt: Option<f32>,
    /// Source Integity Level (2.2.5.1.40)
    #[serde(skip_serializing_if = "Option::is_none", rename = "sil")]
    pub source_integrity_level: Option<SourceIntegrityLevel>, // FIXME: I doubt this is right
    /// interpretation of SIL: unknown, perhour, persample
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sil_type: Option<SourceIntegrityLevelType>, // FIXME: I doubt this is right
    /// Flight status special position identification bit (2.2.3.2.3.2)
    #[serde(skip_serializing_if = "Option::is_none", rename = "spi")]
    pub flight_status_special_position_id_bit: Option<u8>, // FIXME: I doubt this is right
    /// Mode A code (Squawk), encoded as 4 octal digits
    #[serde(skip_serializing_if = "Option::is_none", rename = "squawk")]
    pub transponder_squawk_code: Option<String>, // TODO: This does not serialize with leading 0s right. It should always be at least 4 digits
    /// wiedehopf's aircraft.json aircraft type pulled from database
    #[serde(skip_serializing_if = "Option::is_none", rename = "t")]
    pub aircraft_type_from_database: Option<String>,
    /// wiedehopf's aircraft.json aircraft type pulled from database, long name
    #[serde(skip_serializing_if = "Option::is_none", rename = "desc")]
    pub aircraft_type_from_database_long_name: Option<String>,
    /// list of fields derived from TIS-B data
    pub tisb: Vec<TiSB>,
    /// true track over ground in degrees (0-359)
    #[serde(skip_serializing_if = "Option::is_none", rename = "track")]
    pub true_track_over_ground: Option<Heading>,
    /// calculated track?
    #[serde(skip_serializing_if = "Option::is_none", rename = "calc_track")]
    pub calculated_track: Option<Heading>,
    /// Heading, degrees clockwise from true north (usually only transmitted on ground, in the air usually derived from the magnetic heading using magnetic model WMM2020)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_heading: Option<Heading>,
    /// type of underlying messages / best source of current data for this position / aircraft
    #[serde(rename = "type")]
    pub message_type: MessageType,
    /// ADS-B Version Number 0, 1, 2 (3-7 are reserved) (2.2.3.2.7.5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ADSBVersion>,
    /// Magnetic heading
    #[serde(skip_serializing_if = "Option::is_none", rename = "mag_heading")]
    pub magnetic_heading: Option<Heading>,
    /// GPS Okay before this time.
    #[serde(skip_serializing_if = "Option::is_none", rename = "gpsOkBefore")]
    pub gps_ok_before: Option<TimeStamp>, // TODO: print out
    /// GPS Okay Latitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "gpsOkLat")]
    pub gps_ok_latitude: Option<Latitude>, // TODO: print out
    /// GPS Okay Longitude
    #[serde(skip_serializing_if = "Option::is_none", rename = "gpsOkLon")]
    pub gps_ok_longitude: Option<Longitude>, // TODO: print out
    /// True air speed
    #[serde(skip_serializing_if = "Option::is_none", rename = "tas")]
    pub true_air_speed: Option<Speed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_rate: Option<f32>, // TODO: print this out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f32>, // TODO: print this out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<u32>, // TODO: print this out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wd: Option<u32>, // TODO: print this out

    /// These are internal values that should never get serialized, but used for tracking raw even/odd positions

    #[serde(skip_serializing)]
    pub cpr_even_airborne: Option<super::raw_types::altitude::Altitude>,
    #[serde(skip_serializing)]
    pub cpr_odd_airborne: Option<super::raw_types::altitude::Altitude>,
    #[serde(skip_serializing)]
    pub last_cpr_even_update_time_airborne: Option<TimeStamp>,
    #[serde(skip_serializing)]
    pub last_cpr_odd_update_time_airborne: Option<TimeStamp>,

    #[serde(skip_serializing)]
    pub cpr_even_surface: Option<super::raw_types::surfaceposition::SurfacePosition>,
    #[serde(skip_serializing)]
    pub cpr_odd_surface: Option<super::raw_types::surfaceposition::SurfacePosition>,
    #[serde(skip_serializing)]
    pub last_cpr_even_update_time_surface: Option<TimeStamp>,
    #[serde(skip_serializing)]
    pub last_cpr_odd_update_time_surface: Option<TimeStamp>,
    #[serde(skip_serializing)]
    pub nic_supplement_a: Option<u8>,
    #[serde(skip_serializing)]
    pub nic_supplement_b: Option<u8>,
    #[serde(skip_serializing)]
    pub nic_supplement_c: Option<u8>,
    #[serde(skip_serializing)]
    pub airborne_type_code: Option<u8>,
    #[serde(skip_serializing)]
    pub surface_type_code: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecodeMessage;
    use std::fs::{read_dir, File};
    use std::io::BufRead;

    #[test]
    fn decode_directly_as_json() {
        // open all json_*.json files in test data. convert to JSONMessage and then back to string
        let test_data: std::fs::ReadDir = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: std::path::PathBuf = entry.path();
            if path.is_file() {
                let mut line_number: i32 = 1;
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file: File = File::open(path).unwrap();
                    let reader: std::io::BufReader<File> = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader
                        .lines()
                        .for_each(|l: Result<String, std::io::Error>| {
                            line = l.unwrap();

                            // if the line starts with anything but a {, skip it
                            if line.starts_with("{") && line.trim().len() > 0 {
                                // encode the line as JSONMessage
                                // remove the trailing newline and any other characters after the '}'
                                let final_message_to_process = line.trim().trim_end_matches(',');
                                assert!(
                                    final_message_to_process.ends_with("}"),
                                    "Line {} in file does not end with a curly bracket",
                                    line_number
                                );
                                let json_message = final_message_to_process.to_json();

                                println!("JSONMessage: {:?}", json_message,);

                                assert!(
                                    json_message.is_ok(),
                                    "Failed to decode JSONMessage {:?}",
                                    final_message_to_process
                                );
                            } else {
                                println!("Skipping line {}", line_number);
                            }
                            line_number += 1;
                        });
                }
            }
        }
    }

    #[test]
    fn decode_json_message_as_aircraft_json() {
        let test_data: std::fs::ReadDir = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: std::path::PathBuf = entry.path();
            if path.is_file() {
                let mut line_number: i32 = 1;
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file: File = File::open(path).unwrap();
                    let reader: std::io::BufReader<File> = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader
                        .lines()
                        .for_each(|l: Result<String, std::io::Error>| {
                            line = l.unwrap();

                            // if the line starts with anything but a {, skip it
                            if line.starts_with("{") && line.trim().len() > 0 {
                                // encode the line as JSONMessage
                                // remove the trailing newline and any other characters after the '}'
                                let final_message_to_process = line.trim().trim_end_matches(',');
                                assert!(
                                    final_message_to_process.ends_with("}"),
                                    "Line {} in file does not end with a curly bracket",
                                    line_number
                                );
                                let json_message = final_message_to_process.decode_message();

                                assert!(
                                    json_message.is_ok(),
                                    "Failed to decode JSONMessage {:?}",
                                    final_message_to_process
                                );
                            }
                            line_number += 1;
                        });
                }
            }
        }
    }

    #[test]
    fn decode_json_message_individually() {
        // open all json_*.json files in test data. convert to JSONMessage and then back to string
        let test_data: std::fs::ReadDir = read_dir("test_data").unwrap();
        for entry in test_data {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: std::path::PathBuf = entry.path();
            if path.is_file() {
                let mut line_number: i32 = 1;
                let file_name: &str = path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with("json_") && file_name.ends_with(".json") {
                    println!("Processing file: {}", file_name);
                    let file: File = File::open(path).unwrap();
                    let reader: std::io::BufReader<File> = std::io::BufReader::new(file);

                    // read in a line
                    let mut line = String::new();
                    reader
                        .lines()
                        .for_each(|l: Result<String, std::io::Error>| {
                            line = l.unwrap();

                            // if the line starts with anything but a {, skip it
                            if line.starts_with("{") && line.trim().len() > 0 {
                                // encode the line as JSONMessage
                                // remove the trailing newline and any other characters after the '}'
                                let final_message_to_process = line.trim().trim_end_matches(',');
                                assert!(
                                    final_message_to_process.ends_with("}"),
                                    "Line {} in file does not end with a curly bracket",
                                    line_number
                                );
                                let json_message = final_message_to_process.decode_message();

                                assert!(
                                    json_message.is_ok(),
                                    "Failed to decode JSONMessage {:?}",
                                    final_message_to_process
                                );
                            }
                            line_number += 1;
                        });
                }
            }
        }
    }
}
