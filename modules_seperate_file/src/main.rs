mod storage;

// multi function import.
use storage::{get_file_content, file_exists};

// this enables alias, to solve ambiguity.
use storage::get_file_content as gfc;

fn main() {
    let file_content = gfc("hello.txt");
    println!("{:?}", file_content);

    let bleh_exists = file_exists("bleh.txt");
    println!("{:?}", bleh_exists);

    let bleh_failed_content = get_file_content("bleh.txt");
    println!("{:?}", bleh_failed_content);
}
