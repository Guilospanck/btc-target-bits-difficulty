mod constants;

use std::fmt::format;

use crate::constants::{
  MAXIMUM_FIRST_SIGNIFICANT_BYTE, MAXIMUM_TARGET_COEFFICIENT, MAXIMUM_TARGET_COMPRESSED,
  MAXIMUM_TARGET_EXPONENT, MAXIMUM_TARGET_UNCOMPRESSED, MINIMUM_DIFFICULTY,
  UNCOMPRESSED_LENGTH_IN_BYTES,
};

use hex;

#[derive(Debug)]
struct Bits {
  pub value: i32,
  pub exponent: u8,
  pub coefficient: Vec<u8>,
}

impl Bits {
  fn new() -> Self {
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
  fn new() -> Self {
    Target {
      compressed: Bits::new(),
      uncompressed: MAXIMUM_TARGET_UNCOMPRESSED.to_vec(),
      difficulty: MINIMUM_DIFFICULTY as u128,
    }
  }

  /// Difficulty is defined as max_target / current_target
  ///
  /// NOTE:
  /// As target is represented as a 256-bit (32-byte) value,
  /// We're going to clamp it to a 128bit.
  fn to_difficulty(&mut self) -> u128 {
    // let maximum_target_uncompressed_as_binary_string: String = MAXIMUM_TARGET_UNCOMPRESSED.iter().map(|&b| format!("{:08b}", b)).collect();

    // println!("{}", maximum_target_uncompressed_as_binary_string);

    // let current_target_uncompressed_as_binary_string: String = self.uncompressed.iter().map(|&b| format!("{:08b}", b)).collect();
    // println!("{}", current_target_uncompressed_as_binary_string);

    // let difficulty = maximum_target_uncompressed_as_binary_string/current_target_uncompressed_as_binary_string;
    // println!("{}", difficulty);

    let first_16_bytes_of_max_target: [u8; 16] =
      MAXIMUM_TARGET_UNCOMPRESSED[..16].try_into().unwrap();
    let max_target_as_u128 = u128::from_be_bytes(first_16_bytes_of_max_target);

    let first_16_bytes_of_current_target: [u8; 16] = self.uncompressed[..16].try_into().unwrap();
    let current_target_as_u128 = u128::from_be_bytes(first_16_bytes_of_current_target);

    let difficulty = max_target_as_u128 / current_target_as_u128;

    self.difficulty = difficulty;

    difficulty
  }

  fn to_mining_target(&self) -> Vec<u8> {
    let exponent = self.compressed.exponent;
    let coefficient = self.compressed.coefficient.clone();

    let mut target: [u8; 32] = [0; 32];

    let mut count = 0u8;
    for value in coefficient {
      target[(UNCOMPRESSED_LENGTH_IN_BYTES as u8 - exponent + count) as usize] = value;
      count += 1;
    }

    target.to_vec()
  }

  fn to_bits(&self) -> Bits {
    // get first three signficant bytes and the index of the first signficant byte
    let (coefficient, byte_index) = self._get_coefficient_and_byte_index();
    let exponent = self._get_exponent(byte_index);

    let mut value_as_vec = vec![exponent];
    value_as_vec.append(&mut coefficient.clone());

    let as_array: [u8; 4] = value_as_vec[..]
      .try_into()
      .unwrap_or_else(|e| panic!("Could not transform vector into array of size 4: {}", e));

    let value = i32::from_be_bytes(as_array);

    Bits {
      coefficient,
      exponent,
      value,
    }
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
    // The index starts at 0, so we must add 1 to offset it
    (UNCOMPRESSED_LENGTH_IN_BYTES - byte_index + 1) as u8
  }

  fn _check_if_uncompressed_target_is_valid(self) -> bool {
    if self.uncompressed > MAXIMUM_TARGET_UNCOMPRESSED.to_vec() {
      return false;
    }

    true
  }
}

fn main() {
  let mut target = Target::new();

  println!("=============BITS====================");
  let bits = target.to_bits();

  let encoded_bits_value = hex::encode(bits.value.to_be_bytes());

  println!("{:?}", bits.coefficient);
  println!("{:?}", bits.exponent);
  println!("{:?}", encoded_bits_value);

  println!("=============MINNING TARGET====================");
  let mining_target = target.to_mining_target();
  println!("{:?}", hex::encode(mining_target));

  println!("=============DIFFICULTY====================");
  let difficulty= target.to_difficulty();
  println!("{:?}", difficulty);
}
