use std::collections::HashMap;

#[derive(Debug)]
enum Gender {
    Male,
    Female
}

#[derive(Debug)]
struct Person {
    age: i32,
    first_name: String,
    last_name: String,
    gender: Gender
}

fn main() {

    let mut people: HashMap<i32, Person> = HashMap::new();

    people.insert( 1,Person {
        last_name: String::from("Lebee"),
        first_name: String::from("Eliana"),
        age: 34,
        gender: Gender::Female
    });

    people.insert(2, Person {
        last_name: String::from("Lebee"),
        first_name: String::from("David"),
        age: 34,
        gender: Gender::Male
    });

    for identifier in 1..=3 {
        let person_number_2 = people.get(&identifier);
        match person_number_2 {
            Some(person) => {
                println!("Found a person with the identiifier {}: {:?}", identifier, person);
            },
            None => {
                eprintln!("Could not find a person with the identifier provided, identifier is {}", identifier);
            }
        }
    }

}
