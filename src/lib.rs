//! `chaff-math` — small, dependency-free numeric helpers.
//!
//! See [`abs_diff`] for the absolute difference of two values and [`add`] for
//! their sum.
#![no_std]
#![warn(missing_docs)]

mod abs_diff;
mod add;

pub use abs_diff::abs_diff;
pub use add::add;
