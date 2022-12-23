fn main() {
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);

    // odd rust allows the same variable
    // to be re-declared?
    let scope_test = 0;
    println!("{}", scope_test);
}
