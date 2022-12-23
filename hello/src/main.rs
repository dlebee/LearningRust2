#[allow(unused_variables)]

fn main() {
    let location = ("KCLRE", 41.4904069, -81.8546911);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, latitude: {}, longitude: {}",
        name, latitude, longitude);
}