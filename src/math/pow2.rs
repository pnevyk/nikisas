use super::data::POLY_POW2;
use super::pow::pow_reduce;
use crate::float::{EPSILON, F};
use crate::utils::{nearly_equal, poly, scale};

/// Computes 2 raised to a power.
///
/// # Notes
///
/// The input domain is limited to approximately [log2(min(positive f32)),
/// log2(max(f32))] ≈ [-126.0, 127.9] due to limits of machine representation.
///
/// # Examples
///
/// ```
/// use nikisas::pow2;
/// assert_eq!(pow2(-1.0), 0.5);
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
/// Let us denote z = |y|. Approximation of 2^z is done using polynomial in the
/// form:
///
/// ```plain
///   2^z ≈ 1 + z * P(z)
/// ```
///
/// The "prefix" corresponds to coefficients of low-degree Taylor polynomial of
/// 2^z for z = 0 and P is found using special minimax algorithm in Sollya.
///
/// Now we have
///
/// ```plain
///   2^y = if y ≥ 0 then 2^z else 1 / 2^z
/// ```
///
/// The reconstruction of original value is then
///
/// ```plain
/// 2^x = 2^(k + y) = 2^k * 2^y
/// ```
///
/// Computation of 2^y is (transitively) done using aforementioned polynomial
/// approximation and multiplying by 2^k can be implemented exactly using bit
/// manipulation of floating point number representation.
pub fn pow2(p: F) -> F {
    if nearly_equal(p, 0.0, EPSILON) {
        return 1.0;
    }

    let (k, z, inv) = pow_reduce(p);

    let pow2z = 1.0 + z * poly(z, POLY_POW2);
    let pow2z = if inv { 1.0 / pow2z } else { pow2z };

    scale(pow2z, k)
}

#[cfg(test)]
mod tests {
    use crate::float::F;
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;

    #[test]
    fn pow2() {
        (0..32)
            .fold(Error::with_bounds(error_bounds()), |mut error, k| {
                let y = (1u32 << k) as F;
                error.calculate(y, super::pow2(k as F), y);
                error
            })
            .assert();

        UniformSample::with_count(-0.5, 0.5, 100000)
            .assert(error_bounds(), |x| (super::pow2(x), x.exp2()));

        UniformSample::with_count(-87.3, 88.7, 10000)
            .assert(error_bounds(), |x| (super::pow2(x), x.exp2()));
    }
}
