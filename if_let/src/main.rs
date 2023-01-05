#[allow(irrefutable_let_patterns)]

fn main() {
    let mut _animal = "Duck";

    // let _animal will assigned the right value but also check if its true
    // meaning won't be part of comopilation message isn't part of assembly code
    // because rust can determine it not irrefutable?
    if let _animal = "Duck" {
        println!("Quack");
    } else {
        println!("won't be part of compilation");
    }
}

/*
using cargo rustc --release -- --emit asm you can see that the Woof is never emitted to asm.
*/
   