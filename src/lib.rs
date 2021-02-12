//! This crate provides low-level functions to convert floating point
//! numbers (`f32` and `f64`) to decimal strings and vice versa.
//!
//! Implementing accurate float-string conversion is non-trivial (see
//! for example "Printing floating-point numbers: An always correct
//! method" by Marc Andrysco, Ranjit Jhala and Sorin Lerner). The Rust
//! Standard Library internally uses complex algorithms (found in
//! `libcore/num`) to fulfill this purpose, but they are not exposed
//! and can only be used through `FromStr::from_str`, `ToString::to_string`
//! or `Display::fmt`.
//!
//! These functions impose a format on the string representation
//! of floating point numbers, which may not always be suitable for you.
//! For example, you may want to write `1.2 * 10^4` instead of `1.2e4`.
//!
//! This crate exposes the core algorithms, allowing you to implement
//! your custom float-string conversions without worrying about complex
//! mathematical part.
//!
//! The functionality of this crate is provided through the `FloatExt`
//! trait, which is implemented for `f32` and `f64`.
//!
//! # Minimum Rust version
//!
//! The minimum Rust version required by this crate is 1.43.
//!
//! # Example (float to string)
//!
//! ```
//! // Let's say we want to convert this value to a string
//! // in exponential form.
//! let value = 1.25e20;
//!
//! // Using the standard library, you can use `format!("{:e}")`:
//! assert_eq!(format!("{:e}", value), "1.25e20");
//! // It also allows tu use a capital 'E':
//! assert_eq!(format!("{:E}", value), "1.25E20");
//! // or to include an explicit '+':
//! assert_eq!(format!("{:+e}", value), "+1.25e20");
//! // or to used a fixed number of digits:
//! assert_eq!(format!("{:.04e}", value), "1.2500e20");
//!
//! // However, besides those options, `format!` imposes the
//! // format of the string representation of the floating point
//! // number (using `.` as decimal separator and `e` or `E` as
//! // exponential separator).
//!
//! // This crate provides low-level functions to conver floating point
//! // numbers to strings without an imposed format.
//! use flt2dec2flt::FloatExt as _;
//!
//! // The `FloatExt::preformat_*` functions pre-converts the floating
//! // point numbers into string, providing a decomposed intermediate
//! // result.
//!
//! let mut buf = [0; flt2dec2flt::PREFORMAT_SHORTEST_BUF_LEN];
//! // You could also use `f32::preformat_shortest(value, &mut buf)`
//! let preformatted = value.preformat_shortest(&mut buf);
//! // `false` means the the number is positive, `b"125"` are the
//! // significant digits, `0` is the number of extra zeros at the
//! // right and `21` is the exponent (such as `1.25e20 == 0.125e21`)
//! assert_eq!(
//!     preformatted,
//!     flt2dec2flt::PreFormatted::Finite(false, b"125", 0, 21),
//! );
//!
//! // From this decomposed form, you can now build your custom string
//! // representation of the floating point number.
//! ```
//!
//! # Example (string to float)
//!
//! ```
//! use std::str::FromStr as _;
//! // Let's say you want to convert a string to a floating
//! // point number.
//!
//! // Using the standard library, you can use `FromStr::from_str`:
//! assert_eq!(f32::from_str("1.25e20").unwrap(), 1.25e20);
//!
//! // However, this function imposes the format of the input string.
//!
//! // This crate provides functions to convert a pre-parsed string to
//! // a floating.
//! use flt2dec2flt::FloatExt as _;
//!
//! // You have to implement your pre-parsing based on your string format.
//! // So, `1.25e20` (or `1.25*10^20`) would be pre-parsed as:
//! let preparsed = flt2dec2flt::PreParsed {
//!     // positive
//!     sign: false,
//!     // digits of the integer part
//!     int_digits: b"1",
//!     // digits of the fractional part
//!     frac_digits: b"25",
//!     // exponent
//!     exp: 20,
//! };
//! // Which can be converted to a floating point number:
//! assert_eq!(f32::from_preparsed(preparsed).unwrap(), 1.25e20);
//! ```

#![deny(
    rust_2018_idioms,
    trivial_numeric_casts,
    unreachable_pub,
    unused_must_use,
    unused_qualifications
)]
//#![forbid(unsafe_code)]
#![no_std]

