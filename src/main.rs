mod constants;

use crate::constants::{MAXIMUM_TARGET_COMPRESSED, MAXIMUM_TARGET_UNCOMPRESSED, MINIMUM_DIFFICULTY};

struct Target {
  /// This is a 4-byte (32 bits) value
  /// It is known as BITS.
  compressed: i32,
  /// This is a 32-byte (256 bits) value
  uncompressed: Vec<u8>,
  /// The difficulty is a human-readable number that helps understand
  /// how hard it is to mine a block.
  difficulty: i32,
}

impl Target {
  fn new() -> Target {
    Target {
      compressed: MAXIMUM_TARGET_COMPRESSED,
      uncompressed: MAXIMUM_TARGET_UNCOMPRESSED.to_vec(),
      difficulty: MINIMUM_DIFFICULTY,
    }
  }

  fn to_bits(self) -> i32 {
    0
  }

  fn _check_if_uncompressed_target_is_valid(self) -> bool {
    if self.uncompressed > MAXIMUM_TARGET_UNCOMPRESSED.to_vec() {
      return false;
    }

    true
  }
}

fn main() {
  let target = Target::new();

  println!("{}", target._check_if_uncompressed_target_is_valid());
}
