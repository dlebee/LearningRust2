struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64
}

fn main() {

    let yul = Waypoint {
        name: "YUL".to_string(),
        latitude: 45.478056,
        longitude: -73.744167
    };

    // you can copy fields using spread operator similarly to js
    let yul_copy = Waypoint{
        name: "COPY OF YUL".to_string(),
        ..yul
    };

    println!("{} {} {}", yul.name, yul.latitude, yul.longitude);
    println!("{} {} {}", yul_copy.name, yul_copy.latitude, yul_copy.longitude);
}
