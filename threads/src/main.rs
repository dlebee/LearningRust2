use std::thread; 

fn main() {
    let outer_scope = 412;

    let join_handle = thread::spawn(move || {
        outer_scope * 2
    });

    let result = join_handle.join();
    //println!("{} {:?}", outer_scope, result);

    match result {
        Ok(result_value) => {
            println!("{}", result_value);
        }
        Err(result_error) => {
            println!("{:?}", result_error);
        }
    }
}
