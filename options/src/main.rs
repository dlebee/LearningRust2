fn main() {
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(5);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value);
}
