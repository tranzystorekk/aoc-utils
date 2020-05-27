# Advent of Code Rust utilities

[![Crates.io](https://img.shields.io/crates/v/aoc-utils)](https://crates.io/crates/aoc-utils)
[![Docs.rs](https://docs.rs/aoc-utils/badge.svg)](https://docs.rs/aoc-utils)
[![Crates.io](https://img.shields.io/crates/l/aoc-utils)](https://github.com/tranzystorek-io/aoc-utils/blob/master/LICENSE)

## About

This crate provides a very minimal set of utils to get started
with writing [Advent of Code](https://adventofcode.com/) solutions.

## `BufferedInput`

### Description

This is a wrapper over an input source (either a file or STDIN).
It can be conveniently constructed by parsing cmdline arguments.

Here is an example help printout generated from a program that uses `BufferedInput`:

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
use aoc_utils::BufferedInput;

let input = BufferedInput::parse_args("Example solution").unwrap();
let lines: Vec<String> = input.lines().map(Result::unwrap).collect();

for line in lines {
    println!("{}", line);
}
```
