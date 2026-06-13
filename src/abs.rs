/// Returns the absolute difference between two numbers, i.e. `|left - right|`.
///
/// Uses [`u64::abs_diff`], which computes the difference without unsigned
/// underflow regardless of operand order.
///
/// ```
/// assert_eq!(chaff_math::abs(10, 3), 7);
/// assert_eq!(chaff_math::abs(3, 10), 7);
/// assert_eq!(chaff_math::abs(5, 5), 0);
/// ```
pub fn abs(left: u64, right: u64) -> u64 {
    left.abs_diff(right)
}

#[cfg(test)]
mod tests;
