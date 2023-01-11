use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let filename = "customer.json";
    
    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(filename) {
                        Ok(_file) => {
                            println!("file created");
                        }
                        Err(error) => {
                            println!("{:#?}", error);
                        }
                    }
                },
                _ => {
                    println!("unknown error kind {:#?}", error);
                }
            }
        }
    }    
}
