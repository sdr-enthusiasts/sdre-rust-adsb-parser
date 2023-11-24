use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

impl fmt::Display for TargetStateAndStatusInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "    Target altitude:   MCP, {} ft", self.altitude)?;
        writeln!(f, "    Altimeter setting: {} millibars", self.qnh)?;
        if self.is_heading {
            writeln!(f, "    Target heading:    {}", self.heading)?;
        }
        if self.tcas {
            write!(f, "    ACAS:              operational ")?;
            if self.autopilot {
                write!(f, "autopilot ")?;
            }
            if self.vnac {
                write!(f, "vnav ")?;
            }
            if self.alt_hold {
                write!(f, "altitude-hold ")?;
            }
            if self.approach {
                write!(f, " approach")?;
            }
            writeln!(f)?;
        } else {
            writeln!(f, "    ACAS:              NOT operational")?;
        }
        writeln!(f, "    NACp:              {}", self.nacp)?;
        writeln!(f, "    NICbaro:           {}", self.nicbaro)?;
        writeln!(f, "    SIL:               {} (per sample)", self.sil)?;
        writeln!(f, "    QNH:               {} millibars", self.qnh)?;
        Ok(())
    }
}

/// Target State and Status (§2.2.3.2.7.1)
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, PartialEq)]
pub struct TargetStateAndStatusInformation {
    // TODO Support Target State and Status defined in DO-260A, ADS-B Version=1
    // TODO Support reserved 2..=3
    #[deku(bits = "2")]
    pub subtype: u8,
    #[deku(bits = "1")]
    pub is_fms: bool,
    #[deku(
        bits = "12",
        endian = "big",
        map = "|altitude: u32| -> Result<_, DekuError> {Ok(if altitude > 1 {(altitude - 1) * 32} else {0} )}"
    )]
    pub altitude: u32,
    #[deku(
        bits = "9",
        endian = "big",
        map = "|qnh: u32| -> Result<_, DekuError> {if qnh == 0 { Ok(0.0) } else { Ok(800.0 + ((qnh - 1) as f32) * 0.8)}}"
    )]
    pub qnh: f32,
    #[deku(bits = "1")]
    pub is_heading: bool,
    #[deku(
        bits = "9",
        endian = "big",
        map = "|heading: u16| -> Result<_, DekuError> {Ok(heading as f32 * 180.0 / 256.0)}"
    )]
    pub heading: f32,
    #[deku(bits = "4")]
    pub nacp: u8,
    #[deku(bits = "1")]
    pub nicbaro: u8,
    #[deku(bits = "2")]
    pub sil: u8,
    #[deku(bits = "1")]
    pub mode_validity: bool,
    #[deku(bits = "1")]
    pub autopilot: bool,
    #[deku(bits = "1")]
    pub vnac: bool,
    #[deku(bits = "1")]
    pub alt_hold: bool,
    #[deku(bits = "1")]
    pub imf: bool,
    #[deku(bits = "1")]
    pub approach: bool,
    #[deku(bits = "1")]
    pub tcas: bool,
    #[deku(bits = "1")]
    #[deku(pad_bits_after = "2")] // reserved
    pub lnav: bool,
}
