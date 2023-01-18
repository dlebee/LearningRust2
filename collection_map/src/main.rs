use std::collections::HashMap;

fn main() {
    let mut flights = HashMap::new();

    flights.insert("DA918", ("11:12", "Orlando"));
    flights.insert("DA428", ("12:05", "Salt Lake City"));
    flights.insert("DA98", ("09:43", "London"));
    flights.insert("DA113", ("06:20", "Boston"));
    flights.insert("DA41", ("15:30", "Berlin"));
    flights.insert("DA2815", ("17:11", "Nashville"));

    let flight_number = "DA2815";
    let option = flights.get(flight_number);
    let (time, destination) = option.unwrap();
    println!("{} {}", time, destination);

    let puerto_rico_flight = "DA404";
    if !flights.contains_key(puerto_rico_flight) {
        flights.insert(puerto_rico_flight, ("12:00", "Puerto Rico"));
    } else {
        println!("Flight {} is already entered", puerto_rico_flight);
    }

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
