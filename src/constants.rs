
pub const MAXIMUM_TARGET_EXPONENT: u8 = 0x1d;
pub const MAXIMUM_TARGET_COEFFICIENT: [u8; 3] =  [0x00, 0xff, 0xff];
pub const MAXIMUM_TARGET_COMPRESSED: i32 = 0x1d00ffff;

pub const UNCOMPRESSED_LENGTH_IN_BYTES: usize = 32;
/// 0x00000000ffff0000000000000000000000000000000000000000000000000000
pub const MAXIMUM_TARGET_UNCOMPRESSED: [u8; UNCOMPRESSED_LENGTH_IN_BYTES] = [0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub const MINIMUM_DIFFICULTY: i32 = 1;

/// To avoid setting a negative number. See https://learnmeabitcoin.com/technical/block/bits/
pub const MAXIMUM_FIRST_SIGNIFICANT_BYTE: u8 = 128; // 0x80