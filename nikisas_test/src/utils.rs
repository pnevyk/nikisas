//! Useful utilities that enhance the control over the behavior of the crate.
//!
//! # Examples
//!
//! Suppose you are testing the quality of the implementation of the tangent
//! function. It is a periodic function with period π and asymptotes at -π/2 and
//! π/2, where its value is -infinity and infinity, respectively. Therefore, we
//! need to exclude these from our input domain:
//!
//! ```
//! use nikisas_test::prelude::*;
//! use nikisas_test::utils::{shift_left, shift_right, avoid_odd_mults};
//!
//! fn tan(x: f32) -> f32 {
//!     // your implementation
//!     # 0.0
//! }
//!
//! // Make the range boundaries such that -π/2 and π/2 are not included.
//! let primary_range = UniformSample::with_count(
//!     shift_right(-core::f32::consts::PI / 2.0),
//!     shift_left(core::f32::consts::PI / 2.0),
//!     100000,
//! )
//! .error(|x| (tan(x), x.tan()));
//!
//! // Filter out the odd multipliers of π/2, that is, -π/2, π/2, 3π/2, etc.
//! let entire_range = UniformSample::with_count(-2.1e+9, 2.1e+9, 10000)
//!     .filter(avoid_odd_mults(core::f32::consts::PI / 2.0))
//!     .error(|x| (tan(x), x.tan()));
//! ```
use crate::float::FloatExt;

/// Returns x - [`machine
/// epsilon`](https://en.wikipedia.org/wiki/Machine_epsilon). For finer
/// resolution, use [`nextdown`].
///
/// [`nextdown`]: ../float/trait.FloatExt#method.nextdown.html
pub fn shift_left<F: FloatExt>(x: F) -> F {
    x - F::eps()
}

/// Returns x + [`machine
/// epsilon`](https://en.wikipedia.org/wiki/Machine_epsilon). For finer
/// resolution, use [`nextup`].
///
/// [`nextup`]: ../float/trait.FloatExt#method.nextup.html
pub fn shift_right<F: FloatExt>(x: F) -> F {
    x + F::eps()
}

/// Instructs the iterator to avoid this particular value.
///
/// ```
/// use nikisas_test::prelude::*;
/// use nikisas_test::utils::avoid;
///
/// fn inv(x: f32) -> f32 {
///     // your implementation
///     # 0.0
/// }
///
/// let error = UniformSample::with_count(-4.2e+9, 4.2e+9, 10000)
///     .filter(avoid(0.0))
///     .error(|x| (inv(x), 1.0 / x));
/// ```
pub fn avoid<F: FloatExt>(x: F) -> impl Fn(&F) -> bool {
    let low = shift_left(x);
    let high = shift_right(x);
    move |&y| y < low || y > high
}

/// Instructs the iterator to avoid all multipliers of this particular value.
///
/// ```
/// use nikisas_test::prelude::*;
/// use nikisas_test::utils::avoid_mults;
///
/// fn cos(x: f32) -> f32 {
///     // your implementation
///     # 0.0
/// }
///
/// let error = UniformSample::with_count(-2.1e+9, 2.1e+9, 10000)
///     .filter(avoid_mults(core::f32::consts::PI / 2.0))
///     .error(|x| (cos(x), x.cos()));
/// ```
pub fn avoid_mults<F: FloatExt>(x: F) -> impl Fn(&F) -> bool {
    let low = shift_left(F::zero());
    let high = shift_right(F::zero());
    move |&y| {
        let rounded = (y / x).round();
        let z = y - rounded * x;
        z < low || z > high
    }
}

/// Instructs the iterator to avoid all *even* multipliers of this particular
/// value. That is, for value x, it's 2x, 4x, but not 3x. See [`avoid_mults`]
/// for usage.
///
/// [`avoid_mults`]: fn.avoid_mults.html
pub fn avoid_even_mults<F: FloatExt>(x: F) -> impl Fn(&F) -> bool {
    let low = shift_left(F::zero());
    let high = shift_right(F::zero());
    move |&y| {
        let rounded = (y / x).round();
        let z = y - rounded * x;
        (z < low || z > high) || rounded.modulo(2) == 1
    }
}

/// Instructs the iterator to avoid all *odd* multipliers of this particular
/// value. That is, for value x, it's x, 3x, but not 2x. See [`avoid_mults`] for
/// usage.
///
/// [`avoid_mults`]: fn.avoid_mults.html
pub fn avoid_odd_mults<F: FloatExt>(x: F) -> impl Fn(&F) -> bool {
    let low = shift_left(F::zero());
    let high = shift_right(F::zero());
    move |&y| {
        let rounded = (y / x).round();
        let z = y - rounded * x;
        (z < low || z > high) || rounded.modulo(2) == 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shift() {
        assert!(super::shift_right(1.0) > 1.0);
        assert!(super::shift_right(1.0) < 1.5);
        assert!(super::shift_left(1.0) < 1.0);
        assert!(super::shift_left(1.0) > 0.5);
    }

    #[test]
    fn avoid() {
        assert_eq!(super::avoid(1.0)(&1.0), false);
        assert_eq!(super::avoid(1.0)(&super::shift_right(1.0)), false);
        assert_eq!(super::avoid(1.0)(&super::shift_left(1.0)), false);
        assert_eq!(super::avoid(1.0)(&0.5), true);
        assert_eq!(super::avoid(1.0)(&1.5), true);
    }

    #[test]
    fn avoid_mults() {
        assert_eq!(super::avoid_mults(2.0)(&2.0), false);
        assert_eq!(super::avoid_mults(2.0)(&16.0), false);
        assert_eq!(super::avoid_mults(2.0)(&-16.0), false);
        assert_eq!(super::avoid_mults(2.0)(&super::shift_right(16.0)), false);
        assert_eq!(super::avoid_mults(2.0)(&super::shift_left(16.0)), false);
        assert_eq!(super::avoid_mults(2.0)(&1.5), true);
        assert_eq!(super::avoid_mults(2.0)(&2.5), true);
    }

    #[test]
    fn avoid_even_or_odd_mults() {
        assert_eq!(super::avoid_even_mults(2.0)(&16.0), false);
        assert_eq!(super::avoid_even_mults(2.0)(&14.0), true);
        assert_eq!(super::avoid_even_mults(2.0)(&15.0), true);

        assert_eq!(super::avoid_odd_mults(2.0)(&16.0), true);
        assert_eq!(super::avoid_odd_mults(2.0)(&14.0), false);
        assert_eq!(super::avoid_odd_mults(2.0)(&15.0), true);
    }
}
