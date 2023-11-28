// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "u8")]
pub enum NavigationIntegrityCategory {
    Category11 = 11,
    Category10 = 10,
    Category9 = 9,
    Category8 = 8,
    Category7 = 7,
    Category6 = 6,
    Category5 = 5,
    Category4 = 4,
    Category3 = 3,
    Category2 = 2,
    Category1 = 1,
    #[default]
    Unknown = 0,
}

impl From<u8> for NavigationIntegrityCategory {
    fn from(nic: u8) -> Self {
        match nic {
            11 => NavigationIntegrityCategory::Category11,
            10 => NavigationIntegrityCategory::Category10,
            9 => NavigationIntegrityCategory::Category9,
            8 => NavigationIntegrityCategory::Category8,
            7 => NavigationIntegrityCategory::Category7,
            6 => NavigationIntegrityCategory::Category6,
            5 => NavigationIntegrityCategory::Category5,
            4 => NavigationIntegrityCategory::Category4,
            3 => NavigationIntegrityCategory::Category3,
            2 => NavigationIntegrityCategory::Category2,
            1 => NavigationIntegrityCategory::Category1,
            _ => NavigationIntegrityCategory::Unknown,
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
