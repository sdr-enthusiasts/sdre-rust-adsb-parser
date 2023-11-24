use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

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

impl fmt::Display for DataLinkCapability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Continuation:  {}", self.continuation_flag)?;
        writeln!(f, "  Overlay:       {}", self.overlay_command_capability)?;
        writeln!(f, "  ACAS:          {}", self.acas)?;
        writeln!(
            f,
            "  Mode S subnetwork version number: {}",
            self.mode_s_subnetwork_version_number
        )?;
        writeln!(
            f,
            "  Transponder enhanced protocol indicator: {}",
            self.transponder_enhanced_protocol_indicator
        )?;
        writeln!(
            f,
            "  Mode S specific services capability: {}",
            self.mode_s_specific_services_capability
        )?;
        writeln!(
            f,
            "  Uplink ELM average throughput capability: {}",
            self.uplink_elm_average_throughput_capability
        )?;
        writeln!(f, "  Downlink ELM:  {}", self.downlink_elm)?;
        writeln!(
            f,
            "  Aircraft identification capability: {}",
            self.aircraft_identification_capability
        )?;
        writeln!(
            f,
            "  Squitter capability subfield: {}",
            self.squitter_capability_subfield
        )?;
        writeln!(
            f,
            "  Surveillance identifier code: {}",
            self.surveillance_identifier_code
        )?;
        writeln!(
            f,
            "  Common usage GICB capability report: {}",
            self.common_usage_gicb_capability_report
        )?;
        writeln!(f, "  Reserved ACAS: {}", self.reserved_acas)?;
        writeln!(f, "  Bit array:     {:16b}", self.bit_array)?;
        Ok(())
    }
}
