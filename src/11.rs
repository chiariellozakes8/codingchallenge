use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}
