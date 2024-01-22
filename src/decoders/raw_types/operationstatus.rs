// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    adsbversion::ADSBVersion, capabilityclassairborne::CapabilityClassAirborne,
    capabilityclasssurface::CapabilityClassSurface, operationalmode::OperationalMode,
    operationstatusairborne::OperationStatusAirborne,
    operationstatussurface::OperationStatusSurface,
};

/// Aircraft Operational Status Subtype
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "3")]
pub enum OperationStatus {
    #[deku(id = "0")]
    Airborne(OperationStatusAirborne),

    #[deku(id = "1")]
    Surface(OperationStatusSurface),

    #[deku(id_pat = "2..=7")]
    Reserved(#[deku(bits = "5")] u8, [u8; 5]),
}

impl CapabilityClass {
    pub const fn is_reserved_zero(&self) -> bool {
        match self {
            CapabilityClass::Airborne(airborne) => airborne.is_reserved_zero(),
            CapabilityClass::Surface(surface) => surface.is_reserved_zero(),
            CapabilityClass::Unknown => false,
        }
    }
}

pub enum CapabilityClass {
    Airborne(CapabilityClassAirborne),
    Surface(CapabilityClassSurface),
    Unknown,
}

impl OperationStatus {
    pub fn is_airborne(&self) -> bool {
        matches!(self, OperationStatus::Airborne(_))
    }

    pub fn is_surface(&self) -> bool {
        matches!(self, OperationStatus::Surface(_))
    }

    pub fn is_reserved(&self) -> bool {
        matches!(self, OperationStatus::Reserved(_, _))
    }

    pub const fn is_reserved_zero(&self) -> bool {
        match self {
            OperationStatus::Reserved(_reserved0, _reserved1) => false,

            OperationStatus::Airborne(airborne) => airborne.is_reserved_zero(),

            OperationStatus::Surface(surface) => surface.is_reserved_zero(),
        }
    }

    pub fn get_adsb_version(&self) -> ADSBVersion {
        match self {
            OperationStatus::Airborne(airborne) => airborne.version_number,
            OperationStatus::Surface(surface) => surface.version_number,
            OperationStatus::Reserved(_, _) => ADSBVersion::Unknown,
        }
    }

    pub fn get_capability_class(&self) -> CapabilityClass {
        match self {
            OperationStatus::Airborne(airborne) => {
                CapabilityClass::Airborne(airborne.capability_class)
            }
            OperationStatus::Surface(surface) => CapabilityClass::Surface(surface.capability_class),
            OperationStatus::Reserved(_, _) => CapabilityClass::Unknown,
        }
    }

    pub fn get_operational_mode(&self) -> Option<OperationalMode> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.operational_mode),
            OperationStatus::Surface(surface) => Some(surface.operational_mode),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_nic_supplement_a(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.nic_supplement_a),
            OperationStatus::Surface(surface) => Some(surface.nic_supplement_a),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_navigational_accuracy_category(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.navigational_accuracy_category),
            OperationStatus::Surface(surface) => Some(surface.navigational_accuracy_category),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_geometric_vertical_accuracy(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.geometric_vertical_accuracy),
            OperationStatus::Surface(_surface) => None,
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_source_integrity_level(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.source_integrity_level),
            OperationStatus::Surface(surface) => Some(surface.source_integrity_level),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_barometric_altitude_integrity(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.barometric_altitude_integrity),
            OperationStatus::Surface(_surface) => None,
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_track_heading(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(_airborne) => None,
            OperationStatus::Surface(surface) => Some(surface.track_heading),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_horizontal_reference_direction(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.horizontal_reference_direction),
            OperationStatus::Surface(surface) => Some(surface.horizontal_reference_direction),
            OperationStatus::Reserved(_, _) => None,
        }
    }

    pub fn get_sil_supplement(&self) -> Option<u8> {
        match self {
            OperationStatus::Airborne(airborne) => Some(airborne.sil_supplement),
            OperationStatus::Surface(surface) => Some(surface.sil_supplement),
            OperationStatus::Reserved(_, _) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::adsbversion::ADSBVersion;
    use crate::decoders::raw_types::capabilityclassairborne::CapabilityClassAirborne;
    use crate::decoders::raw_types::capabilityclasssurface::CapabilityClassSurface;
    use crate::decoders::raw_types::df::DF;
    use crate::decoders::raw_types::me::ME;
    use crate::decoders::raw_types::operationalmode::OperationalMode;
    use crate::decoders::raw_types::operationstatusairborne::OperationStatusAirborne;

    #[test]
    fn test_operation_status_airborne() {
        let message = "8DABBD47F8230006004AB87B5E9E";
        let decoded = message.to_adsb_raw().unwrap();
        println!("Decoded {:?}", decoded);

        let expected = OperationStatus::Airborne(OperationStatusAirborne {
            capability_class: CapabilityClassAirborne {
                reserved0: 0,
                acas: 1,
                cdti: 0,
                reserved1: 0,
                arv: 1,
                ts: 1,
                tc: 0,
            },
            operational_mode: OperationalMode {
                reserved: 0,
                tcas_ra_active: false,
                ident_switch_active: false,
                reserved_recv_atc_service: 0,
                single_antenna_flag: true,
                system_design_assurance: 2,
            },
            reserved1: 0,
            version_number: ADSBVersion::ADSBVersion2,
            nic_supplement_a: 0,
            navigational_accuracy_category: 10,
            geometric_vertical_accuracy: 2,
            source_integrity_level: 3,
            barometric_altitude_integrity: 1,
            horizontal_reference_direction: 0,
            sil_supplement: 0,
            reserved: 0,
        });

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                ME::AircraftOperationStatus(operation_status) => {
                    assert_eq!(operation_status, expected);
                }
                _ => panic!("ME is not OperationStatus"),
            },
            _ => panic!("DF is not ADSB"),
        }
    }

    #[test]
    fn test_operational_status_surface() {
        let message = "8CA231A6F9004402874A38F61073";

        let decoded = message.to_adsb_raw().unwrap();

        let expected = OperationStatus::Surface(OperationStatusSurface {
            capability_class: CapabilityClassSurface {
                reserved0: 0,
                poa: 0,
                es1090: 0,
                b2_low: 0,
                uat_in: 0,
                nac_v: 2,
                nic_supplement_c: 0,
                reserved1: 0,
            },
            lw_codes: 4,
            operational_mode: OperationalMode {
                reserved: 0,
                tcas_ra_active: false,
                ident_switch_active: false,
                reserved_recv_atc_service: 0,
                single_antenna_flag: false,
                system_design_assurance: 2,
            },
            gps_antenna_offset: 135,
            version_number: ADSBVersion::ADSBVersion2,
            nic_supplement_a: 0,
            navigational_accuracy_category: 10,
            source_integrity_level: 3,
            track_heading: 1,
            horizontal_reference_direction: 0,
            sil_supplement: 0,
            reserved0: 0,
            reserved1: 0,
        });

        println!("Decoded {:?}", decoded);

        match decoded.df {
            DF::ADSB(adsb) => match adsb.me {
                ME::AircraftOperationStatus(operation_status) => {
                    assert_eq!(operation_status, expected);
                }
                _ => panic!("ME is not OperationStatus"),
            },
            _ => panic!("DF is not ADSB"),
        }
    }
}
