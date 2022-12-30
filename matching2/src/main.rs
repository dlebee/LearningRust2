fn main() {
    let ndb_freq: u16 = 384;

    // match ndb_freq {
    //     200..=500 => println!("NDB Frequency is valid!"),
    //     _ => println!("Not a valid frequency")
    // }

    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB Frequency is valid!");
        },
        _ => println!("not a valid frequency")
    }
}
