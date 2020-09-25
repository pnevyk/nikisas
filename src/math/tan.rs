use super::data::{PI_HALF, PI_HALF_INV, PI_QUARTER, POLY_TAN};
use crate::float::{EPSILON, F};
use crate::utils::{abs_sgn, f, is_even, nearly_equal, poly, reduce};

/// Computes tangent of a number.
///
/// # Notes
///
/// The input domain is limited to approximately [-2.1e+9, 2.1e+9] due to
/// implementation details. Near asymptotes (-π/2, π/2) the values get quite
/// inaccurate.
///
/// # Examples
///
/// ```
/// use nikisas::{tan, consts::PI};
/// assert_eq!(tan(0.25 * PI), 1.0);
/// ```
///
/// # Implementation details
///
/// The input x is reduced to an integer k and real z such that
///
/// ```plain
///   x = k * π / 2 + z and |z| ≤ π / 4
/// ```
///
/// This is the reason why the input domain is limited to smaller range, because
/// the integral part must fit into 32-bit integer.
///
/// Then, the approximation is split into 2 pieces. Let's consider one period of
/// the tangent from -π/2 to π/2:
///
/// * for x in [-π/4, π/4], tan(x) = tan(z),
/// * for x in [-π/2, -π/4) ∪ (π/4, π/2], tan(x) = -1 / tan(z).
///
/// To determine in which part of the period number x falls, i suffices to check
/// if is even (first case) or odd (second case).
///
/// The tangent of z is approximated using a polynomial in the form:
///
/// ```plain
///   tan(z) ≈ z + z^3 * P(z^2)
/// ```
///
/// The "prefix" corresponds to coefficients of low-degree Taylor polynomial of
/// tan(z) for z = 0 and P is found using special minimax algorithm in Sollya.
/// The use of z^2 instead of simply z is due to the fact that the tangent is an
/// odd function (z^3 multiplier before P(z^2) is important).
///
/// There is also a special case when |z| is near π/4. Depending on the sign of
/// z, the exact values of tan(z) are 1, respectively -1. We return them without
/// employing any approximation.
pub fn tan(x: F) -> F {
    let (k, z) = reduce(x, f(PI_HALF), f(PI_HALF_INV));
    let (z_abs, z_sgn) = abs_sgn(z);

    if nearly_equal(z_abs, f(PI_QUARTER), EPSILON) {
        if z_sgn == 1.0 {
            1.0
        } else {
            -1.0
        }
    } else {
        let z2 = z * z;
        let tanz = z + z2 * z * poly(z2, POLY_TAN);

        if is_even(k) {
            tanz
        } else {
            -1.0 / tanz
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::{avoid_odd_mults, shift_left, shift_right};

    #[test]
    fn tan() {
        assert_eq!(super::tan(0.0), 0.0);
        assert_eq!(super::tan(core::f32::consts::PI * 0.25), 1.0);
        assert_eq!(super::tan(-core::f32::consts::PI * 0.25), -1.0);

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
