mod constants;

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
  compressed: Bits,
  /// This is a 32-byte (256 bits) value
  uncompressed: Vec<u8>,
  /// The difficulty is a human-readable number that helps understand
  /// how hard it is to mine a block.
  difficulty: i32,
}

impl Target {
  fn new() -> Self {
    Target {
      compressed: Bits::new(),
      uncompressed: MAXIMUM_TARGET_UNCOMPRESSED.to_vec(),
      difficulty: MINIMUM_DIFFICULTY,
    }
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
  let target = Target::new();

  let bits = target.to_bits();

  let encoded_bits_value = hex::encode(bits.value.to_be_bytes());


  println!("{:?}", bits.coefficient);
  println!("{:?}", bits.exponent);
  println!("{:?}", encoded_bits_value);
}
