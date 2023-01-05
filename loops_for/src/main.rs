fn main() {

    // by default ranges are non inclusive which means this will loop from 1 to 10
    for index in 1..11 {
        println!("default: {}", index);
    }

    // you can write it like so to say inclusive right value
    for index in 1..=10 {
        println!("inclusive: {}", index);
    }

    // iterating using a Iteratable trait
    // you can turn an array into an iteratable using .iter()
    let family_name = "Lebee";
    let family_first_names = ["David", "Yonatan", "Ariella", "Eliana"];
    for first_name in family_first_names.iter() {
        println!("Family member: {} {}", first_name, family_name);
    }
}
