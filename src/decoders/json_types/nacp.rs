// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "u8")]
pub enum NavigationIntegrityCategory {
    Category11,
    Category10,
    Category9,
    Category8,
    Category7,
    Category6,
    Category5,
    Category4,
    Category3,
    Category2,
    Category1,
    #[default]
    Unknown,
}

impl TryFrom<u8> for NavigationIntegrityCategory {
    type Error = String;

    fn try_from(nic: u8) -> Result<Self, Self::Error> {
        match nic {
            11 => Ok(NavigationIntegrityCategory::Category11),
            10 => Ok(NavigationIntegrityCategory::Category10),
            9 => Ok(NavigationIntegrityCategory::Category9),
            8 => Ok(NavigationIntegrityCategory::Category8),
            7 => Ok(NavigationIntegrityCategory::Category7),
            6 => Ok(NavigationIntegrityCategory::Category6),
            5 => Ok(NavigationIntegrityCategory::Category5),
            4 => Ok(NavigationIntegrityCategory::Category4),
            3 => Ok(NavigationIntegrityCategory::Category3),
            2 => Ok(NavigationIntegrityCategory::Category2),
            1 => Ok(NavigationIntegrityCategory::Category1),
            0 => Ok(NavigationIntegrityCategory::Unknown),
            _ => Err(format!(
                "NIC should be a value between 0 and 11, inclusive. Found {}",
                nic
            )),
        }
    }
}

impl fmt::Display for NavigationIntegrityCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NavigationIntegrityCategory::Category11 => write!(f, "Category 11 (< 3m)"),
            NavigationIntegrityCategory::Category10 => write!(f, "Category 10 (< 10m)"),
            NavigationIntegrityCategory::Category9 => write!(f, "Category 9 (< 30m)"),
            NavigationIntegrityCategory::Category8 => write!(f, "Category 8 (< 0.05 NM (93 m)"),
            NavigationIntegrityCategory::Category7 => write!(f, "Category 7 (< 0.1 NM (185 m)"),
            NavigationIntegrityCategory::Category6 => write!(f, "Category 6 (< 0.3 NM (556 m)"),
            NavigationIntegrityCategory::Category5 => write!(f, "Category 5 (< 0.5 NM (926 m)"),
            NavigationIntegrityCategory::Category4 => write!(f, "Category 4 (< 1 NM (1852 m)"),
            NavigationIntegrityCategory::Category3 => write!(f, "Category 3 (< 2 NM (3704 m)"),
            NavigationIntegrityCategory::Category2 => write!(f, "Category 2 (< 4 NM (7408 m)"),
            NavigationIntegrityCategory::Category1 => write!(f, "Category 1 (<10 NM (18520 m)"),
            NavigationIntegrityCategory::Unknown => write!(f, ">10 NM or Unknown "),
        }
    }
}
