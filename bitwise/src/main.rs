fn main() {

    let bitewise_and = 86 & 27;
    println!("bitewise and: {}", bitewise_and);

    let bitwise_or = 86 | 27;
    println!("bitewise or: {}", bitwise_or);

    let bitwise_xor = 86 ^ 27;
    println!("bitwise xor: {}", bitwise_xor);

    let left_shift = 86 << 1;
    println!("left shift: {} - \n{:#034b}\n{:#034b}", left_shift, 86i32, left_shift);

    let left_shift2 = 86 << 2;
    println!("left shift2: {} - \n{:#034b}\n{:#034b}", left_shift2, 86i32, left_shift2);

    // right shift is the same just opposite
    let right_shift = 1 >> 1; // this should become 0?
    println!("right shift: {}\n{:#034b}\n{:#034b}", right_shift, 1, right_shift);

    // right shift is the same just opposite
    let right_shift2 = 2 >> 1; // this should become 1?
    println!("right shift2: {}\n{:#034b}\n{:#034b}", right_shift2, 2, right_shift2);
}
