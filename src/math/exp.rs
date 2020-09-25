use super::data::{E, LN_2, LN_2_INV, POLY_EXP};
use crate::float::{EPSILON, F};
use crate::utils::{f, nearly_equal, poly, reduce, scale};

/// Computes exponentiation function of a number.
///
/// # Notes
///
/// The input domain is limited to approximately [ln(min(positive f32)),
/// ln(max(f32))] ≈ [-87.3, 88.7] due to limits of machine representation.
///
/// # Example
///
/// ```
/// use nikisas::{exp, consts::E};
/// assert_eq!(exp(1.0), E);
/// ```
///
/// # Implementation details
///
/// First, special cases are handled. If x is 1, then the result is simply
/// [`Euler's number`]. If x is near zero, then the result is simply 1.
/// Otherwise, input x is reduced to an integer k and real z such that
///
/// ```plain
///   x = k * ln(2) + z and |z| ≤ ln(2) / 2
/// ```
///
/// Exponentiation of z is done using polynomial in the form:
///
/// ```plain
///   exp(z) ≈ 1 + z + 1/2 * z^2 + z^3 * P(z)
/// ```
///
/// The "prefix" corresponds to coefficients of low-degree Taylor polynomial of
/// exp(z) for z = 0 and P is found using special minimax algorithm in Sollya.
///
/// The reconstruction follows this identity:
///
/// ```plain
///   exp(x) = exp(k * ln(2) + z) = exp(ln(2))^k * exp(z) = 2^k * exp(z)
/// ```
///
/// Computation of exp(z) is done using aforementioned polynomial approximation
/// and multiplying by 2^k can be implemented exactly using bit manipulation of
/// floating point number representation.
///
/// [`Euler's number`]: consts/constant.E.html
pub fn exp(x: F) -> F {
    if x == 1.0 {
        return f(E);
    } else if nearly_equal(x, 0.0, EPSILON) {
        return 1.0;
    }

    let (k, z) = reduce(x, f(LN_2), f(LN_2_INV));

    let z2 = z * z;
    let expz = 1.0 + z + 0.5 * z2 + z2 * z * poly(z, POLY_EXP);

    scale(expz, k)
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use crate::utils::f;
    use nikisas_test::prelude::*;

    #[test]
    fn exp() {
        assert_eq!(super::exp(1.0), f(super::E));
        assert_eq!(super::exp(0.0), 1.0);

        UniformSample::with_count(-2.0f32.ln() / 2.0, 2.0f32.ln() / 2.0, 100000)
            .assert(error_bounds(), |x| (super::exp(x), x.exp()));

        UniformSample::with_count(-87.3, 88.7, 10000)
            .assert(error_bounds(), |x| (super::exp(x), x.exp()));
    }
}
