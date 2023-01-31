fn main() {
    let file_content = storage::get_file_content("hello.txt");
    println!("{:?}", file_content);

    let file_content_not_existing = storage::get_file_content("bleh.txt");
    println!("{:?}", file_content_not_existing);
}

mod storage {

    use std::io::{Error, Read};
    use std::fs::File;

    pub fn get_file_content(filename: &str) -> Result<String, Error> {
        let mut file_handle = File::open(filename)?;
        let mut file_data = String::new();
        file_handle.read_to_string(&mut file_data)?;
        Ok(file_data)
    }
}
