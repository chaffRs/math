/// Returns the sum of two numbers.
///
/// ```
/// assert_eq!(chaff_math::add(2, 2), 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests;
