use crate::error_handling::adsb_raw_error::ADSBRawError;
use deku::error::DekuError;
use hex::FromHexError;
use serde_json::Error as SerdeError;
use std::error::Error;

#[derive(Debug)]
pub enum DeserializationError {
    SerdeError(serde_json::error::Error),
    DekuError(deku::error::DekuError),
    HexError(FromHexError),
    ADSBRawError(ADSBRawError),
    StardardError(Box<dyn Error + Send + Sync>),
}

impl std::fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeserializationError::SerdeError(e) => write!(f, "Serde error: {}", e),
            DeserializationError::DekuError(e) => write!(f, "Deku error: {}", e),
            DeserializationError::HexError(e) => write!(f, "Hex error: {}", e),
            DeserializationError::ADSBRawError(e) => write!(f, "ADSB Raw error: {}", e),
            DeserializationError::StardardError(e) => write!(f, "Standard error: {}", e),
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
