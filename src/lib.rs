//! The minimal set of utils for writing [Advent of Code](https://adventofcode.com/) solutions.

mod args;
mod bufwrap;

pub use args::BufferedInput;
pub use elapsed::measure_time;

use std::fmt::Display;

/// Runs the code, and prints out the elapsed time and the result.
///
/// First prints the elapsed time to stderr,
/// then the result to stdout, in separate lines.
///
/// # Example
///
/// ```
/// use aoc_utils::measure_and_print;
///
/// measure_and_print(|| {
///     (0..10_000).sum::<u64>()
/// });
///
/// // Prints:
/// // 227.81 μs
/// // 49995000
/// ```
pub fn measure_and_print<T: Display, F: FnOnce() -> T>(f: F) {
    let (elapsed, result) = elapsed::measure_time(f);

    eprintln!("{}", elapsed);
    println!("{}", result);
}
