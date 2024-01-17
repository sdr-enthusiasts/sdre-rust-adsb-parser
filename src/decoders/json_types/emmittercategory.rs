// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::decoders::raw_types::typecoding::TypeCoding;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "String")]
pub enum EmitterCategory {
    #[default]
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    C0,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
}

impl Serialize for EmitterCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            EmitterCategory::A0 => serializer.serialize_str("A0"),
            EmitterCategory::A1 => serializer.serialize_str("A1"),
            EmitterCategory::A2 => serializer.serialize_str("A2"),
            EmitterCategory::A3 => serializer.serialize_str("A3"),
            EmitterCategory::A4 => serializer.serialize_str("A4"),
            EmitterCategory::A5 => serializer.serialize_str("A5"),
            EmitterCategory::A6 => serializer.serialize_str("A6"),
            EmitterCategory::A7 => serializer.serialize_str("A7"),
            EmitterCategory::B0 => serializer.serialize_str("B0"),
            EmitterCategory::B1 => serializer.serialize_str("B1"),
            EmitterCategory::B2 => serializer.serialize_str("B2"),
            EmitterCategory::B3 => serializer.serialize_str("B3"),
            EmitterCategory::B4 => serializer.serialize_str("B4"),
            EmitterCategory::B5 => serializer.serialize_str("B5"),
            EmitterCategory::B6 => serializer.serialize_str("B6"),
            EmitterCategory::B7 => serializer.serialize_str("B7"),
            EmitterCategory::C0 => serializer.serialize_str("C0"),
            EmitterCategory::C1 => serializer.serialize_str("C1"),
            EmitterCategory::C2 => serializer.serialize_str("C2"),
            EmitterCategory::C3 => serializer.serialize_str("C3"),
            EmitterCategory::C4 => serializer.serialize_str("C4"),
            EmitterCategory::C5 => serializer.serialize_str("C5"),
            EmitterCategory::C6 => serializer.serialize_str("C6"),
            EmitterCategory::C7 => serializer.serialize_str("C7"),
            EmitterCategory::D0 => serializer.serialize_str("D0"),
            EmitterCategory::D1 => serializer.serialize_str("D1"),
            EmitterCategory::D2 => serializer.serialize_str("D2"),
            EmitterCategory::D3 => serializer.serialize_str("D3"),
            EmitterCategory::D4 => serializer.serialize_str("D4"),
            EmitterCategory::D5 => serializer.serialize_str("D5"),
            EmitterCategory::D6 => serializer.serialize_str("D6"),
            EmitterCategory::D7 => serializer.serialize_str("D7"),
        }
    }
}

impl TryFrom<String> for EmitterCategory {
    type Error = String;

    fn try_from(emitter_category: String) -> Result<Self, Self::Error> {
        match emitter_category.as_str() {
            "A0" => Ok(EmitterCategory::A0),
            "A1" => Ok(EmitterCategory::A1),
            "A2" => Ok(EmitterCategory::A2),
            "A3" => Ok(EmitterCategory::A3),
            "A4" => Ok(EmitterCategory::A4),
            "A5" => Ok(EmitterCategory::A5),
            "A6" => Ok(EmitterCategory::A6),
            "A7" => Ok(EmitterCategory::A7),
            "B0" => Ok(EmitterCategory::B0),
            "B1" => Ok(EmitterCategory::B1),
            "B2" => Ok(EmitterCategory::B2),
            "B3" => Ok(EmitterCategory::B3),
            "B4" => Ok(EmitterCategory::B4),
            "B5" => Ok(EmitterCategory::B5),
            "B6" => Ok(EmitterCategory::B6),
            "B7" => Ok(EmitterCategory::B7),
            "C0" => Ok(EmitterCategory::C0),
            "C1" => Ok(EmitterCategory::C1),
            "C2" => Ok(EmitterCategory::C2),
            "C3" => Ok(EmitterCategory::C3),
            "C4" => Ok(EmitterCategory::C4),
            "C5" => Ok(EmitterCategory::C5),
            "C6" => Ok(EmitterCategory::C6),
            "C7" => Ok(EmitterCategory::C7),
            "D0" => Ok(EmitterCategory::D0),
            "D1" => Ok(EmitterCategory::D1),
            "D2" => Ok(EmitterCategory::D2),
            "D3" => Ok(EmitterCategory::D3),
            "D4" => Ok(EmitterCategory::D4),
            "D5" => Ok(EmitterCategory::D5),
            "D6" => Ok(EmitterCategory::D6),
            "D7" => Ok(EmitterCategory::D7),
            _ => Err(format!("Invalid emitter category: {}", emitter_category)),
        }
    }
}

