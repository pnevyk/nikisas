use super::data::LOG2_E;
use super::ln::ln;
use crate::float::F;
use crate::utils::{decompose, f};

/// Computes binary logarithm of a number.
///
/// # Notes
///
/// Theoretical input domain is (0, max(f32)] ≈ (0, 3.40282347e+38], but near
/// zero the values get quite inaccurate.
///
/// # Examples
///
/// ```
/// use nikisas::log2;
/// assert_eq!(log2(4.0), 2.0);
/// ```
///
/// # Implementation details
///
/// The following identity is used for computation of log2(x):
///
/// ```plain
///   log2(x) = ln(x) / ln(2) = ln(x) * log2(e)
/// ```
///
/// For computing ln(x) we use [`ln`] routine and log2(e) is precomputed
/// constant.
///
/// There is a special case where we can do better however, and that is when x
/// is a power of two. To determine this, x is decomposed into real y and
/// integer k such that
///
/// ```plain
///   x = y * 2^n, where 1 ≤ y < 2
/// ```
///
/// If y is equal to 1, then x = 2^n and thus is power of two. In this case, the
/// identity is as follows:
///
/// ```plain
///   log2(x) = log2(y * 2^n) = log2(y) + n * log2(2) = 0 + n * 1 = n
/// ```
///
/// [`ln`]: fn.ln.html
pub fn log2(x: F) -> F {
    let (y, n) = decompose(x);

    if y == 1.0 {
        return n as F;
    }

    ln(x) * f(LOG2_E)
}

#[cfg(test)]
mod tests {
    use crate::float::F;
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::shift_right;

    #[test]
    fn log2() {
        (0..32)
            .fold(Error::with_bounds(error_bounds()), |mut error, k| {
                let x = (1u32 << k) as F;
                let k = k as F;
                error.calculate(k, super::log2(x), k);
                error
            })
            .assert();

        UniformSample::with_count(shift_right(0.0), 3.4e+38, 10000)
            .assert(error_bounds(), |x| (super::log2(x), x.log2()));
    }
}
