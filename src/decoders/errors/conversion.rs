// Copyright (c) 2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// This file contains the error types for the conversion errors.

use custom_error::custom_error;

custom_error! {pub ConversionError
    ReservedIsNotZero{source_name: String} = "{source_name} reserved field(s) are not 0",
    NotImplemented{source_name: String} = "{source_name} is not implemented",
    LongitudeIsNone = "Calculated longitude is None",
    LatitudeIsNone = "Calculated latitude is None",
    UnknownMessageType{message_me: String, me_type: String} = "Unknown message type {message_me} for {me_type}",
    UnknownADSBVersion = "Unknown ADSB version",
    UnknownCapabilityClass = "Unknown capability class",
    UnknownOperationalMode = "Unknown operational mode",
    LatitudeOrLongitudeIsZero{lat: f64, lon: f64} = "Latitude or longitude is 0.0. Latitude: {lat}, Longitude: {lon}. Unable to calculate position",
    UnableToCalculatePosition = "Unable to calculate position from Even/Odd CPR, supplied reference position, and/or previous aircraft position used as reference position",
}
