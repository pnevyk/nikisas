use crate::float::U;

pub(crate) const E: U = 0x402df854;
pub(crate) const LN_2: U = 0x3f317218;
pub(crate) const LN_2_INV: U = 0x3fb8aa3b;
pub(crate) const SQRT_2: U = 0x3fb504f3;
pub(crate) const LOG2_E: U = 0x3fb8aa3b;
pub(crate) const LOG10_E: U = 0x3ede5bd9;
pub(crate) const PI_HALF: U = 0x3fc90fdb;
pub(crate) const PI_HALF_INV: U = 0x3f22f983;
pub(crate) const PI_QUARTER: U = 0x3f490fdb;

pub(crate) const POLY_EXP: [U; 5] = [0x3e2aaa83, 0x3d2aaa70, 0x3c08c01f, 0x3ab6aaed, 0x39063f86];
pub(crate) const POLY_LN1P: [U; 5] = [0x3eaa95d3, 0xbe7f5a82, 0x3e51db4d, 0xbe3d687c, 0x3defc7b9];
pub(crate) const POLY_POW2: [U; 5] = [0x3f31721a, 0x3e75fcfc, 0x3d637c2c, 0x3c1b5267, 0x3acf2bc8];
pub(crate) const POLY_POW10: [U; 5] = [0x4013623b, 0x402929c4, 0x40069c52, 0x3f694226, 0x3f7749be];
pub(crate) const POLY_SIN: [U; 5] = [0xbe2aaaa8, 0x3c0886a0, 0xb94e294d, 0xb477034f, 0x35ea3ca9];
pub(crate) const POLY_COS: [U; 5] = [0xbf000000, 0x3d2aaaab, 0xbab60baa, 0x37d033fe, 0xb499e1e4];
pub(crate) const POLY_TAN: [U; 5] = [0x3eaaaf56, 0x3e07e0db, 0x3d6d3401, 0x3c3750d4, 0x3cae109d];
