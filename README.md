# chaff-math

Small, dependency-free numeric helpers.

## Usage

```rust
use chaff_math::{abs_diff, add};

assert_eq!(add(2, 3), 5);

// abs_diff is the absolute difference, |a - b|, generic over numeric types:
assert_eq!(abs_diff(10u64, 3), 7);   // ordered first, so unsigned never underflows
assert_eq!(abs_diff(-5_i32, 3), 8);
assert_eq!(abs_diff(1.5_f64, 0.5), 1.0);
```

`abs_diff` works for any `T: Sub<Output = T> + PartialOrd` (integers, floats,
decimal types). It returns the same type as its inputs — see the API docs for
the overflow and `NaN` semantics this implies.

## License

[MIT](LICENSE)
