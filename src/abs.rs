/// Returns the absolute difference between two numbers, i.e. `|left - right|`.
///
/// The branch avoids unsigned underflow that a naive `left - right` would cause.
///
/// ```
/// assert_eq!(chaff_math::abs(10, 3), 7);
/// assert_eq!(chaff_math::abs(3, 10), 7);
/// assert_eq!(chaff_math::abs(5, 5), 0);
/// ```
pub fn abs(left: u64, right: u64) -> u64 {
    if left > right {
        left - right
    } else {
        right - left
    }
}

#[cfg(test)]
mod tests;
