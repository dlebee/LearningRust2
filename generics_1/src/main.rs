#[derive(Debug)]
struct NavAid<T, T2>{
    name: String,
    frequency: T,
    data: T2
}

fn main() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is VOR")
    };

    let nbd_data: Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: nbd_data 
    };

    println!("VOR frequency is {:?}", vor);
    println!("NDB frequency is {:?}", ndb);
}