# nikisas_test

Utilities for testing implementation quality of mathematical functions.
Computing errors for inputs randomly sampled from given interval.

# Usage

To determine the errors:

```rust
use nikisas_test::prelude::*;

fn exp(x: f32) -> f32 {
    // your implementation
    # 0.0
}

// Uniformly sample 100000 values from -87.3 to 88.7.
UniformSample::with_count(-87.3, 88.7, 100000)
    // Use implementation from the standard library as ground truth.
    .error(|x| (exp(x), x.exp()))
    // Print the errors to standard output.
    .print_plain("exp");
```

To ensure desired error bounds:

```rust
use nikisas_test::prelude::*;

// Uniformly sample 100000 values from -87.3 to 88.7.
UniformSample::with_count(-87.3, 88.7, 100000)
    // Use implementation from the standard library as ground truth.
    // If eny specified error bound is violated, the program panics with a readable message.
    .assert(ErrorBounds::new().rel(0.001).abs(0.0001), |x| (exp(x), x.exp()));
```

## Documentation

See [documentation](https://docs.rs/nikisas_test) on crates.io.

## License

nikisas_test is licensed under [MIT](LICENSE). Feel free to use it, contribute or spread the word.