#[cfg(test)]
extern crate std;

#[rustfmt::skip]
#[allow(clippy::all, trivial_numeric_casts, unreachable_pub, unused_qualifications)]
mod core_num;

#[cfg(test)]
mod tests;

mod sealed {
    pub trait Sealed {}
}

/// Minimum buffer size that has to be passed to `FloatExt::preformat_shortest`.
pub const PREFORMAT_SHORTEST_BUF_LEN: usize = core_num::flt2dec::MAX_SIG_DIGITS;

/// Minimum base buffer size that has to be passed to `FloatExt::preformat_exact_fixed`.
// See comment in `core_num::flt2dec::estimate_max_buf_len` for the origin of the value
pub const PREFORMAT_EXACT_FIXED_BASE_BUF_LEN: usize = 826;

/// Represents a pre-formatted floating point number.
///
/// Returned by `flt2dec2flt::f{32,64}::format_{shortest,exact_fixed,exact_exp}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PreFormatted<'a> {
    /// The number is NaN.
    NaN,
    /// The is infinity. The boolean specifies the sign.
    Inf(bool),
    /// The number is (after a possible rounding made by the representation) absolute
    /// zero. The boolean specifies the sign.
    Zero(bool),
    /// The number is finite. The boolean specifies the sign, the slice
    /// specifies the mantissa digits, the `usize` specifies extra zeros
    /// at the right and the `u16` specifies the exponent.
    ///
    /// The represented value is `sign 0.mant * 10 ^ exp`
    Finite(bool, &'a [u8], usize, i16),
}

/// A pre-parsed decimal floating point number.
///
/// The represented value is `sign int_digits.frac_digits * 10 ^ exp`.
///
/// Passed to `flt2dec2flt::f{32,64}::from_preparsed`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PreParsed<'a> {
    pub sign: bool,
    pub int_digits: &'a [u8],
    pub frac_digits: &'a [u8],
    pub exp: i16,
}

/// This trait is used to extend `f32` and `f64`.
///
/// Provides low-level methods to convert floating point numbers
/// to decimal strings and vice versa.
pub trait FloatExt: sealed::Sealed + Sized {
    /// Pre-formats `self` with the lowest lowest number of significant
    /// digits without lossing precision.
    ///
    /// `buf` must be at least `flt2dec2flt::PREFORMAT_SHORTEST_BUF_LEN` long.
    ///
    /// # Example
    ///
    /// ```
    /// use flt2dec2flt::FloatExt as _;
    ///
    /// let mut buf = [0; flt2dec2flt::PREFORMAT_SHORTEST_BUF_LEN];
    ///
    /// let preformatted = f32::preformat_shortest(12.34, &mut buf);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"1234", 0, 2),
    /// );
    ///
    /// let preformatted = f32::preformat_shortest(0.00401, &mut buf);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"401", 0, -2),
    /// );
    ///
    /// let preformatted = f32::preformat_shortest(330.0, &mut buf);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"33", 0, 3),
    /// );
    ///
    /// let preformatted = f32::preformat_shortest(4.58e31, &mut buf);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"458", 0, 32),
    /// );
    ///
    /// let preformatted = f32::preformat_shortest(4.58e-31, &mut buf);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"458", 0, -30),
    /// );
    /// ```
    fn preformat_shortest(self, buf: &mut [u8]) -> PreFormatted<'_>;

    /// Pre-formats a `f32` with an exact number of significant digits.
    ///
    /// `buf` must be at least `num_digits` long.
    ///
    /// # Example
    ///
    /// ```
    /// use flt2dec2flt::FloatExt as _;
    ///
    /// let mut buf = [0; 100];
    ///
    /// let preformatted = f32::preformat_exact_exp(200.0, &mut buf, 2);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"20", 0, 3),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_exp(0.012, &mut buf, 3);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"120", 0, -1),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_exp(12.34, &mut buf, 5);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"12340", 0, 2),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_exp(12.3456, &mut buf, 5);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"12346", 0, 2),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_exp(4.0, &mut buf, 10);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"4000000000", 0, 1),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_exp(4.0, &mut buf, 100);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"40000000000000000000000000000000000000", 62, 1),
    /// );
    /// ```
    fn preformat_exact_exp(self, buf: &mut [u8], num_digits: usize) -> PreFormatted<'_>;

