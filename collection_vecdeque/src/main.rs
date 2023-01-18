use std::collections::{VecDeque};

fn main() {
    let mut waiting_line: VecDeque<&str> = VecDeque::new();

    println!("\n\nOriginal line\n------------------------");
    waiting_line.push_back("David Lebee");
    waiting_line.push_back("John Doe");

    for person in waiting_line.iter() {
        println!("{}", person);
    }


    println!("\n\nAfter liner cutter\n------------------------");

    waiting_line.push_front("Mr Asshole"); // line cutter.

    for person in waiting_line.iter() {
        println!("{}", person);
    }
    
    //useful methods.
    println!("\n\nUseful methods\n------------------------");
    println!("Number of elements inside the vector {}", waiting_line.len());

    waiting_line.clear();
    println!("\n\nnew length after clearing: {}", waiting_line.len());

    // add and contains.
    waiting_line.push_back("John Doe");
    waiting_line.push_back("Chuck Norris");


    println!("\n\nDoes it contain John Doe {:?}", waiting_line.contains(&"John Doe"));
    println!("Does it contain Mr Other {}", waiting_line.contains(&"Mr Other"));
}
