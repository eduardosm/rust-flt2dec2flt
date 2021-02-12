// Taken from libcore::num from Rust commit d416093209d0dd77a4cdeb5a2f1b5de1316787ec.
// `algorithm::fpu_precision` has been removed

//! Numeric traits and functions for the built-in numeric types.

// All these modules are technically private and only exposed for coretests:
pub mod bignum;
pub mod dec2flt;
pub mod diy_float;
pub mod flt2dec;

pub use dec2flt::ParseFloatError;
