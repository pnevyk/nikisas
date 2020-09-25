# nikisas

An implementation of common mathematical functions with focus on speed and
simplicity of implementation at the cost of precision, with support for `no_std`
environments.

The implementations contain explanations of the algorithms and
[Sollya](http://sollya.gforge.inria.fr/) programs for finding the coefficients
of polynomials reside in [`sollya`](sollya) directory.

If you want a reasonable implementation of mathematical functions with small
memory footprint and performance cost, you should use
[micromath](https://crates.io/crates/micromath) crate.

## Usage

```rust
use nikisas::{ln, consts::E};
assert_eq!(ln(E), 1.0);
```

## Documentation

See [documentation](https://docs.rs/nikisas_test) on crates.io.

## License

nikisas is licensed under [MIT](LICENSE). Feel free to use it, contribute or spread the word.