    /// Pre-formats a `f32` with an exact number of fractional digits.
    ///
    /// `buf` must be at least `flt2dec2flt::PREFORMAT_EXACT_FIXED_BASE_BUF_LEN + num_frac_digits`.
    ///
    /// ```
    /// use flt2dec2flt::FloatExt as _;
    ///
    /// let mut buf = [0; flt2dec2flt::PREFORMAT_EXACT_FIXED_BASE_BUF_LEN + 10];
    ///
    /// let preformatted = f32::preformat_exact_fixed(12.34, &mut buf, 4);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"123400", 0, 2),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_fixed(12.3456, &mut buf, 2);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"1235", 0, 2),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_fixed(200.0, &mut buf, 2);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"20000", 0, 3),
    /// );
    ///
    /// // Note that leading zeros count as digits but are omitted.
    /// let preformatted = f32::preformat_exact_fixed(0.03, &mut buf, 3);
    /// assert_eq!(
    ///     preformatted,
    ///     flt2dec2flt::PreFormatted::Finite(false, b"30", 0, -1),
    /// );
    ///
    /// let preformatted = f32::preformat_exact_fixed(0.3e-4, &mut buf, 2);
    /// assert_eq!(preformatted, flt2dec2flt::PreFormatted::Zero(false));
    /// ```
    fn preformat_exact_fixed(self, buf: &mut [u8], num_frac_digits: usize) -> PreFormatted<'_>;

    /// Creates a floating point number from a pre-parsed decimal
    /// floating point number (see `PreParsed`).
    ///
    /// # Example
    ///
    /// ```
    /// use flt2dec2flt::FloatExt as _;
    ///
    /// let v = f32::from_preparsed(flt2dec2flt::PreParsed {
    ///     sign: false,
    ///     int_digits: b"12",
    ///     frac_digits: b"34",
    ///     exp: 0,
    /// });
    /// assert!((v.unwrap() - 12.34).abs() < 1e-9);
    ///
    /// let v = f32::from_preparsed(flt2dec2flt::PreParsed {
    ///     sign: false,
    ///     int_digits: b"0",
    ///     frac_digits: b"41",
    ///     exp: -4,
    /// });
    /// assert!((v.unwrap() - 0.41e-4).abs() < 1e-12);
    /// ```
    fn from_preparsed(preparsed: PreParsed<'_>) -> Option<Self>;
}

mod generic {
    use crate::core_num::flt2dec::decoder::DecodableFloat;
    use crate::{core_num, PreFormatted, PreParsed};

    pub(crate) fn preformat_shortest<T: DecodableFloat>(v: T, buf: &mut [u8]) -> PreFormatted<'_> {
        let (sign, full_decoded) = core_num::flt2dec::decoder::decode(v);
        match full_decoded {
            core_num::flt2dec::decoder::FullDecoded::Nan => PreFormatted::NaN,
            core_num::flt2dec::decoder::FullDecoded::Infinite => PreFormatted::Inf(sign),
            core_num::flt2dec::decoder::FullDecoded::Zero => PreFormatted::Zero(sign),
            core_num::flt2dec::decoder::FullDecoded::Finite(ref decoded) => {
                let (digits, exp) =
                    core_num::flt2dec::strategy::grisu::format_shortest(decoded, buf);
                PreFormatted::Finite(sign, digits, 0, exp)
            }
        }
    }

