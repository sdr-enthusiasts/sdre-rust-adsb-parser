use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Uplink / Downlink
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "1")]
pub enum KE {
    DownlinkELMTx = 0,
    UplinkELMAck = 1,
}

impl fmt::Display for KE {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            KE::DownlinkELMTx => write!(f, "downlink ELM transmission"),
            KE::UplinkELMAck => write!(f, "uplink ELM acknowledgement"),
        }
    }
}
