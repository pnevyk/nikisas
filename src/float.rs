pub type F = f32;
pub type I = i32;
pub type U = u32;

/// Mask for exponent value in single-precision floating point number.
pub const EXP_MASK: U = 0x7f800000;

/// Exponent bias in single-precision floating point number.
pub const EXP_BIAS: I = 127;

/// Maximum exponent value in single-precision floating point number.
pub const EXP_MAX: I = 255;

/// Right offset of exponent value in single-precision floating point number.
pub const MANTISSA_BITS: U = 23;

/// Sign mask in single-precision floating point number.
pub const SIGN_MASK: U = 0x80000000;

/// Constant 2^52 + 2^51 for being used in `round` function.
pub const ROUND_ADD: f64 = 6755399441055744.0;

/// Mask for getting lower 32 bits from double-precision floating point number.
pub const ROUND_MASK: u64 = 0xffffffff;

/// Equality check tolerance, equal to MACHINE_EPSILON.
pub const EPSILON: F = 1.19209290e-07;
