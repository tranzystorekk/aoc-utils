use std::path::PathBuf;

use crate::bufwrap::{file_reader, stdin_reader, BufferedInput};

use clap::{value_parser, Arg, Command};

/// CLI command that provides an input source to an AOC problem.
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
pub struct AocCommand {
    description: String,
}

impl AocCommand {
    /// Returns this struct with the description configured.
    ///
    /// # Arguments
    ///
    /// * `description` - provides an "about" section in the usage manual.
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
        }
    }

    /// Parse cmdline arguments and create a new object
    /// to read lines from an arbitrary input source.
    ///
    /// The application optionally expects a path argument
    /// that specifies the file to read from.
    /// If not present, the input is read from STDIN.
    ///
    /// # Example
    ///
    /// ```
    /// use std::io::BufRead;
    /// use aoc_utils::AocCommand;
    ///
    /// let input = AocCommand::new("Example solution").parse_args().unwrap();
    /// let lines: Vec<String> = input.lines().map(Result::unwrap).collect();
    /// ```
    pub fn parse_args(&self) -> std::io::Result<BufferedInput> {
        let input_arg = Arg::new("input")
            .value_parser(value_parser!(PathBuf))
            .value_name("FILE")
            .help("Input file (defaults to STDIN if not provided)");
        let app = Command::new("").about(&self.description).arg(input_arg);

        let matches = app.get_matches();

        let result = match matches.get_one::<PathBuf>("input") {
            Some(path) => BufferedInput::File(file_reader(path)?),
            _ => BufferedInput::Stdin(stdin_reader()),
        };

        Ok(result)
    }
}
