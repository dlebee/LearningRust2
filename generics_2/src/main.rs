use std::ops::{Mul, Add, Sub};
use rand::Rng; 

fn main() {
    let four_times_five = multiply(4, 5);
    println!("4 x 5 = {}", four_times_five);

    let four_times_five2 = multiply2(4, 5);
    println!("4 x 5 = {}", four_times_five2);

    for number in 0..10 {
        let random_result = random_add_or_substract(number, 5);
        println!("random {}", random_result);
    }
}

fn random_add_or_substract<T>(left: T, right: T) -> T 
    where T: Add<Output = T> + Sub<Output = T>
{
    let mut rng = rand::thread_rng();
    if rng.gen() { 
        left + right
    } else {
        left - right
    }
}

fn multiply<T: Mul<Output = T>>(left: T, right: T) -> T {
    left * right
}

fn multiply2<T>(left: T, right: T) -> T 
    where T: Mul<Output = T>
{
    left * right
}