use super::data::{E, LN_2, POLY_LN1P, SQRT_2};
use crate::float::{EPSILON, F};
use crate::utils::{decompose, f, nearly_equal, poly};

/// Computes natural logarithm of a number.
///
/// # Notes
///
/// Theoretical input domain is (0, max(f32)] ≈ (0, 3.40282347e+38], but near
/// zero the values get quite inaccurate.
///
/// # Examples
///
/// ```
/// use nikisas::{ln, consts::E};
/// assert_eq!(ln(E), 1.0);
/// ```
///
/// # Implementation details
///
/// First, special cases are handled. If x is 1, then the result is simply 0. If
/// x is near [`Euler's number`], then the result is simply 1. Otherwise, the
/// input x is decomposed into real y and integer k such that
///
/// ```plain
///   x = y * 2^n, where 1 ≤ y < 2
/// ```
///
/// At this point, we might use
///
/// ```plain
/// ln(x) = n * ln(2) + ln(y)
/// ```
///
/// but that would lead to [catastrophic
/// cancellation](https://en.wikipedia.org/wiki/Loss_of_significance) for cases
/// where n = -1 and y ≈ 2. Therefore, we adjust the decomposition if y >
/// sqrt(2) as follows
///
/// ```plain
///   y <- y / 2
///   n <- n + 1
/// ```
///
/// This keeps the equality `x = y * 2^n` true, but avoids mentioned
/// cancellation and shifts y to be symmetric around logarithm's root at 1, in
/// range [1/sqrt(2), sqrt(2)].
///
/// For reasons unclear to me, polynomial approximation of ln(1 + z) for -a/2 ≤
/// z < a/2 is easier than of ln(z) for 1 - a/2 ≤ z < 1 + a/2. Thus, we set z =
/// y - 1 and approximate ln(1 + z) using a polynomial in the form:
///
/// ```plain
///   ln(1 + z) ≈ z - 1/2 * z^2 + z^3 * P(z)
/// ```
///
/// The "prefix" corresponds to coefficients of low-degree Taylor polynomial of
/// ln(1 + z) in z = 0 and P is found using special minimax algorithm in Sollya.
///
/// The reconstruction then follows already mentioned identity:
///
/// ```plain
///   ln(x) = n * ln(2) + ln(y) = n * ln(2) + ln(1 + z)
/// ```
///
/// [`Euler's number`]: consts/constant.E.html
pub fn ln(x: F) -> F {
    if x == 1.0 {
        return 0.0;
    } else if nearly_equal(x, f(E), EPSILON) {
        return 1.0;
    }

    let (y, n) = decompose(x);

    let (y, n) = if y > f(SQRT_2) {
        (y * 0.5, n + 1)
    } else {
        (y, n)
    };

    let z = y - 1.0;
    let z2 = z * z;
    let lny = z - 0.5 * z2 + z2 * z * poly(z, POLY_LN1P);

    let n = n as F;
    n * f(LN_2) + lny
}

#[cfg(test)]
mod tests {
    use crate::test::error_bounds;
    use crate::utils::f;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::shift_right;

    #[test]
    fn ln() {
        assert_eq!(super::ln(1.0), 0.0);
        assert_eq!(super::ln(f(super::E)), 1.0);

        UniformSample::with_fraction(1.0 / 2.0f32.sqrt(), 2.0f32.sqrt(), 0.5)
            .assert(error_bounds(), |x| (super::ln(x), x.ln()));

        UniformSample::with_count(shift_right(0.0), 3.4e+38, 10000)
            .assert(error_bounds(), |x| (super::ln(x), x.ln()));
    }
}
