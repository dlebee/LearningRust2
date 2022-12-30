fn main() {

    // let word = "Dog";
    // if word == "Duck" {
    //     println!("Quack!");
    // } else if word == "Dog" {
    //     println!("Woof!");
    // } else {
    //     println!("All quiet out here");
    // }

    let available_aircraft = "Boeing";
    let minimum_crew = 7;
    let available_crew = 8;

    if (available_aircraft == "Boeing" 
        || available_aircraft == "Airbus")
        && minimum_crew <= available_crew
    {
        println!("Okay");
    }
    

}
