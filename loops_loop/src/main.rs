fn main() {
    let mut counter = 0;
    loop {

        counter += 1;
        if counter == 5 {
            continue;
        }

        println!("Looping {}", counter);
        if counter == 10 {
            break;
        }
    }
}
