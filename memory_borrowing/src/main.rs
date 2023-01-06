fn main() {
   ok();
   still_okay();
   still_okay2();
   not_okay();
}

fn ok() {
    let original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    // borrowing, since next is immutable and is only a pointer to original's
    // memory, the ownership isn't transfering.
    let _next = &original;
    println!("original: \t\"{}\"", original);
}

fn still_okay() {
    // even though we made original immutable, the compiler
    // still see's that we aren't changing the value of original
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    // borrowing, since next is immutable and is only a pointer to original's
    // memory, the ownership isn't transfering.
    let _next = &original;
    println!("original: \t\"{}\"", original);
}

fn still_okay2() {
    // even though we made original immutable, the compiler
    // still see's that we aren't changing the value of original
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    // since we changed the value before we declare next
    // its okay.
    original = String::from("coolio");

    // borrowing, since next is immutable and is only a pointer to original's
    // memory, the ownership isn't transfering.
    let _next = &original;
    println!("original: \t\"{}\"", original);
}

fn not_okay() {
    // even though we made original immutable, the compiler
    // still see's that we aren't changing the value of original
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    // now its no longer okay because, because when we switched original to a new memory
    // location, rust says oh no one owns this memory so i can free it, but because we are using
    // next which is a borrowed value of a pointer to heap that is technically
    // freed during re-assignment of original, rust prevents us from doing this
    // to avoid running into memory management mistake
    let next = &original;
    original = String::from("coolio");
    println!("original: \t\"{}\"", next);
}