use crate::float::F;
use nikisas_test::ErrorBounds;

/// Corresponds to 0.1% error.
pub(crate) const REL_ERROR: F = 0.001;

/// Corresponds to precision up to 4 decimal points.
pub(crate) const ABS_ERROR: F = 0.00005;

pub(crate) fn error_bounds() -> ErrorBounds<f32> {
    ErrorBounds::new().rel(REL_ERROR).abs(ABS_ERROR)
}
