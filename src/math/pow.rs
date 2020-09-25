use super::exp::exp;
use super::ln::ln;
use super::pow10::pow10;
use super::pow2::pow2;
use crate::float::{EPSILON, F, I};
use crate::utils::{decompose, is_odd, nearly_equal, reduce1, scale, trunc_fract};

/// Computes a number raised to a power.
///
/// # Notes
///
/// For negative bases, the power must be an integer. If it's not, then NaN is
/// returned.
///
/// Approximation of power function is *hard*. A straightforward way would be to
/// use the following identity:
///
/// ```plain
///   x^p = exp(ln(x^p)) = exp(p * ln(x))
/// ```
///
/// but that would produce very inaccurate results in general, since even a tiny
/// error in calculation of natural logarithm is exponentiated afterwards (not
/// to say that calculation exponentiation has also some error).
///
/// We fallback to partially using square-and-multiply algorithm for doing power
/// on integral parts of the exponent, which might be up to 32 iterations of a
/// small loop. This is a bit more cycles than doing some fancy approximation
/// using a fixed-degree polynomial or rational function, but in most cases
/// should be fine.
///
/// # Examples
///
/// ```
/// use nikisas::pow;
/// assert_eq!(pow(2.0, -1.0), 0.5);
/// ```
///
/// # Implementation details
///
/// First, special cases are handled:
///
/// * if x is near 1, then the result is simply 1,
/// * if p is near 1, then the result is simply x,
/// * if p is near 0, then the result is simply 1,
/// * if x is near 2, then specialized [`pow2`] is used, and
/// * if x is near 10, then specialized [`pow10`] is used.
///
/// If x is non-negative, the procedure goes like this. First, x is decomposed
/// to real y and integer n, such that
///
/// ```plain
///   x = y * 2^n, where 1 ≤ y < 2
/// ```
///
/// Second, p and q = p * n are decomposed as follows:
///
/// ```plain
///   p = pi + pf, such that pi is integer and 0 ≤ pf < 1
///   q = qi + qf, such that qi is integer and |qf| ≤ 1/2
/// ```
///
/// With all this, we can then use the following identity:
///
/// ```plain
///   x^p = (y * 2^n)^p
///       = y^p * 2^(pn)
///       = y^(pi + pf) * 2^(qi + qf)
///       = y^pi * y^pf * 2^qi * 2^qf
/// ```
///
/// For y^pi we use square-and-multiply loop algorithm, for y^pf the
/// aforementioned identity exp(pf * ln(y)) is used with hope that it does not
/// introduce too big error as it is only one term in the whole computation, for
/// 2^qf we use [`pow2`] routine, and multiplying by 2^qi can be implemented
/// exactly using bit manipulation of floating point number representation.
///
/// If x is negative, the p must be an integer. This is true when z is zero,
/// where z is the fractional part of p = k + z. If this is a case, we again
/// decompose x into x = y * 2^n. Then the same procedure as before is used,
/// only that the fractional exponents don't exist, thus
///
/// ```plain
///   y^pf = 2^qf = 1, y^pi = y^k, 2^qi = 2^(k * n)
/// ```
///
/// [`pow2`]: fn.pow2.html
/// [`pow10`]: fn.pow10.html
pub fn pow(x: F, p: F) -> F {
    if nearly_equal(x, 1.0, EPSILON) {
        return 1.0;
    } else if nearly_equal(p, 1.0, EPSILON) {
        return x;
    } else if nearly_equal(p, 0.0, EPSILON) {
        return 1.0;
    } else if nearly_equal(x, 2.0, EPSILON) {
        return pow2(p);
    } else if nearly_equal(x, 10.0, EPSILON) {
        return pow10(p);
    }

    if x >= 0.0 {
        let (y, n) = decompose(x);
        let nd = n as F;

        let (pi, pf) = trunc_fract(p);
        let (pni, pnf) = reduce1(p * nd);

        scale(square_mul(y, pi) * exp(pf * ln(y)) * pow2(pnf), pni)
    } else {
        let (k, z) = reduce1(p);
        if z == 0.0 {
            let (y, n) = decompose(x);
            scale(square_mul(y, k), n * k)
        } else {
            F::NAN
        }
    }
}

pub(crate) fn square_mul(x: F, k: I) -> F {
    let (mut k, mut base) = if k < 0 { (-k, 1.0 / x) } else { (k, x) };
    let mut r = 1.0;

    // At maximum, there are mem::size_of::<I>() * 8 iterations (32, or 64).
    // Power function is hard to approximate, let's accept this cost for now.
    loop {
        if is_odd(k) {
            r *= base;
        }

        k >>= 1;

        if k == 0 {
            break;
        }

        base *= base;
    }

    r
}

pub(crate) fn pow_reduce(x: F) -> (I, F, bool) {
    let (k, y) = reduce1(x);
    let (y, inv) = if y < 0.0 { (-y, true) } else { (y, false) };
    (k, y, inv)
}

#[cfg(test)]
mod tests {
    use crate::float::F;
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::{avoid, shift_right};

    #[test]
    fn pow() {
        assert_eq!(super::pow(3.14, 0.0), 1.0);

        UniformSample::with_count(shift_right(0.0f32), 32.0, 5000)
            .fold(Error::with_bounds(error_bounds()), |error, x| {
                UniformSample::with_count(-10.0, 10.0, 5000)
                    .filter(avoid(0.0))
                    .fold(error, |mut error, p| {
                        if x.powf(p).is_finite() {
                            error.calculate((x, p), super::pow(x, p), x.powf(p));
                        }
                        error
                    })
            })
            .assert();

        UniformSample::with_count(shift_right(0.0f32), 10.0, 5000)
            .fold(Error::with_bounds(error_bounds()), |error, x| {
                UniformSample::with_count(-64.0, 64.0, 5000)
                    .filter(avoid(0.0))
                    .fold(error, |mut error, p| {
                        if x.powf(p).is_finite() {
                            error.calculate((x, p), super::pow(x, p), x.powf(p));
                        }
                        error
                    })
            })
            .assert();

        UniformSample::with_count(-10.0f32, 10.0, 5000)
            .fold(Error::with_bounds(error_bounds()), |error, x| {
                UniformSample::with_count(1.0, 100.0, 5000)
                    .map(F::round)
                    .fold(error, |mut error, p| {
                        if x.powf(p).is_finite() {
                            error.calculate((x, p), super::pow(x, p), x.powf(p));
                        }
                        error
                    })
            })
            .assert();
    }
}
