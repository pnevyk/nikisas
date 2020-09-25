//! Traits and constants to abstract f32 and f64 types.

use std::fmt;
use std::ops;

use rand::distributions::uniform::SampleUniform;

/// Trait for all operations on floating point numbers that are required by the
/// crate. It also define some useful methods like [`nextup`], [`decompose`] or
/// [`floats_between`].
///
/// [`nextup`]: trait.FloatExt#method.nextup
/// [`decompose`]: trait.FloatExt#method.decompose
/// [`floats_between`]: trait.FloatExt#method.floats_between
pub trait FloatExt:
    SampleUniform
    + Copy
    + fmt::Debug
    + Default
    + PartialOrd<Self>
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::Mul<Self, Output = Self>
    + ops::Div<Self, Output = Self>
{
    /// Gives the next machine number after self.
    fn nextup(self) -> Self;

    /// Gives the previous machine number before self.
    fn nextdown(self) -> Self;

    /// Decomposes the floating number into real f and integer n, such that self
    /// = f * 2^n and 1 ≤ f < 2.
    fn decompose(self) -> (Self, i32);

    /// Gets the total number of machine numbers between self and other.
    fn floats_between(self, other: Self) -> u64;

    #[doc(hidden)]
    fn abs(self) -> Self;
    #[doc(hidden)]
    fn sqrt(self) -> Self;
    #[doc(hidden)]
    fn round(self) -> Self;
    #[doc(hidden)]
    fn modulo(self, m: i64) -> i64;
    #[doc(hidden)]
    fn zero() -> Self;
    #[doc(hidden)]
    fn one() -> Self;
    #[doc(hidden)]
    fn eps() -> Self;
}

macro_rules! nextup {
    ($value:expr, $float:ty) => {{
        debug_assert!($value.is_finite());

        let value = if $value == -0.0 { 0.0 } else { $value };

        let bits = value.to_bits();
        let bits = if value >= 0.0 { bits + 1 } else { bits - 1 };

        <$float>::from_bits(bits)
    }};
}

macro_rules! nextdown {
    ($value:expr, $float:ty) => {{
        debug_assert!($value.is_finite());
        -(-$value).nextup()
    }};
}

macro_rules! decompose {
    ($value:expr, $float:tt, $uint:ty) => {{
        let xbits = $value.to_bits();

        let fbits = xbits & !consts::$float::EXP_MASK;
        let fbits = fbits | (consts::$float::EXP_BIAS as $uint) << consts::$float::MANTISSA_DIGITS;

        let nbits = xbits & consts::$float::EXP_MASK;
        let nbits = (nbits >> consts::$float::MANTISSA_DIGITS) as i32 - consts::$float::EXP_BIAS;

        (<$float>::from_bits(fbits), nbits)
    }};
}

macro_rules! floats_between {
    ($low:expr, $high:expr, $float:tt) => {{
        let low = $low;
        let high = $high;

        if low == high {
            return 1;
        }

        assert!(low < high);

        let low_positive = low >= 0.0;
        let high_positive = high >= 0.0;

        // If the range crosses zero, we compute the result as the sum of negative
        // and positive parts. Otherwise, if we are in negative range, we swap the
        // arguments, because magnitude-wise, the original low is greater than
        // original high.
        let (low, high) = if low_positive != high_positive {
            // Subtract one because we counted zero two times.
            return low.floats_between(0.0.nextdown()) + 0.0.floats_between(high);
        } else if !low_positive {
            (-high, -low)
        } else {
            (low, high)
        };

        // Decompose numbers to f * 2^n form.
        let (f_low, n_low) = low.decompose();
        let (f_high, n_high) = high.decompose();

        let f_high = (f_high.to_bits() & consts::$float::MANTISSA_MASK) as u64;
        let f_low = (f_low.to_bits() & consts::$float::MANTISSA_MASK) as u64;

        let floats_per_exponent = 1u64 << consts::$float::MANTISSA_DIGITS;

        // Make sure that f_high > f_low.
        let (f_high, n_high) = if f_low > f_high {
            (f_high + floats_per_exponent, n_high - 1)
        } else {
            (f_high, n_high)
        };

        // Count all possible numbers between the two exponents.
        let floats = (n_low..n_high).fold(0, |acc, _| acc + floats_per_exponent);

        // Add the difference between mantissas. The count is inclusive, so we must
        // add 1 to include high boundary.
        floats + f_high - f_low + 1
    }};
}

mod consts {
    pub mod f32 {
        pub const EXP_MASK: u32 = 0x7f800000;
        pub const EXP_BIAS: i32 = 127;
        pub const MANTISSA_MASK: u32 = 0x007fffff;
        pub const MANTISSA_DIGITS: u32 = 23;
    }

    pub mod f64 {
        pub const EXP_MASK: u64 = 0x7ff0000000000000;
        pub const EXP_BIAS: i32 = 1023;
        pub const MANTISSA_MASK: u64 = 0x000fffffffffffff;
        pub const MANTISSA_DIGITS: u64 = 52;
    }
}

impl FloatExt for f32 {
    fn nextup(self) -> Self {
        nextup!(self, f32)
    }

    fn nextdown(self) -> Self {
        nextdown!(self, f32)
    }

    fn decompose(self) -> (Self, i32) {
        decompose!(self, f32, u32)
    }

    fn floats_between(self, other: Self) -> u64 {
        floats_between!(self, other, f32)
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn modulo(self, m: i64) -> i64 {
        (self.round() as i64) % m
    }

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn eps() -> Self {
        std::f32::EPSILON
    }
}

impl FloatExt for f64 {
    fn nextup(self) -> Self {
        nextup!(self, f64)
    }

    fn nextdown(self) -> Self {
        nextdown!(self, f64)
    }

    fn decompose(self) -> (Self, i32) {
        decompose!(self, f64, u64)
    }

    fn floats_between(self, other: Self) -> u64 {
        floats_between!(self, other, f64)
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn modulo(self, m: i64) -> i64 {
        (self.round() as i64) % m
    }

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn eps() -> Self {
        std::f64::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::consts::f32::{EXP_BIAS, MANTISSA_DIGITS};
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn next(x: f32) {
            if x.is_finite() {
                assert_eq!(x.nextup().nextdown(), x);
            }
        }
    }

    #[test]
    fn next_special() {
        assert!(0.0f32.nextup() > 0.0);
        assert!(0.0f32.nextdown() < 0.0);
        assert!(0.0f32.nextdown().nextup().nextup() > 0.0);
    }

    #[test]
    fn floats_between() {
        let floats_per_exponent = (1 << MANTISSA_DIGITS) as u64;
        let bias = EXP_BIAS as u64;

        assert_eq!(1.0f32.floats_between(2.0), floats_per_exponent + 1);
        assert_eq!(1.0f32.floats_between(2.0.nextdown()), floats_per_exponent);
        assert_eq!((-2.0f32).floats_between(-1.0), floats_per_exponent + 1);
        assert_eq!(3.14f32.floats_between(3.14.nextup()), 2);
        assert_eq!(0.0f32.floats_between(1.0), bias * floats_per_exponent + 1);
        assert_eq!(
            (-1.0f32).floats_between(0.0),
            bias * floats_per_exponent + 1
        );
        // assert_eq!(
        //     (-2.0f32).floats_between(0.0),
        //     bias * floats_per_exponent + 2
        // );
        // assert_eq!(
        //     (-2.0f32).floats_between(1.0),
        //     (-2.0f32).floats_between(0.0) + (0.0f32).floats_between(1.0) - 1
        // );
    }
}
