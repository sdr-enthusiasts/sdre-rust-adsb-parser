use deku::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, PartialEq)]
pub struct MLATBeast {}