    pub(crate) fn preformat_exact_exp<T: DecodableFloat>(
        v: T,
        buf: &mut [u8],
        ndigits: usize,
    ) -> PreFormatted<'_> {
        let (sign, full_decoded) = core_num::flt2dec::decoder::decode(v);
        match full_decoded {
            core_num::flt2dec::decoder::FullDecoded::Nan => PreFormatted::NaN,
            core_num::flt2dec::decoder::FullDecoded::Infinite => PreFormatted::Inf(sign),
            core_num::flt2dec::decoder::FullDecoded::Zero => PreFormatted::Zero(sign),
            core_num::flt2dec::decoder::FullDecoded::Finite(ref decoded) => {
                // Similar as done in `core::num::flt2dec::to_exact_exp_str`

                let maxlen = core_num::flt2dec::estimate_max_buf_len(decoded.exp);
                let trunc = if ndigits < maxlen { ndigits } else { maxlen };

                let (digits, exp) = core_num::flt2dec::strategy::grisu::format_exact(
                    decoded,
                    &mut buf[..trunc],
                    i16::min_value(),
                );
                PreFormatted::Finite(sign, digits, ndigits - digits.len(), exp)
            }
        }
    }

    pub(crate) fn preformat_exact_fixed<T: DecodableFloat>(
        v: T,
        buf: &mut [u8],
        frac_digits: usize,
    ) -> PreFormatted<'_> {
        let (sign, full_decoded) = core_num::flt2dec::decoder::decode(v);
        match full_decoded {
            core_num::flt2dec::decoder::FullDecoded::Nan => PreFormatted::NaN,
            core_num::flt2dec::decoder::FullDecoded::Infinite => PreFormatted::Inf(sign),
            core_num::flt2dec::decoder::FullDecoded::Zero => PreFormatted::Zero(sign),
            core_num::flt2dec::decoder::FullDecoded::Finite(ref decoded) => {
                // Similar as done in `core::num::flt2dec::to_exact_fixed_str`

                let maxlen = core_num::flt2dec::estimate_max_buf_len(decoded.exp);

                // it *is* possible that `frac_digits` is ridiculously large.
                // `format_exact` will end rendering digits much earlier in this case,
                // because we are strictly limited by `maxlen`.
                let limit = if frac_digits < 0x8000 {
                    -(frac_digits as i16)
                } else {
                    i16::min_value()
                };
                let (digits, exp) = core_num::flt2dec::strategy::grisu::format_exact(
                    decoded,
                    &mut buf[..maxlen],
                    limit,
                );
                if exp <= limit {
                    // the restriction couldn't been met, so this should render like zero no matter
                    // `exp` was. this does not include the case that the restriction has been met
                    // only after the final rounding-up; it's a regular case with `exp = limit + 1`.
                    PreFormatted::Zero(sign)
                } else {
                    let num_zeros = if exp > 0 {
                        let ndigits = frac_digits + exp as usize;
                        ndigits - digits.len()
                    } else {
                        0
                    };
                    PreFormatted::Finite(sign, digits, num_zeros, exp)
                }
            }
        }
    }

    pub(crate) fn from_preparsed<T: core_num::dec2flt::rawfp::RawFloat>(
        preparsed: PreParsed<'_>,
    ) -> Option<T> {
        // `core_num::dec2flt` does not handle cases where `exp` has
        // more than 18 digits, but it cannot be the case here because
        // we use `i16`.
        let parsed = core_num::dec2flt::parse::Decimal::new(
            preparsed.int_digits,
            preparsed.frac_digits,
            i64::from(preparsed.exp),
        );
        let v = core_num::dec2flt::convert::<T>(parsed).ok()?;
        if preparsed.sign {
            Some(-v)
        } else {
            Some(v)
        }
    }
}

impl sealed::Sealed for f32 {}
impl sealed::Sealed for f64 {}

impl FloatExt for f32 {
    fn preformat_shortest(self, buf: &mut [u8]) -> PreFormatted<'_> {
        generic::preformat_shortest(self, buf)
    }

    fn preformat_exact_exp(self, buf: &mut [u8], num_digits: usize) -> PreFormatted<'_> {
        generic::preformat_exact_exp(self, buf, num_digits)
    }

    fn preformat_exact_fixed(self, buf: &mut [u8], num_frac_digits: usize) -> PreFormatted<'_> {
        generic::preformat_exact_fixed(self, buf, num_frac_digits)
    }

    fn from_preparsed(preparsed: PreParsed<'_>) -> Option<Self> {
        generic::from_preparsed(preparsed)
    }
}

impl FloatExt for f64 {
    fn preformat_shortest(self, buf: &mut [u8]) -> PreFormatted<'_> {
        generic::preformat_shortest(self, buf)
    }

    fn preformat_exact_exp(self, buf: &mut [u8], num_digits: usize) -> PreFormatted<'_> {
        generic::preformat_exact_exp(self, buf, num_digits)
    }

    fn preformat_exact_fixed(self, buf: &mut [u8], num_frac_digits: usize) -> PreFormatted<'_> {
        generic::preformat_exact_fixed(self, buf, num_frac_digits)
    }

    fn from_preparsed(preparsed: PreParsed<'_>) -> Option<Self> {
        generic::from_preparsed(preparsed)
    }
}
