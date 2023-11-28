// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// emitter category https://www.adsbexchange.com/emitter-category-ads-b-do-260b-2-2-3-2-5-2/

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "String")]
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
}

impl From<String> for EmitterCategory {
    fn from(emitter_category: String) -> Self {
        match emitter_category.as_str() {
            "A0" => EmitterCategory::A0,
            "A1" => EmitterCategory::A1,
            "A2" => EmitterCategory::A2,
            "A3" => EmitterCategory::A3,
            "A4" => EmitterCategory::A4,
            "A5" => EmitterCategory::A5,
            "A6" => EmitterCategory::A6,
            "A7" => EmitterCategory::A7,
            "B0" => EmitterCategory::B0,
            "B1" => EmitterCategory::B1,
            "B2" => EmitterCategory::B2,
            "B3" => EmitterCategory::B3,
            "B4" => EmitterCategory::B4,
            "B5" => EmitterCategory::B5,
            "B6" => EmitterCategory::B6,
            "B7" => EmitterCategory::B7,
            "C0" => EmitterCategory::C0,
            "C1" => EmitterCategory::C1,
            "C2" => EmitterCategory::C2,
            "C3" => EmitterCategory::C3,
            "C4" => EmitterCategory::C4,
            "C5" => EmitterCategory::C5,
            "C6" => EmitterCategory::C6,
            "C7" => EmitterCategory::C7,
            _ => EmitterCategory::A0,
        }
    }
}

impl fmt::Display for EmitterCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmitterCategory::A0 => write!(f, "A0 : No ADS-B emitter category information"),
            EmitterCategory::A1 => write!(f, "A1 : Light (< 15500 lbs)"),
            EmitterCategory::A2 => write!(f, "A2 : Small (15500 to 75000 lbs)"),
            EmitterCategory::A3 => write!(f, "A3 : Large (75000 to 300000 lbs)"),
            EmitterCategory::A4 => write!(f, "A4 :  High vortex large"),
            EmitterCategory::A5 => write!(f, "A5 : Heavy (> 300000 lbs) "),
            EmitterCategory::A6 => write!(f, "A6 : High performance"),
            EmitterCategory::A7 => write!(f, "A7 : Rotorcraft"),
            EmitterCategory::B0 => write!(f, "B0 : No ADS-B emitter category information"),
            EmitterCategory::B1 => write!(f, "B1 : Glider / sailplane"),
            EmitterCategory::B2 => write!(f, "B2 : Lighter-than-air"),
            EmitterCategory::B3 => write!(f, "B3 : Parachutist / skydiver"),
            EmitterCategory::B4 => write!(f, "B4 : Ultralight / hang-glider / paraglider"),
            EmitterCategory::B5 => write!(f, "B5 : Reserved"),
            EmitterCategory::B6 => write!(f, "B6 : Unmanned Aerial Vehicle"),
            EmitterCategory::B7 => write!(f, "B7 : Space / trans-atmospheric vehicle"),
            EmitterCategory::C0 => write!(f, "C0 : No ADS-B emitter category information"),
            EmitterCategory::C1 => write!(f, "C1 : Surface vehicle - emergency vehicle"),
            EmitterCategory::C2 => write!(f, "C2 : Surface vehicle - service vehicle"),
            EmitterCategory::C3 => write!(f, "C3 : Point obstacle (includes tethered balloons)"),
            EmitterCategory::C4 => write!(f, "C4 : Cluster obstacle"),
            EmitterCategory::C5 => write!(f, "C5 : Line obstacle"),
            EmitterCategory::C6 => write!(f, "C6: Reserved"),
            EmitterCategory::C7 => write!(f, "C7: Reserved"),
        }
    }
}
