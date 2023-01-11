fn main() {
    // you can enable rust backtrace to get a stack trace during panics
    //$env:RUST_BACKTRACE=1
    //panic_vector();
    manual_panic();
    
}

fn panic_vector() {
    // unrecoverable error -- panic.
    let vector = vec![1, 2, 3, 4, 5];
    println!("{}", vector[10]);
}

fn manual_panic() {
    panic!("Sorry, I panicked");
}
