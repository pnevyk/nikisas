use super::data::LOG10_E;
use super::ln::ln;
use crate::float::{EPSILON, F};
use crate::utils::{f, nearly_equal, round_small};

/// Computes decimal logarithm of a number.
///
/// # Notes
///
/// Theoretical input domain is (0, max(f32)] ≈ (0, 3.40282347e+38], but near
/// zero the values get quite inaccurate.
///
/// # Examples
///
/// ```
/// use nikisas::log10;
/// assert_eq!(log10(100.0), 2.0);
/// ```
///
/// # Implementation details
///
/// The following identity is used for computation of log10(x):
///
/// ```plain
///   log10(x) = ln(x) / ln(10) = ln(x) * log10(e)
/// ```
///
/// For computing ln(x) we use [`ln`] routine and log10(e) is precomputed
/// constant.
///
/// We would like to get exact values when the input number is a power of ten.
/// However, in this case it's not that straightforward as in [`pow2`]. We
/// fallback to the following faithful determination: If the computed value of
/// log10(x) is close to an integer, than we assume that the input was indeed a
/// power of ten. Then we return the rounded value. This is not always true
/// because the tolerance for "closeness" is a bit bigger than in other cases
/// throughout this library.
///
/// [`ln`]: fn.ln.html
/// [`pow2`]: fn.pow2.html
pub fn log10(x: F) -> F {
    let log10x = ln(x) * f(LOG10_E);
    let rounded = round_small(log10x) as F;

    if nearly_equal(log10x, rounded, 16.0 * EPSILON) {
        rounded
    } else {
        log10x
    }
}

#[cfg(test)]
mod tests {
    use crate::float::F;
    use crate::test::error_bounds;
    use nikisas_test::prelude::*;
    use nikisas_test::utils::shift_right;

    #[test]
    fn log10() {
        (0..32)
            .fold(Error::with_bounds(error_bounds()), |mut error, k| {
                let x = 10.0f32.powi(k);
                let k = k as F;
                error.calculate(k, super::log10(x), k);
                error
            })
            .assert();

        UniformSample::with_count(shift_right(0.0), 3.4e+38, 10000)
            .assert(error_bounds(), |x| (super::log10(x), x.log10()));
    }
}
