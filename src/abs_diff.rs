use core::ops::Sub;

/// Returns the absolute difference between two values, i.e. `|left - right|`.
///
/// Works for any subtractable, comparable type — unsigned/signed integers,
/// floats, or decimal types. The operands are ordered first, so the subtraction
/// never underflows even for unsigned integers.
///
/// # Floating point
///
/// For `f32`/`f64` the result is a single, correctly-rounded subtraction (it
/// neither adds nor removes precision). If either operand is `NaN`, the result
/// is `NaN`.
///
/// # Overflow
///
/// The result has the same type as the inputs. For signed integers whose true
/// difference exceeds the type's range (e.g. `abs_diff(i32::MIN, i32::MAX)`) the
/// subtraction overflows — panicking in debug builds and wrapping in release.
/// The standard library's `iN::abs_diff` sidesteps this by returning the
/// unsigned counterpart; this function keeps input and output the same type.
///
/// # Ordering
///
/// Intended for numeric, totally-ordered types. Only [`PartialOrd`] is required
/// (so that floats are accepted); if two operands are mutually incomparable —
/// the only numeric case being `NaN` — the `right - left` branch is taken.
///
/// ```
/// assert_eq!(chaff_math::abs_diff(10u64, 3), 7);
/// assert_eq!(chaff_math::abs_diff(3u64, 10), 7);
/// assert_eq!(chaff_math::abs_diff(1.5_f64, 0.5), 1.0);
/// ```
pub fn abs_diff<T: Sub<Output = T> + PartialOrd>(left: T, right: T) -> T {
    if left > right {
        left - right
    } else {
        right - left
    }
}

#[cfg(test)]
mod tests;
