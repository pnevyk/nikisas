//! Utilities for testing implementation quality of mathematical functions.
//! Computing errors for inputs randomly sampled from given interval.
//!
//! If you want to learn more about testing approximation errors, read [W. J.
//! Cody: Performance testing of function
//! subroutines](https://dl.acm.org/doi/10.1145/1476793.1476921).
//!
//! # Usage
//!
//! To determine the errors:
//!
//! ```
//! use nikisas_test::prelude::*;
//!
//! fn exp(x: f32) -> f32 {
//!     // your implementation
//!     # 0.0
//! }
//!
//! // Uniformly sample 100000 values from -87.3 to 88.7.
//! UniformSample::with_count(-87.3, 88.7, 100000)
//!     // Use implementation from the standard library as ground truth.
//!     .error(|x| (exp(x), x.exp()))
//!     // Print the errors to standard output.
//!     .print_plain("exp");
//! ```
//!
//! To ensure desired error bounds:
//!
//! ```should_panic
//! use nikisas_test::prelude::*;
//! # fn exp(x: f32) -> f32 {
//! #     // your implementation
//! #     0.0
//! # }
//!
//! // Uniformly sample 100000 values from -87.3 to 88.7.
//! UniformSample::with_count(-87.3, 88.7, 100000)
//!     // Use implementation from the standard library as ground truth.
//!     // If eny specified error bound is violated, the program panics with a readable message.
//!     .assert(ErrorBounds::new().rel(0.001).abs(0.0001), |x| (exp(x), x.exp()));
//! ```
//!
//! # Errors
//!
//! ## Relative error
//!
//! Relative error approximates the worst-case behavior of the function in the
//! interval. It is computed as
//!
//! ```plain
//! rel_err = max_{x_i in I} | (F(x_i) - f(x_i)) / f(x_i) |
//! ```
//!
//! where I is the tested interval, F(x_i) is the value being tested and f(x_i)
//! is the real value of approximated function. The error is scaled by f(x_i) in
//! order to normalize the error relative to the magnitude of the error. If we
//! compute exp(80), the absolute approximation error might be quite large, but
//! compared to the magnitude of the result it might be acceptable. On the other
//! hand, when computing ln(2), an absolute error 0.1 might be too large because
//! the scale is very tiny.
//!
//! ## Absolute error
//!
//! Absolute error represents the real worst-case behavior of the function in
//! the interval. It s computed as
//!
//! ```plain
//! abs_err = max_{x_i in I} | F(x_i) - f(x_i) |
//! ```
//!
//! There are circumstances where the absolute error is more appropriate then
//! the relative. For example if we want to bounds the error even for large
//! values regardless of the magnitude.
//!
//! When both relative and absolute errors are specified in [`ErrorBounds`],
//! then they are checked such that at least *one* of the bounds holds. This is
//! useful when computing errors for very small values, where achieving small
//! enough relative error might be difficult. The use case is when there is a
//! requirement for given relative error, but the error less than certain number
//! of decimal places is also fine.
//!
//! ## Root-mean-square error
//!
//! Root-mean-square error takes all sampled values into account and indicates
//! the overall quality of the implementation. It is computed as
//!
//! ```plain
//! rms_err = sqrt( 1 / N * sum_{x_i in N} ( (F(x_i) - f(x_i)) / f(x_i) )^2 )
//! ```
//!
//! where N is the total number of sampled values. If the root-mean-square error
//! is close to the maximum relative error, it indicates that the implementation
//! is very stable without pathological inputs. If it is significantly lower,
//! that means than there are pathological inputs at which the implementation
//! performs poorly in comparison with others.
//!
//! # Domain
//!
//! The approximations usually reduce the input into a small *primary* range,
//! the reduced argument is then approximated, and the result value is
//! reconstructed from it.
//!
//! The tests should therefore be split at least to two parts: the first one
//! samples inputs from the primary range to exercise the approximation error,
//! and the second one samples inputs from the entire input range to determine
//! the additional error caused by argument reduction.
//!
//! Values from primary range should be sampled uniformly. For the whole range
//! that is usually much bigger, values should be sampled in logarithmic scale,
//! because that more simulates the distribution of numbers encountered in
//! real-world. This is not implemented yet.
//!
//! # TODO
//!
//! * Logarithmic distribution for large intervals.
//! * Confidence estimation for the error bounds.
//! * More comfortable testing for multiple-argument functions.
//!
//! # License
//!
//! nikisas_test is licensed under MIT. Feel free to use it, contribute or
//! spread the word.
//!
//! [`ErrorBounds`]: error/struct.ErrorBounds.html

#![warn(missing_docs)]

pub mod domain;
pub mod error;
pub mod float;
pub mod utils;

pub use domain::{Domain, Exhaustive, UniformSample};
pub use error::{Error, ErrorBounds};

/// Convenience re-export of common members.
pub mod prelude {
    pub use super::{Domain, Error, ErrorBounds, Exhaustive, UniformSample};
}
