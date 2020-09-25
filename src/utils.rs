use crate::float::*;

/// Extracts bits form x using given left-shifted mask as unsigned integer
/// (right-shifted back).
#[cfg(test)]
pub fn extract_bits(x: F, mask: U, shift: U) -> U {
    let xbits = x.to_bits();
    let mask = mask << shift;
    let m = xbits & mask;
    let m = m >> shift;
    m
}

/// Returns absolute value of x.
pub fn abs(x: F) -> F {
    let xbits = x.to_bits();
    let ybits = xbits & !SIGN_MASK;
    F::from_bits(ybits)
}

/// Returns absolute value and sign of x.
pub fn abs_sgn(x: F) -> (F, F) {
    let xbits = x.to_bits();

    let ybits = xbits & !SIGN_MASK;

    let sbits = xbits & SIGN_MASK;
    let sbits = sbits | (EXP_BIAS << MANTISSA_BITS) as U;

    (F::from_bits(ybits), F::from_bits(sbits))
}

/// Rounds x to nearest 32-bit integer. Hence, it only works for the doubles
/// whose nearest integer fits in a 32-bit machine signed integer.
pub fn round_small(x: F) -> I {
    let t = (x as f64) + ROUND_ADD;
    let tbits = t.to_bits();
    (tbits & ROUND_MASK) as I
}

/// Decomposes x into real f and integer n such that
///
/// ```plain
///     x = f * 2^n and 1 <= |f| < 2.
/// ```
///
/// Since this is the machine representation of floating point number, this
/// decomposition is exact.
pub fn decompose(x: F) -> (F, I) {
    let xbits = x.to_bits();

    let fbits = xbits & !EXP_MASK;
    let fbits = fbits | (EXP_BIAS as U) << MANTISSA_BITS;

    let nbits = xbits & EXP_MASK;
    let nbits = (nbits >> MANTISSA_BITS) as I - EXP_BIAS;

    (F::from_bits(fbits), nbits)
}

