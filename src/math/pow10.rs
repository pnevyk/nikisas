use super::data::POLY_POW10;
use super::pow::{pow_reduce, square_mul};
use crate::float::{EPSILON, F};
use crate::utils::{nearly_equal, poly};

/// Computes 10 raised to a power.
///
/// # Notes
///
/// The input domain is limited to approximately [log10(min(positive f32)),
/// log10(max(f32))] ≈ [-37.9, 38.5] due to limits of machine representation.
///
/// # Examples
///
/// ```
/// use nikisas::pow10;
/// assert_eq!(pow10(-1.0), 0.1);
/// ```
///
/// # Implementation details
///
/// First, the special case when x is near zero is handled such that the result
/// is simply 1. Otherwise, the input x is reduced to an integer k and real y
/// such that
///
/// ```plain
///   x = k + y and |y| ≤ 1/2
/// ```
///
/// Let us denote z = |y|. Approximation of 10^z is done using polynomial in the
/// form:
///
/// ```plain
///   10^z ≈ 1 + z * P(z)
/// ```
///
/// The "prefix" corresponds to coefficients of low-degree Taylor polynomial of
/// 10^z for z = 0 and P is found using special minimax algorithm in Sollya.
///
/// Now we have
///
/// ```plain
///   10^y = if y ≥ 0 then 10^z else 1 / 10^z
/// ```
///
/// The reconstruction of original value is then
///
/// ```plain
/// 10^x = 10^(k + y) = 10^k * 10^y
/// ```
///
/// Computation of 10^y is (transitively) done using aforementioned polynomial
/// approximation and multiply-and-square loop algorithm is used for computation
/// of 10^k. Note that in this case, the maximum number of iterations is limited
/// by log2(max(|input range of x|)) < 6.
pub fn pow10(p: F) -> F {
    if nearly_equal(p, 0.0, EPSILON) {
        return 1.0;
    }

    let (k, z, inv) = pow_reduce(p);

    let pow10z = 1.0 + z * poly(z, POLY_POW10);
    let pow10z = if inv { 1.0 / pow10z } else { pow10z };

    square_mul(10.0, k) * pow10z
}

#[cfg(test)]
mod tests {
    use crate::float::F;
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;

    #[test]
    fn pow10() {
        (0..32)
            .fold(Error::with_bounds(error_bounds()), |mut error, k| {
                let y = 10.0f32.powi(k);
                error.calculate(y, super::pow10(k as F), y);
                error
            })
            .assert();

        UniformSample::with_count(-0.5, 0.5, 100000)
            .assert(error_bounds(), |x| (super::pow10(x), 10.0f32.powf(x)));

        UniformSample::with_count(-37.9, 38.5, 10000)
            .assert(error_bounds(), |x| (super::pow10(x), 10.0f32.powf(x)));
    }
}
