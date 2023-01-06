fn main() {
    let mut original = String::from("original value");
    println!("outer scope original: \t\"{}\"", original);

    // scope, just like most C-Style languages
    {
        let next = &mut original;

        // just like C, C++ you can dereference the value to assign 
        *next = String::from("next value");

        println!("inner scope next: \t\"{}\"", next);
        println!("inner scope original: \t\"{}\"", original);
    }

    println!("outer scope original: \t\"{}\"", original);
}
