// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Default)]
#[serde(try_from = "u8")]
pub enum DBFlags {
    Military,
    Interesting,
    PIA,
    LADD,
    #[default]
    None,
}

impl Serialize for DBFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DBFlags::Military => serializer.serialize_u8(1),
            DBFlags::Interesting => serializer.serialize_u8(2),
            DBFlags::PIA => serializer.serialize_u8(4),
            DBFlags::LADD => serializer.serialize_u8(8),
            DBFlags::None => serializer.serialize_u8(0),
        }
    }
}

impl TryFrom<u8> for DBFlags {
    type Error = String;

    fn try_from(db_flags: u8) -> Result<Self, Self::Error> {
        // the u8 should be bitwise ANDed with the following values:
        // 1, 2, 4, 8
        // if the result is 0, then the flag is not set
        // if the result is not 0, then the flag is set

        // military = dbFlags & 1;
        // interesting = dbFlags & 2;
        // PIA = dbFlags & 4;
        // LADD = dbFlags & 8;

        if db_flags & 1 != 0 {
            Ok(Self::Military)
        } else if db_flags & 2 != 0 {
            Ok(Self::Interesting)
        } else if db_flags & 4 != 0 {
            Ok(Self::PIA)
        } else if db_flags & 8 != 0 {
            Ok(Self::LADD)
        } else {
            Err(format!(
                "DBFlags should be a value between 0 and 15, inclusive. Found: {}",
                db_flags
            ))
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
