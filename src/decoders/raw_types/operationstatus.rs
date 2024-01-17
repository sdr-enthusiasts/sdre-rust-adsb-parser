// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::decoders::raw::NewAdsbRawMessage;
    use crate::decoders::raw_types::adsbversion::ADSBVersion;
    use crate::decoders::raw_types::capabilityclassairborne::CapabilityClassAirborne;
    use crate::decoders::raw_types::df::DF;
    use crate::decoders::raw_types::me::ME;
    use crate::decoders::raw_types::operationalmode::OperationalMode;
    use crate::decoders::raw_types::operationstatusairborne::OperationStatusAirborne;

    #[test]
    fn test_operation_status() {
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
                reserved_recv_atc_service: false,
                single_antenna_flag: true,
                system_design_assurance: 2,
            },
            version_number: ADSBVersion::ADSBVersion2,
            nic_supplement_a: 0,
            navigational_accuracy_category: 10,
            geometric_vertical_accuracy: 2,
            source_integrity_level: 3,
            barometric_altitude_integrity: 1,
            horizontal_reference_direction: 0,
            sil_supplement: 0,
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
}
