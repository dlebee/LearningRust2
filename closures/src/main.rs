// closures are functions without a name
// they are also known as anonymous functions


fn main() {
    let full_name = "David Lebee";
     
    let my_closure = || {
        println!("{}", full_name);
    };

    my_closure();

    let closure_with_param = |some_name| {
        println!("closure with param: {}", some_name);
    };

    closure_with_param(full_name);

    // you can call it without storing it into any variable names
    // by wrapping it using (|| {})();
    (|param_full_name| {
        println!("closure called right away with param {}", param_full_name);
    })(full_name);

    // closure that returns a value
    let format_company_name_and_slogan = |name, slogan| -> String {
        format!("\n\n{}\n\t{}\n\n", name, slogan)
    };

    println!("Formatted company: {}", format_company_name_and_slogan("Powered Softwares Inc.", "Your Digital Transformation Partner"))
}
