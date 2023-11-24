use deku::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

use super::{helper_functions::aircraft_identification_read, typecoding::TypeCoding};

#[derive(Serialize, Deserialize, DekuRead, Debug, Clone, Eq, PartialEq)]
pub struct Identification {
    pub tc: TypeCoding,

    #[deku(bits = "3")]
    pub ca: u8,

    /// N-Number / Tail Number
    #[deku(reader = "aircraft_identification_read(deku::rest)")]
    pub cn: String,
}

impl fmt::Display for Identification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Type code:      {}{}", self.tc, self.ca)?;
        writeln!(f, "  Callsign:       {}", self.cn)?;
        Ok(())
    }
}
