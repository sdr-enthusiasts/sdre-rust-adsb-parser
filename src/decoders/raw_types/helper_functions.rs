// Copyright (c) 2023-2024 Frederick Clausen II

// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use deku::ctx::BitSize;
use deku::no_std_io::{Read, Seek};
use deku::prelude::*;
use deku::{DekuError, error::NeedSize};

pub(crate) fn decode_id13_field(id13_field: u32) -> u32 {
    let mut hex_gillham: u32 = 0;

    if id13_field & 0x1000 != 0 {
        hex_gillham |= 0x0010;
    } // Bit 12 = C1
    if id13_field & 0x0800 != 0 {
        hex_gillham |= 0x1000;
    } // Bit 11 = A1
    if id13_field & 0x0400 != 0 {
        hex_gillham |= 0x0020;
    } // Bit 10 = C2
    if id13_field & 0x0200 != 0 {
        hex_gillham |= 0x2000;
    } // Bit  9 = A2
    if id13_field & 0x0100 != 0 {
        hex_gillham |= 0x0040;
    } // Bit  8 = C4
    if id13_field & 0x0080 != 0 {
        hex_gillham |= 0x4000;
    } // Bit  7 = A4
    //if id13_field & 0x0040 != 0 {hex_gillham |= 0x0800;} // Bit  6 = X  or M
    if id13_field & 0x0020 != 0 {
        hex_gillham |= 0x0100;
    } // Bit  5 = B1
    if id13_field & 0x0010 != 0 {
        hex_gillham |= 0x0001;
    } // Bit  4 = D1 or Q
    if id13_field & 0x0008 != 0 {
        hex_gillham |= 0x0200;
    } // Bit  3 = B2
    if id13_field & 0x0004 != 0 {
        hex_gillham |= 0x0002;
    } // Bit  2 = D2
    if id13_field & 0x0002 != 0 {
        hex_gillham |= 0x0400;
    } // Bit  1 = B4
    if id13_field & 0x0001 != 0 {
        hex_gillham |= 0x0004;
    } // Bit  0 = D4

    hex_gillham
}

pub(crate) fn mode_a_to_mode_c(mode_a: u32) -> Result<u32, String> {
    let mut five_hundreds: u32 = 0;
    let mut one_hundreds: u32 = 0;

    // check zero bits are zero, D1 set is illegal; C1,,C4 cannot be Zero
    if (mode_a & 0xffff_8889) != 0 || (mode_a & 0x0000_00f0) == 0 {
        return Err("Invalid altitude".to_string());
    }

    if mode_a & 0x0010 != 0 {
        one_hundreds ^= 0x007;
    } // C1
    if mode_a & 0x0020 != 0 {
        one_hundreds ^= 0x003;
    } // C2
    if mode_a & 0x0040 != 0 {
        one_hundreds ^= 0x001;
    } // C4

    // Remove 7s from OneHundreds (Make 7->5, snd 5->7).
    if (one_hundreds & 5) == 5 {
        one_hundreds ^= 2;
    }

    // Check for invalid codes, only 1 to 5 are valid
    if one_hundreds > 5 {
        return Err("Invalid altitude".to_string());
    }

    // if mode_a & 0x0001 {five_hundreds ^= 0x1FF;} // D1 never used for altitude
    if mode_a & 0x0002 != 0 {
        five_hundreds ^= 0x0ff;
    } // D2
    if mode_a & 0x0004 != 0 {
        five_hundreds ^= 0x07f;
    } // D4

    if mode_a & 0x1000 != 0 {
        five_hundreds ^= 0x03f;
    } // A1
    if mode_a & 0x2000 != 0 {
        five_hundreds ^= 0x01f;
    } // A2
    if mode_a & 0x4000 != 0 {
        five_hundreds ^= 0x00f;
    } // A4

    if mode_a & 0x0100 != 0 {
        five_hundreds ^= 0x007;
    } // B1
    if mode_a & 0x0200 != 0 {
        five_hundreds ^= 0x003;
    } // B2
    if mode_a & 0x0400 != 0 {
        five_hundreds ^= 0x001;
    } // B4

    // Correct order of one_hundreds.
    if five_hundreds & 1 != 0 && one_hundreds <= 6 {
        one_hundreds = 6 - one_hundreds;
    }

    let n: u32 = (five_hundreds * 5) + one_hundreds;
    if n >= 13 {
        Ok(n - 13)
    } else {
        Err("Invalid altitude".to_string())
    }
}

