//! Computation of the error.

use std::fmt;

use crate::float::FloatExt;

/// Bounds for errors to be asserted. By default, all are empty and therefore
/// not checked. By specifying a bound for given error type, you enable checking
/// it.
///
/// # Examples
///
/// ```
/// use nikisas_test::error::ErrorBounds;
/// // Specify bounds for relative and absolute errors.
/// let bounds = ErrorBounds::new().rel(0.001).abs(0.0001);
/// ```
pub struct ErrorBounds<F> {
    rel: Option<F>,
    abs: Option<F>,
    rms: Option<F>,
}

impl<F: FloatExt> ErrorBounds<F> {
    /// Creates empty bounds, that is, none is checked.
    pub fn new() -> Self {
        ErrorBounds {
            rel: None,
            abs: None,
            rms: None,
        }
    }

    /// Specifies the bound for maximum relative error.
    pub fn rel(mut self, bound: F) -> Self {
        self.rel = Some(bound);
        self
    }

    /// Specifies the bound for maximum absolute error.
    pub fn abs(mut self, bound: F) -> Self {
        self.abs = Some(bound);
        self
    }

    /// Specifies the bound for root-mean-square error.
    pub fn rms(mut self, bound: F) -> Self {
        self.rms = Some(bound);
        self
    }

    /// Checks if the relative and absolute errors satisfy specified bounds.
    pub fn check_rel_or_abs(&self, rel_err: F, abs_err: F) -> bool {
        match (self.rel, self.abs) {
            (Some(rel), Some(abs)) => rel_err <= rel || abs_err <= abs,
            (Some(rel), None) => rel_err <= rel,
            (None, Some(abs)) => abs_err <= abs,
            (None, None) => true,
        }
    }

    /// Checks if the absolute error satisfies specified bound. This is mainly
    /// used when the relative error is undefined due to dividing by zero.
    pub fn check_abs(&self, abs_error: F) -> bool {
        match self.abs {
            Some(abs) => abs_error <= abs,
            None => true,
        }
    }

    /// Checks if the root-mean-square error satisfies specified bound.
    pub fn check_rms(&self, rms_error: F) -> bool {
        match self.rms {
            Some(rms) => rms_error <= rms,
            None => true,
        }
    }
}

/// Aggregator structure that compares computed and real values, input by input,
/// computes the corresponding errors and stores them.
///
/// The first generic parameter specifies the type of floating point used. The
/// second one specifies the input argument(s). In most cases, this will be a
/// single floating point number, however, for multiple argument functions this
/// can be a tuple.
pub struct Error<F, In> {
    max_abs: (In, F),
    max_rel: (In, F),
    sum_rel: F,
    total: F,
    bounds: ErrorBounds<F>,
}

impl<F: FloatExt, In: fmt::Debug + Default + Copy> Error<F, In> {
    /// Initializes the structure without any bounds.
    pub fn new() -> Self {
        Error::with_bounds(ErrorBounds::new())
    }

    /// Initializes the structure with given bounds.
    pub fn with_bounds(bounds: ErrorBounds<F>) -> Self {
        Error {
            max_abs: (In::default(), F::zero()),
            max_rel: (In::default(), F::zero()),
            sum_rel: F::zero(),
            total: F::zero(),
            bounds,
        }
    }

    /// Calculates the errors between computed value and real value. If it is
    /// the current maximum, its value is stored along with the argument that
    /// caused it.
    pub fn calculate(&mut self, arg: In, computed: F, real: F) {
        let abs = (computed - real).abs();

        if abs > self.max_abs.1 {
            self.max_abs = (arg, abs);
        }

        if real != F::zero() {
            let rel = abs / real;

            if rel > self.max_rel.1 {
                self.max_rel = (arg, rel);
            }

            self.sum_rel = self.sum_rel + rel * rel;
            self.total = self.total + F::one();

            if !self.bounds.check_rel_or_abs(rel, abs) {
                panic!(
                    "error exceeded at {:?}, relative error = {:?}, absolute error = {:?}",
                    arg, rel, abs
                );
            }
        } else {
            if !self.bounds.check_abs(abs) {
                panic!("error exceeded at {:?}, absolute error = {:?}", arg, abs);
            }
        }
    }

    /// Returns maximum relative error encountered.
    pub fn max_rel(&self) -> F {
        self.max_rel.1
    }

    /// Returns the argument for maximum relative error encountered.
    pub fn max_rel_arg(&self) -> In {
        self.max_rel.0
    }

    /// Returns maximum absolute error encountered.
    pub fn max_abs(&self) -> F {
        self.max_abs.1
    }

    /// Returns the argument for absolute relative error encountered.
    pub fn max_abs_arg(&self) -> In {
        self.max_abs.0
    }

    /// Returns root-mean-square error for all values encountered.
    pub fn rms(&self) -> F {
        (self.sum_rel / self.total).sqrt()
    }

    /// Asserts the bounds for the errors that were encountered.
    pub fn assert(&self) {
        // The errors for individual inputs are asserted in Error::compare.
        let rms = self.rms();
        if !self.bounds.check_rms(rms) {
            panic!("overall quality is {:?} which is not satisfying", rms);
        }
    }

    /// Prints the errors (and arguments) in a plain, human-readable form.
    pub fn print_plain(&self, name: &str) {
        println!(
            "{}:\trelative = {:?} (at {:?}), absolute = {:?} (at {:?}), root-mean-square = {:?}",
            name,
            self.max_rel(),
            self.max_rel_arg(),
            self.max_abs(),
            self.max_abs_arg(),
            self.rms()
        );
    }

    /// Prints the errors (and arguments) as one line in CSV format. Use
    /// [`print_csv_header`] method to print the header for the CSV file.
    ///
    /// [`print_csv_header`]: struct.Error.html#method.print_csv_header
    pub fn print_csv(&self, name: &str) {
        println!(
            "{},{:?},{:?},{:?},{:?},{:?}",
            name,
            self.max_rel(),
            self.max_rel_arg(),
            self.max_abs(),
            self.max_abs_arg(),
            self.rms()
        );
    }

    /// Prints the header for CSV file which contents are given by [`print_csv`]
    /// method.
    ///
    /// [`print_csv`]: struct.Error.html#method.print_csv
    pub fn print_csv_header() {
        println!("function,maximum relative,maximum relative argument,maximum absolute,maximum absolute argument,root-mean-square");
    }
}
