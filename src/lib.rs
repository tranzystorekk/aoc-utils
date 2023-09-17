//! The minimal set of utils for writing [Advent of Code](https://adventofcode.com/) solutions.

mod args;
mod bufwrap;

pub use args::BufferedInput;

use std::fmt::Display;
use std::time::Instant;

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
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    eprintln!("{:?}", elapsed);
    println!("{}", result);
}
