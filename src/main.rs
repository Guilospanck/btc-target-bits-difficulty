mod constants;

use std::fmt::format;

use crate::constants::{
  MAXIMUM_FIRST_SIGNIFICANT_BYTE, MAXIMUM_TARGET_COEFFICIENT, MAXIMUM_TARGET_COMPRESSED,
  MAXIMUM_TARGET_EXPONENT, MAXIMUM_TARGET_UNCOMPRESSED, MINIMUM_DIFFICULTY,
  UNCOMPRESSED_LENGTH_IN_BYTES,
};

use hex;

#[derive(Debug, Clone)]
struct Bits {
  pub value: i32,
  pub exponent: u8,
  pub coefficient: Vec<u8>,
}

impl Bits {
  fn empty() -> Self {
    Bits {
      value: 0i32,
      exponent: 0u8,
      coefficient: vec![],
    }
  }

  fn max() -> Self {
    Bits {
      value: MAXIMUM_TARGET_COMPRESSED,
      exponent: MAXIMUM_TARGET_EXPONENT,
      coefficient: MAXIMUM_TARGET_COEFFICIENT.to_vec(),
    }
  }
}

struct Target {
  /// This is a 4-byte (32 bits) value
  /// It is known as BITS.
  /// It is used in the block header.
  compressed: Bits,
  /// This is a 32-byte (256 bits) value.
  /// It is used by miners to check validity of block.
  uncompressed: Vec<u8>,
  /// The difficulty is a human-readable number that helps understand
  /// how hard it is to mine a block.
  difficulty: u128,
}

impl Target {
  fn empty() -> Self {
    Target {
      compressed: Bits::empty(),
      uncompressed: vec![],
      difficulty: 0u128,
    }
  }

  fn max() -> Self {
    Target {
      compressed: Bits::max(),
      uncompressed: MAXIMUM_TARGET_UNCOMPRESSED.to_vec(),
      difficulty: MINIMUM_DIFFICULTY as u128,
    }
  }

  fn from_uncompressed(uncompressed: String) -> Self {
    let hex_bytes = hex::decode(uncompressed).unwrap();

    let mut target = Target::empty();
    target.uncompressed = hex_bytes;

    target.set_difficulty();
    target.set_compressed();

    target
  }

  fn from_compressed(compressed: i32) -> Self {
    let bytes = compressed.to_be_bytes();
    assert_eq!(bytes.len(), 4);

    let exponent = bytes[0];
    let coefficient = &bytes[1..].to_vec();

    let mut bits = Bits::empty();
    bits.exponent = exponent;
    bits.coefficient = coefficient.clone();
    bits.value = compressed;

    let mut target = Target::empty();
    target.compressed = bits;

    target.set_uncompressed();
    target.set_difficulty();

    target
  }

  /// Difficulty is defined as max_target / current_target
  ///
  /// NOTE:
  /// As target is represented as a 256-bit (32-byte) value,
  /// We're going to clamp it to a 128bit.
  fn set_difficulty(&mut self) {
    let first_16_bytes_of_max_target: [u8; 16] =
      MAXIMUM_TARGET_UNCOMPRESSED[..16].try_into().unwrap();
    let max_target_as_u128 = u128::from_be_bytes(first_16_bytes_of_max_target);

    let first_16_bytes_of_current_target: [u8; 16] = self.uncompressed[..16].try_into().unwrap();
    let current_target_as_u128 = u128::from_be_bytes(first_16_bytes_of_current_target);

    let difficulty = max_target_as_u128 / current_target_as_u128;

    self.difficulty = difficulty;
  }

  fn set_uncompressed(&mut self) {
    let exponent = self.compressed.exponent;
    let coefficient = self.compressed.coefficient.clone();

    let mut target: [u8; 32] = [0; 32];

    let mut count = 0u8;
    for value in coefficient {
      target[(UNCOMPRESSED_LENGTH_IN_BYTES as u8 - exponent + count) as usize] = value;
      count += 1;
    }

    let mining_target = target.to_vec();
    self.uncompressed = mining_target.clone();
  }

  fn set_compressed(&mut self) {
    // get first three signficant bytes and the index of the first signficant byte
    let (coefficient, byte_index) = self._get_coefficient_and_byte_index();
    let exponent = self._get_exponent(byte_index);

    let mut value_as_vec = vec![exponent];
    value_as_vec.append(&mut coefficient.clone());

    let as_array: [u8; 4] = value_as_vec[..]
      .try_into()
      .unwrap_or_else(|e| panic!("Could not transform vector into array of size 4: {}", e));

    let value = i32::from_be_bytes(as_array);

    let bits = Bits {
      coefficient,
      exponent,
      value,
    };

    self.compressed = bits;
  }

  fn _get_coefficient_and_byte_index(&self) -> (Vec<u8>, usize) {
    let mut coefficient_bytes: Vec<u8> = vec![];
    let mut byte_index: usize = 0;

    for (index, byte) in self.uncompressed.iter().enumerate() {
      if coefficient_bytes.len() == 3 {
        break;
      }

      if *byte == 0u8 {
        continue;
      }

      if *byte >= MAXIMUM_FIRST_SIGNIFICANT_BYTE && coefficient_bytes.len() == 0 {
        byte_index = index;
        coefficient_bytes.push(0);
      }

      if coefficient_bytes.len() == 0 {
        byte_index = index;
      }

      coefficient_bytes.push(*byte);
    }

    (coefficient_bytes, byte_index)
  }

  fn _get_exponent(&self, byte_index: usize) -> u8 {
    (UNCOMPRESSED_LENGTH_IN_BYTES - byte_index) as u8
  }

  fn _check_if_uncompressed_target_is_valid(self) -> bool {
    if self.uncompressed > MAXIMUM_TARGET_UNCOMPRESSED.to_vec() {
      return false;
    }

    true
  }
}

fn main() {
  {
    let max_target = Target::max();
    let encoded_bits_value = hex::encode(max_target.compressed.value.to_be_bytes());
    let uncompressed = hex::encode(max_target.uncompressed);
    println!("\nMax target: \n- Mining target (uncompressed) = {}\n- Block Header nBits (compressed) = {}\n- Difficulty: {}", uncompressed, encoded_bits_value, max_target.difficulty);
  }
  println!("=========================================================================================================");
  {
    let target = Target::from_uncompressed(
      "000000000000000000038c120000000000000000000000000000000000000000".to_owned(),
    );
    let encoded_bits_value = hex::encode(target.compressed.value.to_be_bytes());
    let uncompressed = hex::encode(target.uncompressed);
    println!("Target: \n- Mining target (uncompressed) = {}\n- Block Header nBits (compressed) = {}\n- Difficulty: {}", uncompressed, encoded_bits_value, target.difficulty);
    assert_eq!("17038c12", encoded_bits_value);
  }
  println!("=========================================================================================================");
  {
    let target = Target::from_compressed(0x17038c12);
    let encoded_bits_value = hex::encode(target.compressed.value.to_be_bytes());
    let uncompressed = hex::encode(target.uncompressed);
    println!("Target: \n- Mining target (uncompressed) = {}\n- Block Header nBits (compressed) = {}\n- Difficulty: {}", uncompressed, encoded_bits_value, target.difficulty);
    assert_eq!("000000000000000000038c120000000000000000000000000000000000000000", uncompressed);
  }
}
