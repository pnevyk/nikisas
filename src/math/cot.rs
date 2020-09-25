use super::tan::tan;
use crate::float::F;

/// Computes the cotangent of a number in radians.
///
/// # Notes
///
/// The input domain is limited to approximately [-2.1e+9, 2.1e+9] due to
/// implementation details (see [`tan`]).
///
/// # Examples
///
/// ```
/// use nikisas::{cot, consts::PI};
/// assert_eq!(cot(0.25 * PI), 1.0);
/// ```
///
/// # Implementations details
///
/// It is simply computed as 1 / tan(x) using [`tan`] routine.
///
/// [`tan`]: fn.tan.html
pub fn cot(x: F) -> F {
    1.0 / tan(x)
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::{avoid_odd_mults, shift_left, shift_right};

    #[test]
    fn cot() {
        UniformSample::with_count(
            shift_right(-core::f32::consts::PI / 2.0),
            shift_left(core::f32::consts::PI / 2.0),
            100000,
        )
        .assert(error_bounds(), |x| (super::tan(x), x.tan()));

        UniformSample::with_count(-2.1e+9, 2.1e+9, 10000)
            .filter(avoid_odd_mults(core::f32::consts::PI / 2.0))
            .assert(error_bounds(), |x| (super::tan(x), x.tan()));
    }
}
