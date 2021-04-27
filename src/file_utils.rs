use std::fs::File;
use std::io::Read;
use std::io::Result;

pub fn path_to_binary(path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}
