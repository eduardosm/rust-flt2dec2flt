// Taken from rust 38114ff16e7856f98b2b4be7ab4cd29b38bed59a libcore::num.
// `algorithm::fpu_precision` is removed

//! Numeric traits and functions for the built-in numeric types.

// All these modules are technically private and only exposed for coretests:
pub mod bignum;
pub mod dec2flt;
pub mod diy_float;
pub mod flt2dec;

pub use crate::core_num::dec2flt::ParseFloatError;
