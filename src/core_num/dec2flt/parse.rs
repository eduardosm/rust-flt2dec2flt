//! Validating and decomposing a decimal string of the form:
//!
//! `(digits | digits? '.'? digits?) (('e' | 'E') ('+' | '-')? digits)?`
//!
//! In other words, standard floating-point syntax, with two exceptions: No sign, and no
//! handling of "inf" and "NaN". These are handled by the driver function (super::dec2flt).
//!
//! Although recognizing valid inputs is relatively easy, this module also has to reject the
//! countless invalid variations, never panic, and perform numerous checks that the other
//! modules rely on to not panic (or overflow) in turn.
//! To make matters worse, all that happens in a single pass over the input.
//! So, be careful when modifying anything, and double-check with the other modules.

#[derive(Debug, PartialEq, Eq)]
/// The interesting parts of a decimal string.
pub struct Decimal<'a> {
    pub integral: &'a [u8],
    pub fractional: &'a [u8],
    /// The decimal exponent, guaranteed to have fewer than 18 decimal digits.
    pub exp: i64,
}

impl<'a> Decimal<'a> {
    pub fn new(integral: &'a [u8], fractional: &'a [u8], exp: i64) -> Decimal<'a> {
        Decimal { integral, fractional, exp }
    }
}
