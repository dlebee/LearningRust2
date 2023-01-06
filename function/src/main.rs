fn main() {
    println!("{}", return_greater(1, 5));
}

fn return_greater(left: i32, right: i32) -> i32 {
    if left > right {
        left
    } else {
        right
    }
}
 