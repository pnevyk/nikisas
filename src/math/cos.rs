use super::data::PI_HALF;
use super::sin::sin;
use crate::float::F;
use crate::utils::f;

/// Computes the cosine of a number in radians.
///
/// # Notes
///
/// The input domain is limited to approximately [-2.1e+9, 2.1e+9] due to
/// implementation details (see [`sin`]).
///
/// # Examples
///
/// ```
/// use nikisas::{cos, consts::PI};
/// assert_eq!(cos(PI), -1.0);
/// ```
///
/// # Implementations details
///
/// It is simply computed as sin(x + pi/2) using [`sin`] routine.
///
/// [`sin`]: fn.sin.html
pub fn cos(x: F) -> F {
    sin(x + f(PI_HALF))
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;

    #[test]
    fn cos() {
        assert_eq!(super::cos(0.0), 1.0);
        assert_eq!(super::cos(core::f32::consts::PI * 0.5), 0.0);
        assert_eq!(super::cos(core::f32::consts::PI), -1.0);
        assert_eq!(super::cos(core::f32::consts::PI * 1.5), 0.0);

        UniformSample::with_count(-core::f32::consts::PI, core::f32::consts::PI, 100000)
            .assert(error_bounds(), |x| (super::cos(x), x.cos()));

        UniformSample::with_count(-2.1e+9, 2.1e+9, 10000)
            .assert(error_bounds(), |x| (super::cos(x), x.cos()));
    }
}
