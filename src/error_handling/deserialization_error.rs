// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use custom_error::custom_error;

use deku::error::DekuError;
use hex::FromHexError;
use serde_json::Error as SerdeError;
use std::error::Error;

use super::adsb_beast_error::ADSBBeastError;
use super::adsb_raw_error::ADSBRawError;

custom_error! {pub WrongType
    WrongTypeForAircraft{message: String} = "Wrong type: {message}",
}

#[derive(Debug)]
pub enum DeserializationError {
    SerdeError(serde_json::error::Error),
    DekuError(deku::error::DekuError),
    HexError(FromHexError),
    ADSBRawError(ADSBRawError),
    ADSBBeastError(ADSBBeastError),
    StardardError(Box<dyn Error + Send + Sync>),
    WrongType(WrongType),
    CombinedError(Vec<DeserializationError>),
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeserializationError::SerdeError(e) => write!(f, "Serde error: {e}"),
            DeserializationError::DekuError(e) => write!(f, "Deku error: {e}"),
            DeserializationError::HexError(e) => write!(f, "Hex error: {e}"),
            DeserializationError::ADSBRawError(e) => write!(f, "ADSB Raw error: {e}"),
            DeserializationError::ADSBBeastError(e) => write!(f, "ADSB Beast error: {e}"),
            DeserializationError::StardardError(e) => write!(f, "Standard error: {e}"),
            DeserializationError::WrongType(e) => write!(f, "Wrong type error: {e}"),
            DeserializationError::CombinedError(e) => {
                for error in e {
                    writeln!(f, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl From<FromHexError> for DeserializationError {
    fn from(value: FromHexError) -> Self {
        DeserializationError::HexError(value)
    }
}

impl From<SerdeError> for DeserializationError {
    fn from(value: SerdeError) -> Self {
        DeserializationError::SerdeError(value)
    }
}

impl From<DekuError> for DeserializationError {
    fn from(value: DekuError) -> Self {
        DeserializationError::DekuError(value)
    }
}

impl From<ADSBRawError> for DeserializationError {
    fn from(value: ADSBRawError) -> Self {
        DeserializationError::ADSBRawError(value)
    }
}

impl From<Box<dyn Error + Send + Sync>> for DeserializationError {
    fn from(value: Box<dyn Error + Send + Sync>) -> Self {
        DeserializationError::StardardError(value)
    }
}

impl From<WrongType> for DeserializationError {
    fn from(value: WrongType) -> Self {
        DeserializationError::WrongType(value)
    }
}

impl From<ADSBBeastError> for DeserializationError {
    fn from(value: ADSBBeastError) -> Self {
        DeserializationError::ADSBBeastError(value)
    }
}
