fn main() {

    // exponents
    let squared = i32::pow(8, 2);
    let float_integer = f32::powi(6.3, 3);
    let float_float = f32::powf(6.5, 3.14);

    println!("integer: {}", squared);
    println!("tloat to int: {}", float_integer);
    println!("float to float: {}", float_float);

    // operators follow pmdas
    let order_ops = 8+4*2-(12/3+7)+4;
    println!("{}", order_ops);

    // logical operators
    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    let are_not_equal = 1 != 2;
    let is_true = true;
    let is_false = !is_true;

    println!("is_true: {}, is_false: {}",
        is_true, is_false);

    let have_driver_license = false;
    let have_passport = true;
    let have_proof = have_driver_license || have_passport;

    let have_boarding_pass = true;
    let have_id = false;
    let can_board = have_boarding_pass && have_id;

    println!("Boarding Pass: {}, ID: {}", have_boarding_pass, have_id);
    println!("Can board plane: {}", can_board);

    // > < operators
    let _is_larger = 2 > 1;
    let _is_larger_or_equal = 2 >= 2;
    let _is_smaller = 1 < 4;
    let _is_smaller_or_equal = 4 <= 4;
} 