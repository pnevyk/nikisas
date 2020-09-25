//! Iterators over specific input interval that compute (or assert) errors over
//! that domain.
//
// Use [`UniformSample`] for random sampling in given interval. Use
// [`Exhaustive`] to iterate over all machine numbers around an extreme of
// interest.
//
// [`UniformSample`]: struct.UniformSample.html
// [`Exhaustive`]: struct.Exhaustive.html

use rand::distributions::{DistIter, Uniform};
use rand::prelude::*;
use rand::rngs::SmallRng;

use crate::error::{Error, ErrorBounds};
use crate::float::FloatExt;

/// Uniformly samples values in given interval. This should be primarily used
/// for determining errors on the domain.
pub struct UniformSample<F: FloatExt> {
    count: usize,
    iter: DistIter<Uniform<F>, SmallRng, F>,
}

impl<F: FloatExt> UniformSample<F> {
    /// Creates new iterator. The number of sampled values is fixed to given
    /// count.
    pub fn with_count(low: F, high: F, count: usize) -> Self {
        assert!(low < high);
        let distr = Uniform::new_inclusive(low, high);
        let rng = SmallRng::seed_from_u64(3);
        let iter = rng.sample_iter(distr);

        UniformSample { count, iter }
    }

    /// Creates new iterator. The number of samples is determined by the total
    /// number of machine numbers within given interval. Be careful with
    /// intervals crossing zeros, there is *a lot* of machine numbers around
    /// zeros, and the number of sampled values might become infeasible in such
    /// case. The `fraction` argument must be a number between zero and one.
    pub fn with_fraction(low: F, high: F, fraction: f32) -> Self {
        assert!(low < high);
        assert!(fraction > 0.0 && fraction <= 1.0);
        let count = (low.floats_between(high) as f64 * fraction as f64).round() as usize;

        UniformSample::with_count(low, high, count)
    }
}

impl<F: FloatExt> Iterator for UniformSample<F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            self.iter.next()
        }
    }
}

/// Iterates over *all* machine numbers in given interval. This might be useful
/// to test values near certain extremas.
pub struct Exhaustive<F: FloatExt> {
    low: F,
    high: F,
}

impl<F: FloatExt> Exhaustive<F> {
    /// Creates new iterator. The range is specified exactly by the user.
    pub fn bounded(low: F, high: F) -> Self {
        assert!(low < high);
        Exhaustive { low, high }
    }

    /// Creates new iterator. The range determined by the middle point and an
    /// epsilon to both sides. This creates an interval symmetric around the
    /// value.
    pub fn near(value: F, eps: F) -> Self {
        assert!(eps > F::zero());
        let low = value - eps;
        let high = value + eps;
        Exhaustive { low, high }
    }
}

impl<F: FloatExt> Iterator for Exhaustive<F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low > self.high {
            None
        } else {
            let current = self.low;
            self.low = self.low.nextup();
            Some(current)
        }
    }
}

/// Trait for interval iterators for computing (or asserting) errors.
pub trait Domain<F: FloatExt> {
    /// Computes the errors encountered on the interval.
    fn error<T>(self, compute: T) -> Error<F, F>
    where
        T: Fn(F) -> (F, F);

    /// Asserts the errors encountered on the interval to have given bounds.
    fn assert<T>(self, bounds: ErrorBounds<F>, compute: T)
    where
        T: Fn(F) -> (F, F);
}

impl<F: FloatExt, I: Iterator<Item = F>> Domain<F> for I {
    fn error<T>(self, compute: T) -> Error<F, F>
    where
        T: Fn(F) -> (F, F),
    {
        let mut error = Error::new();

        for x in self {
            let (computed, real) = compute(x);
            error.calculate(x, computed, real);
        }

        error
    }

    fn assert<T>(self, bounds: ErrorBounds<F>, compute: T)
    where
        T: Fn(F) -> (F, F),
    {
        let mut error = Error::with_bounds(bounds);

        for x in self {
            let (computed, real) = compute(x);
            error.calculate(x, computed, real);
        }

        error.assert();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn uniform_sample() {
        let count = 100000;
        let low = 1.0f32;
        let high = 2.0f32;

        let values = UniformSample::with_count(low, high, count).fold(
            HashSet::with_capacity(count),
            |mut values, x| {
                assert!(x >= low && x <= high);

                // To overcome the fact that floats do not implement Hash nor
                // Eq.
                values.insert(x.to_bits());
                values
            },
        );

        // Sufficient spread?
        let uniqueness = values.len() as f64 / count as f64;
        assert!(uniqueness > 0.99);
    }

    proptest! {
        #[test]
        fn exhaustive(x: f32, k in 1usize..100) {
            if x.is_finite() {
                let eps = (0..k).fold(0.0, |eps, _| eps.nextup());
                let low = x - eps;
                let high = x + eps;
                assert_eq!(
                    Exhaustive::near(x, eps).count(),
                    low.floats_between(high) as usize
                );
            }
        }
    }
}
