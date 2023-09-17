# Advent of Code Rust utilities

[![Crates.io](https://img.shields.io/crates/v/aoc-utils)](https://crates.io/crates/aoc-utils)
[![Docs.rs](https://docs.rs/aoc-utils/badge.svg)](https://docs.rs/aoc-utils)
[![Crates.io](https://img.shields.io/crates/l/aoc-utils)](https://github.com/tranzystorek-io/aoc-utils/blob/master/LICENSE)

## About

This crate provides a very minimal set of utils to get started
with writing [Advent of Code](https://adventofcode.com/) solutions.

## `AocCommand`

### Description

This is a CLI command builder that provides an input source (either a file or STDIN).
The suggested workflow is to create a Rust project with one binary per AOC solution.

Here is an example help printout generated from a program generated by `AocCommand`:

```console
Example description

USAGE:
    prog [FILE]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    Input file (defaults to STDIN if not provided)
```

### Usage

Collect all lines from input:

```rust
use std::io::BufRead;
use aoc_utils::AocCommand;

let input = AocCommand::new("Example solution").parse_args().unwrap();
let lines: Vec<String> = input.lines().map(Result::unwrap).collect();

for line in lines {
    println!("{}", line);
}
```
