use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Stdin};
use std::path::Path;

pub type FileReader = BufReader<File>;
pub type StdinReader = BufReader<Stdin>;

/// Buffered input that can be either STDIN or a file.
pub enum BufferedInput {
    File(FileReader),
    Stdin(StdinReader),
}

pub fn file_reader<P: AsRef<Path>>(path: P) -> std::io::Result<FileReader> {
    let file = File::open(path)?;
    let meta = file.metadata()?;

    if meta.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Is a directory"));
    }

    let returned_reader = BufReader::new(file);

    Ok(returned_reader)
}

pub fn stdin_reader() -> StdinReader {
    BufReader::new(std::io::stdin())
}

impl BufferedInput {
    /// Returns an iterator over the lines of the input
    /// that panics when an error occurs.
    pub fn unwrapped_lines(self) -> impl Iterator<Item = String> {
        self.lines().map(Result::unwrap)
    }
}

impl Read for BufferedInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::File(file_reader) => file_reader.read(buf),
            Self::Stdin(stdin_reader) => stdin_reader.read(buf),
        }
    }
}

impl BufRead for BufferedInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            Self::File(file_reader) => file_reader.fill_buf(),
            Self::Stdin(stdin_reader) => stdin_reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Self::File(file_reader) => file_reader.consume(amt),
            Self::Stdin(stdin_reader) => stdin_reader.consume(amt),
        };
    }
}
