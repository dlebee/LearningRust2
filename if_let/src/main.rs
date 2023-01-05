#[allow(irrefutable_let_patterns)]

fn main() {
    let _animal = "Duck";


    // the video says that, but im convinced its not true because
    // when i run cargo expand i see the else if dog there.

    // if let will only include code in compilation
    // if the condition is irrefutable (not impossible)
    if let _animal = "Duck" {
        println!("Qack");
    } else if let _animal = "Dog" {
        println!("Woof!"); // this else if won't be in compiled code.
    }
}

/* compiled code.
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(irrefutable_let_patterns)]
fn main() {
    let _animal = "Duck";
    if let _animal = "Duck" {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["Qack\n"], &[]));
        };
    } else if let _animal = "Dog" {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["Woof!\n"], &[]));
        };
    }
}
*/