const CHAR_LOOKUP: &[u8; 64] = b"#ABCDEFGHIJKLMNOPQRSTUVWXYZ##### ###############0123456789######";

pub const CRC_TABLE: [u32; 256] = [
    0x0000_0000,
    0x00ff_f409,
    0x0000_1c1b,
    0x00ff_e812,
    0x0000_3836,
    0x00ff_cc3f,
    0x0000_242d,
    0x00ff_d024,
    0x0000_706c,
    0x00ff_8465,
    0x0000_6c77,
    0x00ff_987e,
    0x0000_485a,
    0x00ff_bc53,
    0x0000_5441,
    0x00ff_a048,
    0x0000_e0d8,
    0x00ff_14d1,
    0x0000_fcc3,
    0x00ff_08ca,
    0x0000_d8ee,
    0x00ff_2ce7,
    0x0000_c4f5,
    0x00ff_30fc,
    0x0000_90b4,
    0x00ff_64bd,
    0x0000_8caf,
    0x00ff_78a6,
    0x0000_a882,
    0x00ff_5c8b,
    0x0000_b499,
    0x00ff_4090,
    0x0001_c1b0,
    0x00fe_35b9,
    0x0001_ddab,
    0x00fe_29a2,
    0x0001_f986,
    0x00fe_0d8f,
    0x0001_e59d,
    0x00fe_1194,
    0x0001_b1dc,
    0x00fe_45d5,
    0x0001_adc7,
    0x00fe_59ce,
    0x0001_89ea,
    0x00fe_7de3,
    0x0001_95f1,
    0x00fe_61f8,
    0x0001_2168,
    0x00fe_d561,
    0x0001_3d73,
    0x00fe_c97a,
    0x0001_195e,
    0x00fe_ed57,
    0x0001_0545,
    0x00fe_f14c,
    0x0001_5104,
    0x00fe_a50d,
    0x0001_4d1f,
    0x00fe_b916,
    0x0001_6932,
    0x00fe_9d3b,
    0x0001_7529,
    0x00fe_8120,
    0x0003_8360,
    0x00fc_7769,
    0x0003_9f7b,
    0x00fc_6b72,
    0x0003_bb56,
    0x00fc_4f5f,
    0x0003_a74d,
    0x00fc_5344,
    0x0003_f30c,
    0x00fc_0705,
    0x0003_ef17,
    0x00fc_1b1e,
    0x0003_cb3a,
    0x00fc_3f33,
    0x0003_d721,
    0x00fc_2328,
    0x0003_63b8,
    0x00fc_97b1,
    0x0003_7fa3,
    0x00fc_8baa,
    0x0003_5b8e,
    0x00fc_af87,
    0x0003_4795,
    0x00fc_b39c,
    0x0003_13d4,
    0x00fc_e7dd,
    0x0003_0fcf,
    0x00fc_fbc6,
    0x0003_2be2,
    0x00fc_dfeb,
    0x0003_37f9,
    0x00fc_c3f0,
    0x0002_42d0,
    0x00fd_b6d9,
    0x0002_5ecb,
    0x00fd_aac2,
    0x0002_7ae6,
    0x00fd_8eef,
    0x0002_66fd,
    0x00fd_92f4,
    0x0002_32bc,
    0x00fd_c6b5,
    0x0002_2ea7,
    0x00fd_daae,
    0x0002_0a8a,
    0x00fd_fe83,
    0x0002_1691,
    0x00fd_e298,
    0x0002_a208,
    0x00fd_5601,
    0x0002_be13,
    0x00fd_4a1a,
    0x0002_9a3e,
    0x00fd_6e37,
    0x0002_8625,
    0x00fd_722c,
    0x0002_d264,
    0x00fd_266d,
    0x0002_ce7f,
    0x00fd_3a76,
    0x0002_ea52,
    0x00fd_1e5b,
    0x0002_f649,
    0x00fd_0240,
    0x0007_06c0,
    0x00f8_f2c9,
    0x0007_1adb,
    0x00f8_eed2,
    0x0007_3ef6,
    0x00f8_caff,
    0x0007_22ed,
    0x00f8_d6e4,
    0x0007_76ac,
    0x00f8_82a5,
    0x0007_6ab7,
    0x00f8_9ebe,
    0x0007_4e9a,
    0x00f8_ba93,
    0x0007_5281,
    0x00f8_a688,
    0x0007_e618,
    0x00f8_1211,
    0x0007_fa03,
    0x00f8_0e0a,
    0x0007_de2e,
    0x00f8_2a27,
    0x0007_c235,
    0x00f8_363c,
    0x0007_9674,
    0x00f8_627d,
    0x0007_8a6f,
    0x00f8_7e66,
    0x0007_ae42,
    0x00f8_5a4b,
    0x0007_b259,
    0x00f8_4650,
    0x0006_c770,
    0x00f9_3379,
    0x0006_db6b,
    0x00f9_2f62,
    0x0006_ff46,
    0x00f9_0b4f,
    0x0006_e35d,
    0x00f9_1754,
    0x0006_b71c,
    0x00f9_4315,
    0x0006_ab07,
    0x00f9_5f0e,
    0x0006_8f2a,
    0x00f9_7b23,
    0x0006_9331,
    0x00f9_6738,
    0x0006_27a8,
    0x00f9_d3a1,
    0x0006_3bb3,
    0x00f9_cfba,
    0x0006_1f9e,
    0x00f9_eb97,
    0x0006_0385,
    0x00f9_f78c,
    0x0006_57c4,
    0x00f9_a3cd,
    0x0006_4bdf,
    0x00f9_bfd6,
    0x0006_6ff2,
    0x00f9_9bfb,
    0x0006_73e9,
    0x00f9_87e0,
    0x0004_85a0,
    0x00fb_71a9,
    0x0004_99bb,
    0x00fb_6db2,
    0x0004_bd96,
    0x00fb_499f,
    0x0004_a18d,
    0x00fb_5584,
    0x0004_f5cc,
    0x00fb_01c5,
    0x0004_e9d7,
    0x00fb_1dde,
    0x0004_cdfa,
    0x00fb_39f3,
    0x0004_d1e1,
    0x00fb_25e8,
    0x0004_6578,
    0x00fb_9171,
    0x0004_7963,
    0x00fb_8d6a,
    0x0004_5d4e,
    0x00fb_a947,
    0x0004_4155,
    0x00fb_b55c,
    0x0004_1514,
    0x00fb_e11d,
    0x0004_090f,
    0x00fb_fd06,
    0x0004_2d22,
    0x00fb_d92b,
    0x0004_3139,
    0x00fb_c530,
    0x0005_4410,
    0x00fa_b019,
    0x0005_580b,
    0x00fa_ac02,
    0x0005_7c26,
    0x00fa_882f,
    0x0005_603d,
    0x00fa_9434,
    0x0005_347c,
    0x00fa_c075,
    0x0005_2867,
    0x00fa_dc6e,
    0x0005_0c4a,
    0x00fa_f843,
    0x0005_1051,
    0x00fa_e458,
    0x0005_a4c8,
    0x00fa_50c1,
    0x0005_b8d3,
    0x00fa_4cda,
    0x0005_9cfe,
    0x00fa_68f7,
    0x0005_80e5,
    0x00fa_74ec,
    0x0005_d4a4,
    0x00fa_20ad,
    0x0005_c8bf,
    0x00fa_3cb6,
    0x0005_ec92,
    0x00fa_189b,
    0x0005_f089,
    0x00fa_0480,
];

