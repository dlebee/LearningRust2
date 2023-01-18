fn main() {
    let mut flights: Vec<&str> = Vec::new();

    // short handed way to write the creation
    // of a vector (its a macro)
    //let vec_macro = vec![1, 2, 3, 4];

    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA97\tto New York departs at 13:43");

    for flight in flights.iter() {
        println!("{}", flight);
    }

    // less safe, because it can cause a panic
    let third = flights[2];
    println!("\nThe third entry in the vector is {}\n", third);

    // more safe.
    let fourth = flights.get(4); // creates an option similar to nth
    match fourth {
        Some(flight) => {
            println!("\nThe fourt flight is {}", flight);
        },
        _ => {
            println!("There is no fourt flight");
        }
    }

    // could also uset
    if let Some(flight_value) = flights.get(2) {
        println!("\nUsing if let second flight is: {}", flight_value);
    }

    println!("\nAfter inserted");
    flights.insert(2, "DA918\tto Orlando departs at 11:12");
    for flight in flights.iter() {
        println!("{}", flight);
    }


    // remove index.
    println!("\nAfter removed");
    flights.remove(1);
    for flight in flights.iter() {
        println!("{}", flight);
    }
}
