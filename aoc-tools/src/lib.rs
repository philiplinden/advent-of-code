//! Common tools for advent of code problems

use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::Path;

/// Given a path to an input file, load the file and return the contents as a
/// string. The file is expected to be plain text.
pub fn load_input_file(input: &str) -> Result<String, Error> {
    // input is a path to a file
    if !Path::new(input).exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("input file does not exist: {}", input),
        ));
    }

    let file = match File::open(input) {
        Ok(file) => file,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("failed to open input file: {}", e),
            ));
        }
    };

    // read the file into a string
    let mut contents = String::new();
    let mut reader = BufReader::new(file);
    match reader.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(Error::new(
            ErrorKind::InvalidData,
            format!("failed to read input file: {}", e),
        )),
    }
}
