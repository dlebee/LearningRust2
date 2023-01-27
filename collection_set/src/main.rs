use std::collections::HashSet;

fn main() {
    let mut flights = HashSet::new();
    
    flights.insert(("DA918", "11:12", "Orlando"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA98", "09:43", "London"));
    flights.insert(("DA113", "06:20", "Boston"));
    flights.insert(("DA41", "15:30", "Berlin"));
    flights.insert(("DA2815", "17:11", "Nashville"));

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
