use std::io::{Error, Read};
use std::fs::File;
use std::fs;

pub fn get_file_content(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?;
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}

pub fn file_exists(filename: &str) -> bool {
    fs::metadata(filename).is_ok()
}