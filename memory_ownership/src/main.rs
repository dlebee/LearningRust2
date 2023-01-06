fn main() {
    let original = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    // this won't build
    // leaving this cargo not succesful build on purpose
    // rust only allows one owner of memory
    // by putting original (pointer to heap of a string) into next
    // the ownership is now on next, so original is considered unassigned
    // that why the prinln!() can no longer print original.
    let next = original;
    println!("{}", original);
}
