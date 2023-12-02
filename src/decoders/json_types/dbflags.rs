// Copyright 2023 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(from = "u8")]
pub enum DBFlags {
    Military,
    Interesting,
    PIA,
    LADD,
    #[default]
    None,
}

impl From<u8> for DBFlags {
    fn from(db_flags: u8) -> Self {
        // the u8 should be bitwise ANDed with the following values:
        // 1, 2, 4, 8
        // if the result is 0, then the flag is not set
        // if the result is not 0, then the flag is set

        // military = dbFlags & 1;
        // interesting = dbFlags & 2;
        // PIA = dbFlags & 4;
        // LADD = dbFlags & 8;

        if db_flags & 1 != 0 {
            Self::Military
        } else if db_flags & 2 != 0 {
            Self::Interesting
        } else if db_flags & 4 != 0 {
            Self::PIA
        } else if db_flags & 8 != 0 {
            Self::LADD
        } else {
            panic!(
                "DBFlags should be a value between 0 and 15, inclusive. Found: {}",
                db_flags
            ) // TODO: propagate this error
        }
    }
}

impl fmt::Display for DBFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DBFlags::Military => write!(f, "Military"),
            DBFlags::Interesting => write!(f, "Interesting"),
            DBFlags::PIA => write!(f, "PIA"),
            DBFlags::LADD => write!(f, "LADD"),
            DBFlags::None => write!(f, "None"),
        }
    }
}
