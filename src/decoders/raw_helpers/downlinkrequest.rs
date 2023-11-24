use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/// Type of `DownlinkRequest`
#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Copy, Eq, PartialEq)]
#[deku(type = "u8", bits = "5")]
pub enum DownlinkRequest {
    None = 0b00000,
    RequestSendCommB = 0b00001,
    CommBBroadcastMsg1 = 0b00100,
    CommBBroadcastMsg2 = 0b00101,
    #[deku(id_pat = "_")]
    Unknown,
}

impl fmt::Display for DownlinkRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DownlinkRequest::None => write!(f, "none"),
            DownlinkRequest::RequestSendCommB => write!(f, "request send Comm-B"),
            DownlinkRequest::CommBBroadcastMsg1 => write!(f, "Comm-B broadcast message 1"),
            DownlinkRequest::CommBBroadcastMsg2 => write!(f, "Comm-B broadcast message 2"),
            DownlinkRequest::Unknown => write!(f, "unknown"),
        }
    }
}
