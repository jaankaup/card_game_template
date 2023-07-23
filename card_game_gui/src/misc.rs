use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs::File;

/// Load file and return the content as String.
pub fn loadFile(filename: &str) -> io::Result<String> { // Result<String, io::Error> {
    let mut buffer = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut buffer);
    Ok(buffer)
}
