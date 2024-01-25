// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Default)]
pub enum GroundSpeed {
    #[default]
    None,
    Stopped,
    Speed(u8),
    ReseveredDeaccelerating,
    ReseveredAccelerating,
    ReseveredBackingUp,
}

impl From<u8> for GroundSpeed {
    fn from(speed: u8) -> Self {
        match speed {
            1 => GroundSpeed::Stopped,
            2..=123 => GroundSpeed::Speed(speed),
            124 => GroundSpeed::ReseveredAccelerating,
            125 => GroundSpeed::ReseveredBackingUp,
            126 => GroundSpeed::ReseveredDeaccelerating,
            _ => GroundSpeed::None,
        }
    }
}

impl GroundSpeed {
    #[must_use]
    pub fn calculate(&self) -> Option<f32> {
        //         0 	Speed not available
        // 1 	Stopped (v
        // 0.125 kt)
        // 2–8 	0.125
        // v
        // 1 kt 	0.125 kt steps
        // 9–12 	1 kt
        // v
        // 2 kt 	0.25 kt steps
        // 13–38 	2 kt
        // v
        // 15 kt 	0.5 kt steps
        // 39–93 	15 kt
        // v
        // 70 kt 	1 kt steps
        // 94–108 	70 kt
        // v
        // 100 kt 	2 kt steps
        // 109–123 	100 kt
        // v
        // 175 kt 	5 kt steps
        // 124 	v
        // 175 kt
        // 125–127 	Reserved

        match self {
            GroundSpeed::Stopped => Some(0.0),
            GroundSpeed::Speed(speed) => {
                let speed = f32::from(*speed);
                if speed <= 8.0 {
                    Some(0.125 + (speed - 2.0) * 0.125)
                } else if speed <= 12.0 {
                    Some(1.0 + (speed - 9.0) * 0.25)
                } else if speed <= 38.0 {
                    Some(2.0 + (speed - 13.0) * 0.5)
                } else if speed <= 93.0 {
                    Some(15.0 + (speed - 39.0) * 1.0)
                } else if speed <= 108.0 {
                    Some(70.0 + (speed - 94.0) * 2.0)
                } else if speed <= 123.0 {
                    Some(100.0 + (speed - 109.0) * 5.0)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl fmt::Display for GroundSpeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GroundSpeed::None => write!(f, "N/A"),
            GroundSpeed::Stopped => write!(f, "0 kt"),
            GroundSpeed::Speed(speed) => {
                let speed = f32::from(*speed);
                if speed <= 8.0 {
                    write!(f, "{} kt", 0.125 + (speed - 2.0) * 0.125)
                } else if speed <= 12.0 {
                    write!(f, "{} kt", 1.0 + (speed - 9.0) * 0.25)
                } else if speed <= 38.0 {
                    write!(f, "{} kt", 2.0 + (speed - 13.0) * 0.5)
                } else if speed <= 93.0 {
                    write!(f, "{} kt", 15.0 + (speed - 39.0) * 1.0)
                } else if speed <= 108.0 {
                    write!(f, "{} kt", 70.0 + (speed - 94.0) * 2.0)
                } else if speed <= 123.0 {
                    write!(f, "{} kt", 100.0 + (speed - 109.0) * 5.0)
                } else {
                    write!(f, "N/A")
                }
            }
            GroundSpeed::ReseveredAccelerating => write!(f, "Reserved (Accelerating)"),
            GroundSpeed::ReseveredBackingUp => write!(f, "Reserved (Backing Up)"),
            GroundSpeed::ReseveredDeaccelerating => write!(f, "Reserved (Deaccelerating)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_speed() {
        assert_eq!(GroundSpeed::None.calculate(), None);
        assert_eq!(GroundSpeed::Stopped.calculate(), Some(0.0));
        assert_eq!(GroundSpeed::Speed(2).calculate(), Some(0.125));
        assert_eq!(GroundSpeed::Speed(3).calculate(), Some(0.25));
        assert_eq!(GroundSpeed::Speed(4).calculate(), Some(0.375));
        assert_eq!(GroundSpeed::Speed(5).calculate(), Some(0.5));
        assert_eq!(GroundSpeed::Speed(6).calculate(), Some(0.625));
        assert_eq!(GroundSpeed::Speed(7).calculate(), Some(0.75));
        assert_eq!(GroundSpeed::Speed(8).calculate(), Some(0.875));
        assert_eq!(GroundSpeed::Speed(9).calculate(), Some(1.0));
        assert_eq!(GroundSpeed::Speed(10).calculate(), Some(1.25));
        assert_eq!(GroundSpeed::Speed(11).calculate(), Some(1.5));
        assert_eq!(GroundSpeed::Speed(12).calculate(), Some(1.75));
        assert_eq!(GroundSpeed::Speed(13).calculate(), Some(2.0));
        assert_eq!(GroundSpeed::Speed(14).calculate(), Some(2.5));
        assert_eq!(GroundSpeed::Speed(15).calculate(), Some(3.0));
        assert_eq!(GroundSpeed::Speed(16).calculate(), Some(3.5));
        assert_eq!(GroundSpeed::Speed(17).calculate(), Some(4.0));
        assert_eq!(GroundSpeed::Speed(18).calculate(), Some(4.5));
        assert_eq!(GroundSpeed::Speed(19).calculate(), Some(5.0));
        assert_eq!(GroundSpeed::Speed(20).calculate(), Some(5.5));
        assert_eq!(GroundSpeed::Speed(21).calculate(), Some(6.0));
        assert_eq!(GroundSpeed::Speed(22).calculate(), Some(6.5));
        assert_eq!(GroundSpeed::Speed(23).calculate(), Some(7.0));
        assert_eq!(GroundSpeed::Speed(24).calculate(), Some(7.5));
        assert_eq!(GroundSpeed::Speed(25).calculate(), Some(8.0));
        assert_eq!(GroundSpeed::Speed(26).calculate(), Some(8.5));
        assert_eq!(GroundSpeed::Speed(27).calculate(), Some(9.0));
        assert_eq!(GroundSpeed::Speed(28).calculate(), Some(9.5));
        assert_eq!(GroundSpeed::Speed(29).calculate(), Some(10.0));
        assert_eq!(GroundSpeed::Speed(30).calculate(), Some(10.5));
        assert_eq!(GroundSpeed::Speed(31).calculate(), Some(11.0));
        assert_eq!(GroundSpeed::Speed(32).calculate(), Some(11.5));
        assert_eq!(GroundSpeed::Speed(33).calculate(), Some(12.0));
        assert_eq!(GroundSpeed::Speed(34).calculate(), Some(12.5));
        assert_eq!(GroundSpeed::Speed(35).calculate(), Some(13.0));
        assert_eq!(GroundSpeed::Speed(36).calculate(), Some(13.5));
        assert_eq!(GroundSpeed::Speed(37).calculate(), Some(14.0));
        assert_eq!(GroundSpeed::Speed(38).calculate(), Some(14.5));
        assert_eq!(GroundSpeed::Speed(39).calculate(), Some(15.0));
        assert_eq!(GroundSpeed::Speed(40).calculate(), Some(16.0));
        assert_eq!(GroundSpeed::Speed(41).calculate(), Some(17.0));
        assert_eq!(GroundSpeed::Speed(42).calculate(), Some(18.0));
        assert_eq!(GroundSpeed::Speed(43).calculate(), Some(19.0));
        assert_eq!(GroundSpeed::Speed(44).calculate(), Some(20.0));
        assert_eq!(GroundSpeed::Speed(45).calculate(), Some(21.0));
        assert_eq!(GroundSpeed::Speed(46).calculate(), Some(22.0));
        assert_eq!(GroundSpeed::Speed(47).calculate(), Some(23.0));
        assert_eq!(GroundSpeed::Speed(48).calculate(), Some(24.0));
        assert_eq!(GroundSpeed::Speed(49).calculate(), Some(25.0));
        assert_eq!(GroundSpeed::Speed(50).calculate(), Some(26.0));
        assert_eq!(GroundSpeed::Speed(51).calculate(), Some(27.0));
        assert_eq!(GroundSpeed::Speed(52).calculate(), Some(28.0));
        assert_eq!(GroundSpeed::Speed(53).calculate(), Some(29.0));
        assert_eq!(GroundSpeed::Speed(54).calculate(), Some(30.0));
        assert_eq!(GroundSpeed::Speed(55).calculate(), Some(31.0));
        assert_eq!(GroundSpeed::Speed(56).calculate(), Some(32.0));
        assert_eq!(GroundSpeed::Speed(57).calculate(), Some(33.0));
        assert_eq!(GroundSpeed::Speed(58).calculate(), Some(34.0));
        assert_eq!(GroundSpeed::Speed(59).calculate(), Some(35.0));
        assert_eq!(GroundSpeed::Speed(60).calculate(), Some(36.0));
        assert_eq!(GroundSpeed::Speed(61).calculate(), Some(37.0));
        assert_eq!(GroundSpeed::Speed(62).calculate(), Some(38.0));
        assert_eq!(GroundSpeed::Speed(63).calculate(), Some(39.0));
        assert_eq!(GroundSpeed::Speed(64).calculate(), Some(40.0));
        assert_eq!(GroundSpeed::Speed(65).calculate(), Some(41.0));
        assert_eq!(GroundSpeed::Speed(66).calculate(), Some(42.0));
        assert_eq!(GroundSpeed::Speed(67).calculate(), Some(43.0));
        assert_eq!(GroundSpeed::Speed(68).calculate(), Some(44.0));
        assert_eq!(GroundSpeed::Speed(69).calculate(), Some(45.0));
        assert_eq!(GroundSpeed::Speed(70).calculate(), Some(46.0));
        assert_eq!(GroundSpeed::Speed(71).calculate(), Some(47.0));
        assert_eq!(GroundSpeed::Speed(72).calculate(), Some(48.0));
        assert_eq!(GroundSpeed::Speed(73).calculate(), Some(49.0));
        assert_eq!(GroundSpeed::Speed(74).calculate(), Some(50.0));
        assert_eq!(GroundSpeed::Speed(75).calculate(), Some(51.0));
        assert_eq!(GroundSpeed::Speed(76).calculate(), Some(52.0));
        assert_eq!(GroundSpeed::Speed(77).calculate(), Some(53.0));
        assert_eq!(GroundSpeed::Speed(78).calculate(), Some(54.0));
        assert_eq!(GroundSpeed::Speed(79).calculate(), Some(55.0));
        assert_eq!(GroundSpeed::Speed(80).calculate(), Some(56.0));
        assert_eq!(GroundSpeed::Speed(81).calculate(), Some(57.0));
        assert_eq!(GroundSpeed::Speed(82).calculate(), Some(58.0));
        assert_eq!(GroundSpeed::Speed(83).calculate(), Some(59.0));
        assert_eq!(GroundSpeed::Speed(84).calculate(), Some(60.0));
        assert_eq!(GroundSpeed::Speed(85).calculate(), Some(61.0));
        assert_eq!(GroundSpeed::Speed(86).calculate(), Some(62.0));
        assert_eq!(GroundSpeed::Speed(87).calculate(), Some(63.0));
        assert_eq!(GroundSpeed::Speed(88).calculate(), Some(64.0));
        assert_eq!(GroundSpeed::Speed(89).calculate(), Some(65.0));
        assert_eq!(GroundSpeed::Speed(90).calculate(), Some(66.0));
        assert_eq!(GroundSpeed::Speed(91).calculate(), Some(67.0));
        assert_eq!(GroundSpeed::Speed(92).calculate(), Some(68.0));
        assert_eq!(GroundSpeed::Speed(93).calculate(), Some(69.0));
        assert_eq!(GroundSpeed::Speed(94).calculate(), Some(70.0));
        assert_eq!(GroundSpeed::Speed(95).calculate(), Some(72.0));
        assert_eq!(GroundSpeed::Speed(96).calculate(), Some(74.0));
        assert_eq!(GroundSpeed::Speed(97).calculate(), Some(76.0));
        assert_eq!(GroundSpeed::Speed(98).calculate(), Some(78.0));
        assert_eq!(GroundSpeed::Speed(99).calculate(), Some(80.0));
        assert_eq!(GroundSpeed::Speed(100).calculate(), Some(82.0));
        assert_eq!(GroundSpeed::Speed(101).calculate(), Some(84.0));
        assert_eq!(GroundSpeed::Speed(102).calculate(), Some(86.0));
        assert_eq!(GroundSpeed::Speed(103).calculate(), Some(88.0));
        assert_eq!(GroundSpeed::Speed(104).calculate(), Some(90.0));
        assert_eq!(GroundSpeed::Speed(105).calculate(), Some(92.0));
        assert_eq!(GroundSpeed::Speed(106).calculate(), Some(94.0));
        assert_eq!(GroundSpeed::Speed(107).calculate(), Some(96.0));
        assert_eq!(GroundSpeed::Speed(108).calculate(), Some(98.0));
        assert_eq!(GroundSpeed::Speed(109).calculate(), Some(100.0));
        assert_eq!(GroundSpeed::Speed(110).calculate(), Some(105.0));
        assert_eq!(GroundSpeed::Speed(111).calculate(), Some(110.0));
        assert_eq!(GroundSpeed::Speed(112).calculate(), Some(115.0));
        assert_eq!(GroundSpeed::Speed(113).calculate(), Some(120.0));
        assert_eq!(GroundSpeed::Speed(114).calculate(), Some(125.0));
        assert_eq!(GroundSpeed::Speed(115).calculate(), Some(130.0));
        assert_eq!(GroundSpeed::Speed(116).calculate(), Some(135.0));
        assert_eq!(GroundSpeed::Speed(117).calculate(), Some(140.0));
        assert_eq!(GroundSpeed::Speed(118).calculate(), Some(145.0));
        assert_eq!(GroundSpeed::Speed(119).calculate(), Some(150.0));
        assert_eq!(GroundSpeed::Speed(120).calculate(), Some(155.0));
        assert_eq!(GroundSpeed::Speed(121).calculate(), Some(160.0));
        assert_eq!(GroundSpeed::Speed(122).calculate(), Some(165.0));
        assert_eq!(GroundSpeed::Speed(123).calculate(), Some(170.0));
        assert_eq!(GroundSpeed::Speed(124).calculate(), None);
    }
}