pub(crate) fn modes_checksum(message: &[u8], bits: usize) -> Result<u32, DekuError> {
    let mut rem: u32 = 0;
    let n: usize = bits / 8;

    if (n < 3) || (message.len() < n) {
        return Err(DekuError::Incomplete(NeedSize::new(4)));
    }

    for i in 0..(n - 3) {
        rem =
            (rem << 8) ^ CRC_TABLE[(u32::from(message[i]) ^ ((rem & 0x00ff_0000) >> 16)) as usize];
        rem &= 0x00ff_ffff;
    }

    let msg_1: u32 = u32::from(message[n - 3]) << 16;
    let msg_2: u32 = u32::from(message[n - 2]) << 8;
    let msg_3: u32 = u32::from(message[n - 1]);
    let xor_term: u32 = msg_1 ^ msg_2 ^ msg_3;

    rem ^= xor_term;

    Ok(rem)
}

pub(crate) fn aircraft_identification_read<R: Read + Seek>(
    reader: &mut Reader<R>,
) -> Result<String, DekuError> {
    let mut chars = vec![];
    for _ in 0..=6 {
        let c = <u8>::from_reader_with_ctx(reader, BitSize(6))?;
        if c != 32 {
            chars.push(c);
        }
    }
    let encoded = chars
        .into_iter()
        .map(|b| CHAR_LOOKUP[b as usize] as char)
        .collect::<String>();

    Ok(encoded)
}
