# Target, nBits and Difficulty

This project shows how to calculate those three values that are used in the Bitcoin protocol.

## nBits

This is the _compressed_ value that is used in the block header. It's a 4 byte (32 bits) value defined by:

- `exponent`: the first byte. It tells how _"far to the left"_ is the value;
- `coefficient`: the last 3 bytes. It is the value itself (with some precision, but not full).

Example:

![image](https://github.com/Guilospanck/btc-target-bits-difficulty/assets/22435398/20f75082-b8e4-4958-98f8-1a911f11e77b)
_From https://learnmeabitcoin.com/technical/block/bits/_


> Note that the nBits is a value that is not 100% accurate, but nevertheless it is used by the miners.

## Target

This is the _uncompressed_ value that miners use to validate the block found, i.e., if the block hash is below it.
It's a 32-byte (256 bits) value.

It is defined by:

$$target = coefficient * 2^{8(exponent-3)}$$


## Difficulty

This value actually does not exist in the protocol, it is more to provide a human-readable information to how difficult it is
to mine a block. It has the inverse meaning of the target, i.e., higher the target, lower the difficulty (easier it is to mine
a block); lower it is the target, higher the difficulty (harder to mine a block).

It is defined by:

$$Difficulty = \frac{max_{-}target}{current_{-}target}$$

where:

- `max_target`: hardcoded value defined in the protocol that has the value of `0x00000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffff` (but because of how the protocol is implemented, it is truncated to `0x00000000ffff0000000000000000000000000000000000000000000000000000`;
- `current_target`: the current (uncompressed) value.
