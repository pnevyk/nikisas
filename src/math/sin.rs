use super::data::{PI_HALF, PI_HALF_INV, POLY_COS, POLY_SIN};
use crate::float::{EPSILON, F};
use crate::utils::{f, modulo_mask, nearly_equal, poly, reduce};

/// Computes the sine of a number in radians.
///
/// # Notes
///
/// The input domain is limited to approximately [-2.1e+9, 2.1e+9] due
/// to implementation details.
///
/// # Examples
///
/// ```
/// use nikisas::{sin, consts::PI};
/// assert_eq!(sin(PI), 0.0);
/// ```
///
/// # Implementations details
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
/// Then, the approximation is split into 4 pieces. Let's consider one period of
/// the sine from -π/4 to 7π/4:
///
/// * for x in [-π/4, π/4), sin(x) = sin(z),
/// * for x in [π/4, 3π/4), sin(x) = cos(z),
/// * for x in [3π/4, 5π/4), sin(x) = -sin(z), and
/// * for x in [5π/4, 7π/4), sin(x) = -cos(z).
///
/// To determine in which part of the period number x falls, we compute
///
/// ```plain
///   i = k mod 4
/// ```
///
/// Individual parts of the period then correspond to i = 0, 1, 2, 3. The
/// approximations of sin(z) and cos(z) are done using polynomials in the form:
///
/// ```plain
///   sin(z^2) ≈ z + z^3 * P(z^2)
///   cos(z^2) ≈ 1 + z^2 * Q(z^2)
/// ```
///
/// The "prefixes" correspond to coefficients of low-degree Taylor polynomials
/// of sin(z), respectively cos(z), for z = 0 and P, Q are found using special
/// minimax algorithm in Sollya. The use of z^2 instead of simply z is due to
/// the fact that the sine is an odd function and the cosine is an even function
/// (z^3 and z^2 multipliers before P(z^2), respectively Q(z^2), are important).
///
/// There is also a special case when z is equal to zero, that is, x is 0, π/2,
/// π, 3π/2 or a periodic multiplier of one of these. We know exact values (0,
/// 1, 0, -1) for these inputs and so we return them without employing any
/// approximation.
pub fn sin(x: F) -> F {
    let (k, z) = reduce(x, f(PI_HALF), f(PI_HALF_INV));
    let i = modulo_mask(k, 0x3);

    if nearly_equal(z, 0.0, EPSILON) {
        return match i {
            0 => 0.0,
            1 => 1.0,
            2 => 0.0,
            3 => -1.0,
            _ => unreachable!(),
        };
    }

    let z2 = z * z;

    match i {
        0 => z + z2 * z * poly(z2, POLY_SIN),
        1 => 1.0 + z2 * poly(z2, POLY_COS),
        2 => -(z + z2 * z * poly(z2, POLY_SIN)),
        3 => -(1.0 + z2 * poly(z2, POLY_COS)),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;

    #[test]
    fn sin() {
        assert_eq!(super::sin(0.0), 0.0);
        assert_eq!(super::sin(core::f32::consts::PI * 0.5), 1.0);
        assert_eq!(super::sin(core::f32::consts::PI), 0.0);
        assert_eq!(super::sin(core::f32::consts::PI * 1.5), -1.0);

        UniformSample::with_count(-core::f32::consts::PI, core::f32::consts::PI, 100000)
            .assert(error_bounds(), |x| (super::sin(x), x.sin()));

        UniformSample::with_count(-2.1e+9, 2.1e+9, 10000)
            .assert(error_bounds(), |x| (super::sin(x), x.sin()));
    }
}
