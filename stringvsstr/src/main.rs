#[allow(unused_variables)]

fn main() {

    // how to declare string and use references.
    let person_name_slice = "Donald Mallard";
    let person_name_string = person_name_slice.to_string();
    let person_name_string_ref = &person_name_string;
    let person_name_as_str =  person_name_string.as_str();

    // string concatenation 

    // using array and concat
    let duck = "Duck";
    let airlines = "Airlines"; 
    let airline_name = [duck, " ", airlines].concat();
    println!("{}", airline_name);

    // using format 
    let airline_name2 = format!("{} {}", duck, airlines);
    println!("{}", airline_name2);

    // using mutable string (String)
    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "everytime";
    println!("{}", slogan);
}
