//! An implementation of common mathematical functions with focus on speed and
//! simplicity of implementation at the cost of precision, with support for
//! `no_std` environments.
//!
//! The implementations contain explanations of the algorithms and
//! [Sollya](http://sollya.gforge.inria.fr/) programs for finding the
//! coefficients of polynomials reside in [`sollya`](sollya) directory.
//!
//! If you want a reasonable implementation of mathematical functions with small
//! memory footprint and performance cost, you should use
//! [micromath](https://crates.io/crates/micromath) crate.
//!
//!
//! # Usage
//!
//! ```
//! use nikisas::{ln, consts::E};
//! assert_eq!(ln(E), 1.0);
//! ```
//!
//! # What's included
//!
//! Not much. This is (at least for now) for educational purposes. Here is the
//! list:
//!
//! * exponentiation - `exp(x)`, `pow(x, p)`, `pow2(p)`, `pow10(p)`
//! * logarithms - `ln(x)`, `log2(x)`, `log10(x)`
//! * trigonometric functions - `sin(x)`, `cos(x)`, `tan(x)`, `cot(x)`
//!
//! Note that implementation of trigonometric functions give poor results for
//! some inputs (and therefore they fail our current tests).
//!
//! # Errors
//!
//! The implementations are thoroughly tested and the error is bound to be 0.1%
//! *or* 4 decimal places. The testing involves random sampling from valid
//! interval. The ground truth for error computation are the implementations of
//! the corresponding functions in the Rust's standard library.
//!
//! The table of real errors is here:
//!
//! | function | maximum relative | root mean square (overall quality) |
//! | -------- | ---------------- | ---------------------------------- |
//! | cos      | N/A              | N/A                                |
//! | cot      | N/A              | N/A                                |
//! | exp      | 4.15e-6          | 1.39e-6                            |
//! | ln       | 9.60e-8          | 4.05e-8                            |
//! | log2     | 1.29e-7          | 4.08e-8                            |
//! | log10    | 2.02e-7          | 6.24e-8                            |
//! | pow2     | 1.19e-7          | 3.53e-8                            |
//! | pow10    | 4.47e-6          | 1.49e-6                            |
//! | sin      | N/A              | N/A                                |
//! | tan      | N/A              | N/A                                |
//!
//! # Name
//!
//! So this is the story. If we read "libm" (widely-used abbreviation for
//! mathematical library) as "lib em" and do not make the pause in the middle,
//! we get the word "libem". While employing little imagination, we could hear
//! "líbem" /'liːbɛm/, a colloquial form of czech word
//! "[líbáme](https://en.wiktionary.org/wiki/l%C3%ADbat)" /'liːbaːmɛ/, meaning
//! "we kiss". Now *wekiss* is not that cool name for a library, so it needs to
//! go through [english-esperanto
//! translation](https://translate.google.com/#view=home&op=translate&sl=en&tl=eo&text=we%20kiss)
//! first and here we are: *nikisas*. Naming is hard, but at least you can
//! experience love while using this small piece of software.
//!
//! # License
//!
//! nikisas is licensed under MIT. Feel free to use it, contribute or spread the
//! word.

#![no_std]
#![warn(missing_docs)]

pub mod consts;
mod float;
mod math;
#[cfg(test)]
mod test;
mod utils;

pub use math::*;
