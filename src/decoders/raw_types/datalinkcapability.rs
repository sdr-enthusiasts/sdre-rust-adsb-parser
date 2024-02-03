// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::prelude::*;
use serde::{Deserialize, Serialize};

// FIXME: remove this lint when the issue is fixed
#[allow(clippy::struct_excessive_bools)]
/// To report the data link capability of the Mode S transponder/data link installation
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
pub struct DataLinkCapability {
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "5")] // reserved
    pub continuation_flag: bool,
    #[deku(bits = "1")]
    pub overlay_command_capability: bool,
    #[deku(bits = "1")]
    pub acas: bool,
    #[deku(bits = "7")]
    pub mode_s_subnetwork_version_number: u8,
    #[deku(bits = "1")]
    pub transponder_enhanced_protocol_indicator: bool,
    #[deku(bits = "1")]
    pub mode_s_specific_services_capability: bool,
    #[deku(bits = "3")]
    pub uplink_elm_average_throughput_capability: u8,
    #[deku(bits = "4")]
    pub downlink_elm: u8,
    #[deku(bits = "1")]
    pub aircraft_identification_capability: bool,
    #[deku(bits = "1")]
    pub squitter_capability_subfield: bool,
    #[deku(bits = "1")]
    pub surveillance_identifier_code: bool,
    #[deku(bits = "1")]
    pub common_usage_gicb_capability_report: bool,
    #[deku(bits = "4")]
    pub reserved_acas: u8,
    pub bit_array: u16,
}