/// Restricts a value to a certain interval.
pub fn clamp(x: I, min: I, max: I) -> I {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

/// Multiplies x by 2^n.
pub fn scale(x: F, n: I) -> F {
    let xbits = x.to_bits();
    let ebits = xbits & EXP_MASK;
    let e = (ebits >> MANTISSA_BITS) as I;
    let e = clamp(e + n, 0, EXP_MAX);
    let ebits = (e << MANTISSA_BITS) as U;
    let xbits = xbits & !EXP_MASK;
    let xbits = xbits | ebits;
    F::from_bits(xbits)
}

/// Decomposes x into integer k and real y such that
///
/// ```plain
///     x = k * cst + y and |y| < cst / 2.
/// ```
///
/// It must hold that cst_inv = 1 / cst (explicit inverse is required because it
/// is more precise to compute the inverse of a number that cannot be stored in
/// finite precision and then round it to nearest).
pub fn reduce(x: F, cst: F, cst_inv: F) -> (I, F) {
    let k = round_small(x * cst_inv);
    let kd = k as F;
    let y = x - kd * cst;
    (k, y)
}

/// Optimized version of reduce(x, 1, 1), that is, it decomposes x into integer
/// k and real y such that
///
/// ```plain
///     x = k + y and |y| < 0.5.
/// ```
///
/// For decomposing the number into its integral and fractional parts, use
/// `trunc_fract`.
pub fn reduce1(x: F) -> (I, F) {
    let k = round_small(x);
    let kd = k as F;

    (k, x - kd)
}

/// Decomposes x into its integral and fractional parts, that is, into integer k
/// and real y such that
///
/// ```plain
///     x = k + y and 0 <= y < 1.
/// ```
pub fn trunc_fract(x: F) -> (I, F) {
    let (k, y) = reduce1(x);
    if y < 0.0 {
        (k - 1, y + 1.0)
    } else {
        (k, y)
    }
}

/// Compares x with a with given tolerance.
pub fn nearly_equal(x: F, a: F, tol: F) -> bool {
    abs(x - a) <= tol
}

/// Determines if n is even integer.
pub fn is_even(n: I) -> bool {
    n & 0x1 == 0x0
}

/// Determines if n is odd integer.
pub fn is_odd(n: I) -> bool {
    n & 0x1 == 0x1
}

fn is_modulo_mask(mut m: U) -> bool {
    for _ in 0..(8 * core::mem::size_of::<U>()) {
        if m & 0x1 == 0 {
            return m == 0;
        }

        m >>= 1;
    }

    true
}

/// Calculates n modulo m, where m is always positive.
pub fn modulo_mask(n: I, m: U) -> U {
    debug_assert!(is_modulo_mask(m));
    (n & (m as I)) as U
}

/// A shortcut for `F::from_bits`.
pub fn f(x: U) -> F {
    F::from_bits(x)
}

// Fused-multiply add operation (x * m + a).
pub fn fma(x: F, m: F, a: F) -> F {
    x * m + a
}

pub fn poly(x: F, coeffs: [U; 5]) -> F {
    let p = f(coeffs[4]);
    let p = fma(x, p, f(coeffs[3]));
    let p = fma(x, p, f(coeffs[2]));
    let p = fma(x, p, f(coeffs[1]));
    let p = fma(x, p, f(coeffs[0]));

    p
}

#[cfg(test)]
mod tests {
    use crate::float::EPSILON;
    use nikisas_test::float::FloatExt;
    use proptest::prelude::*;

    #[test]
    fn extract_bits() {
        assert_eq!(super::extract_bits(1.75, 0x3, 21), 3);
        assert_eq!(super::extract_bits(-0.875, 0x1, 31), 1);
        use crate::float::{EXP_BIAS, MANTISSA_BITS};
        assert_eq!(
            super::extract_bits(1792.0, 0xff, MANTISSA_BITS) as i32 - EXP_BIAS,
            super::decompose(1792.0).1
        );
    }

    proptest! {
        #[test]
        fn abs_sgn(x: f32) {
            if x.is_finite() {
                let x = if x == -0.0 { 0.0 } else { x };
                let (abs, sgn) = super::abs_sgn(x);
                assert!(abs >= 0.0);
                assert_eq!(sgn, if x >= 0.0 { 1.0 } else { -1.0 });
            }
        }
    }

    proptest! {
        #[test]
        fn round_small(x in -1000.0f32..1000.0) {
            // Our implementation has different rounding rules for values
            // exactly between two integers. For our purposes, that is fine.
            fn round(x: f32) -> f32 {
                let rounded = x.round();
                if (x - rounded).abs() == 0.5 {
                    rounded - x.signum()
                } else {
                    rounded
                }
            }
            if x.is_finite() {
                assert_eq!(super::round_small(x) as f32, round(x));
            }
        }
    }

    proptest! {
        #[test]
        fn decompose(x: f32) {
            if x.is_finite() && x != 0.0 {
                let (y, n) = super::decompose(x);
                assert!(y.abs() >= 1.0 && y.abs() < 2.0);
                assert!((y * 2.0f32.powi(n) - x).abs() <= 0.000_000_1);
            }
        }
    }

    proptest! {
        #[test]
        fn clamp(x: i32, middle: i32) {
            let min = middle.saturating_sub(10);
            let max = middle.saturating_add(10);
            let y = super::clamp(x, min, max);
            assert!(y >= min && y <= max);

            let min = x.saturating_sub(10);
            let max = x.saturating_add(10);
            let y = super::clamp(x, min, max);
            assert!(y >= min && y <= max);
        }
    }

    proptest! {
        #[test]
        fn scale(y in 1.0f32..2.0, n in -126i32..127) {
            let x = super::scale(y, n);
            assert!((y * 2.0f32.powi(n) - x).abs() <= 0.000_000_1);
        }
    }

    proptest! {
        #[test]
        fn reduce(x in -100.0f32..100.0, cst in 1.0f32..16.0) {
            if x.is_finite() {
                let cst_inv = 1.0 / cst;
                let (k, y) = super::reduce(x, cst, cst_inv);
                assert_eq!((k as f32) * cst + y, x);
                assert!(y.abs() <= cst / 2.0);
            }
        }
    }

    #[test]
    fn reduce_special() {
        let data = [
            (
                -2.1e+9,
                core::f32::consts::PI / 2.0,
                2.0 / core::f32::consts::PI,
            ),
            (
                2.1e+9,
                core::f32::consts::PI / 2.0,
                2.0 / core::f32::consts::PI,
            ),
        ];

        for &(x, cst, cst_inv) in data.iter() {
            let (k, y) = super::reduce(x, cst, cst_inv);
            assert_eq!((k as f32) * cst + y, x);
            assert!(y.abs() <= cst / 2.0);
        }
    }

    proptest! {
        #[test]
        fn reduce1(x in -1000.0f32..1000.0) {
            if x.is_finite() {
                let (k, y) = super::reduce1(x);
                assert_eq!((k as f32) + y, x);
                assert!(y.abs() <= 0.5);
                assert_eq!((k, y), super::reduce(x, 1.0, 1.0));
            }
        }
    }

    proptest! {
        #[test]
        fn trunc_fract(x in -1000.0f32..1000.0) {
            if x.is_finite() {
                let (k, y) = super::trunc_fract(x);
                assert_eq!((k as f32) + y, x);
                assert!(y >= 0.0 && y < 1.0);
            }
        }
    }

    #[test]
    fn nearly_equal() {
        let data = [0.0, 1.0, -1.0];

        for &x in data.iter() {
            assert!(super::nearly_equal(x, x, EPSILON));
            assert!(super::nearly_equal(x.nextup(), x, EPSILON));
            assert!(super::nearly_equal(x.nextdown(), x, EPSILON));
        }
    }

    #[test]
    fn integers() {
        let data = -16..16;
        let mask = 0x3;
        let m = 4;

        for n in data {
            let k = super::modulo_mask(n, mask);
            assert!(k < m);

            let even = super::is_even(n);
            let odd = super::is_odd(n);
            assert!(even || odd && !(even && odd));
        }

        assert!(!super::is_even(3));
        assert!(super::is_even(2));
        assert!(super::is_odd(3));
        assert!(!super::is_odd(2));

        assert!(super::is_modulo_mask(0x3));
        assert!(!super::is_modulo_mask(0x2));
    }
}
