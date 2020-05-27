use std::io::{BufRead, Read};

use crate::bufwrap::{file_reader, stdin_reader, FileReader, StdinReader};

use clap::{App, Arg};

/// Wraps the input source to an AOC problem. Handles cmdline arguments.
///
/// Supports input from a given file as well as from STDIN.
/// Implements `std::io::BufRead` for the convenience of the `BufRead::lines` iterator.
///
/// # Example
///
/// If your executable is named `prog` and the description is "Example description",
/// then the help (`prog --help`) will look like this:
///
/// ```console
/// Example description
///
/// USAGE:
///     prog [FILE]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <FILE>    Input file (defaults to STDIN if not provided)
/// ```
pub enum BufferedInput {
    File(FileReader),
    Stdin(StdinReader),
}

impl BufferedInput {
    /// Parse cmdline arguments and create a new object
    /// to read lines from an arbitrary input source.
    ///
    /// The application optionally expects a path argument
    /// that specifies the file to read from.
    /// If not present, the input is read from STDIN.
    ///
    /// # Arguments
    ///
    /// * `description` - provides an "about" section in the usage manual.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::BufRead;
    /// use aoc_utils::BufferedInput;
    ///
    /// let input = BufferedInput::parse_args("Example solution").unwrap();
    /// let lines: Vec<String> = input.lines().map(Result::unwrap).collect();
    /// ```
    pub fn parse_args(description: &str) -> std::io::Result<Self> {
        let input_arg = Arg::with_name("input")
            .value_name("FILE")
            .help("Input file (defaults to STDIN if not provided)");
        let app = App::new("").about(description).arg(input_arg);

        let matches = app.get_matches();

        let result = match matches.value_of("input") {
            Some(path) => BufferedInput::File(file_reader(path)?),
            _ => BufferedInput::Stdin(stdin_reader()),
        };

        Ok(result)
    }

    /// Returns an iterator over the lines of the input
    /// that panics when an error occurs.
    pub fn unwrapped_lines(self) -> impl Iterator<Item = String> {
        self.lines().map(Result::unwrap)
    }
}

impl Read for BufferedInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BufferedInput::File(file_reader) => file_reader.read(buf),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.read(buf),
        }
    }
}

impl BufRead for BufferedInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            BufferedInput::File(file_reader) => file_reader.fill_buf(),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            BufferedInput::File(file_reader) => file_reader.consume(amt),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.consume(amt),
        };
    }
}
