enum NavigationAids {
    NDB = 0,
    VOR = 1,
    VORDME = 2,
    //FIX {name: String, latitude: f32, longitude: f32}
}

fn main() {
    println!("NDB:    \t{}", NavigationAids::NDB as u8);
    println!("VOR:    \t{}", NavigationAids::VOR as u8);
    println!("VORDME: \t{}", NavigationAids::VORDME as u8);
}