impl EmitterCategory {
    pub fn new(tc: TypeCoding, ca: u8) -> Result<Self, String> {
        match tc {
            TypeCoding::A => match ca {
                0 => Ok(EmitterCategory::A0),
                1 => Ok(EmitterCategory::A1),
                2 => Ok(EmitterCategory::A2),
                3 => Ok(EmitterCategory::A3),
                4 => Ok(EmitterCategory::A4),
                5 => Ok(EmitterCategory::A5),
                6 => Ok(EmitterCategory::A6),
                7 => Ok(EmitterCategory::A7),
                _ => Err(format!("Invalid emitter category: A{}", ca)),
            },
            TypeCoding::B => match ca {
                0 => Ok(EmitterCategory::B0),
                1 => Ok(EmitterCategory::B1),
                2 => Ok(EmitterCategory::B2),
                3 => Ok(EmitterCategory::B3),
                4 => Ok(EmitterCategory::B4),
                5 => Ok(EmitterCategory::B5),
                6 => Ok(EmitterCategory::B6),
                7 => Ok(EmitterCategory::B7),
                _ => Err(format!("Invalid emitter category: B{}", ca)),
            },
            TypeCoding::C => match ca {
                0 => Ok(EmitterCategory::C0),
                1 => Ok(EmitterCategory::C1),
                2 => Ok(EmitterCategory::C2),
                3 => Ok(EmitterCategory::C3),
                4 => Ok(EmitterCategory::C4),
                5 => Ok(EmitterCategory::C5),
                6 => Ok(EmitterCategory::C6),
                7 => Ok(EmitterCategory::C7),
                _ => Err(format!("Invalid emitter category: C{}", ca)),
            },
            TypeCoding::D => match ca {
                0 => Ok(EmitterCategory::D0),
                1 => Ok(EmitterCategory::D1),
                2 => Ok(EmitterCategory::D2),
                3 => Ok(EmitterCategory::D3),
                4 => Ok(EmitterCategory::D4),
                5 => Ok(EmitterCategory::D5),
                6 => Ok(EmitterCategory::D6),
                7 => Ok(EmitterCategory::D7),
                _ => Err(format!("Invalid emitter category: D{}", ca)),
            },
        }
    }
}

impl fmt::Display for EmitterCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmitterCategory::A0 => write!(f, "A0 : No ADS-B information"),
            EmitterCategory::A1 => write!(f, "A1 : Light (< 15500 lbs)"),
            EmitterCategory::A2 => write!(f, "A2 : Small (15500 to 75000 lbs)"),
            EmitterCategory::A3 => write!(f, "A3 : Large (75000 to 300000 lbs)"),
            EmitterCategory::A4 => write!(f, "A4 : High vortex large"),
            EmitterCategory::A5 => write!(f, "A5 : Heavy (> 300000 lbs) "),
            EmitterCategory::A6 => write!(f, "A6 : High performance"),
            EmitterCategory::A7 => write!(f, "A7 : Rotorcraft"),
            EmitterCategory::B0 => write!(f, "B0 : No ADS-B information"),
            EmitterCategory::B1 => write!(f, "B1 : Glider / sailplane"),
            EmitterCategory::B2 => write!(f, "B2 : Lighter-than-air"),
            EmitterCategory::B3 => write!(f, "B3 : Parachutist / skydiver"),
            EmitterCategory::B4 => write!(f, "B4 : Ultralight / hang-glider / paraglider"),
            EmitterCategory::B5 => write!(f, "B5 : Reserved"),
            EmitterCategory::B6 => write!(f, "B6 : Unmanned Aerial Vehicle"),
            EmitterCategory::B7 => {
                write!(f, "B7 : Space / trans-atmospheric vehicle")
            }
            EmitterCategory::C0 => write!(f, "C0 : No ADS-B information"),
            EmitterCategory::C1 => write!(f, "C1 : Surface vehicle - emergency vehicle"),
            EmitterCategory::C2 => {
                write!(f, "C2 : Surface vehicle - service vehicle")
            }
            EmitterCategory::C3 => write!(f, "C3 : Point obstacle (includes tethered balloons)"),
            EmitterCategory::C4 => write!(f, "C4 : Cluster obstacle"),
            EmitterCategory::C5 => write!(f, "C5 : Line obstacle"),
            EmitterCategory::C6 => write!(f, "C6: Reserved"),
            EmitterCategory::C7 => write!(f, "C7: Reserved"),
            EmitterCategory::D0 => write!(f, "D0 : Reserved/No ADS-B information"),
            EmitterCategory::D1 => write!(f, "D1 : Reserved"),
            EmitterCategory::D2 => write!(f, "D2 : Reserved"),
            EmitterCategory::D3 => write!(f, "D3 : Reserved"),
            EmitterCategory::D4 => write!(f, "D4 : Reserved"),
            EmitterCategory::D5 => write!(f, "D5 : Reserved"),
            EmitterCategory::D6 => write!(f, "D6 : Reserved"),
            EmitterCategory::D7 => write!(f, "D7 : Reserved"),
        }
    }
}
