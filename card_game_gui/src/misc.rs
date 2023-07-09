use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs::File;

/// Load file and return the content as String.
pub fn loadFile(filename: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    let mut f = File::open(filename);
    let result = match f {
        Ok(mut file) => {
            println!("File {:?} loaded.", filename);
            file.read_to_string(&mut buffer);
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("File {:?} not found.", filename);
            }
            return Err(e);
        }
    };
    Ok(buffer)
}
