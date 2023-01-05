fn calculate_distance(from: (f64, f64), to: (f64, f64)) -> f64 { 

    const EARTH_RADIUS_IN_KILOMETERS:f64 = 6371.0;

    // two points.
    let (left_lat, left_lng) = from;
    let (right_lat, right_lng) = to;

    // you need latitudes as radians
    let left_lat_rad = left_lat.to_radians();
    let right_lat_rad = right_lat.to_radians();

    // deltas
    let delta_lat = (left_lat - right_lat).to_radians();
    let delta_lng = (left_lng - right_lng).to_radians();


    let central_angle_inner = 
        (delta_lat / 2.0).sin().powi(2) 
        + left_lat_rad.cos() 
        * right_lat_rad.cos() 
        * (delta_lng / 2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();
    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    return distance;
}

fn main() {


    let route = [
        ("YUL", 45.478056, -73.744167),
        ("FLL", 26.0725, -80.152778),
        ("CDG", 49.012779, 2.55),
        ("TLV", 32.011389, 34.886667),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;
    let mut route_path = String::from("");

    for waitpoint in route.iter() {

        match previous_waypoint {
            None => {
                route_path = format!("{}", waitpoint.0);
                previous_waypoint = Option::from(waitpoint.clone());
                continue;
            },
            Some(previous_waypoint_value) => {
                let distance = calculate_distance(
                    (previous_waypoint_value.1, previous_waypoint_value.2),
                    (waitpoint.1, waitpoint.2)
                );

                route_path = format!("{} -> ({:.2}KM) -> {}", 
                    route_path, distance, waitpoint.0);

                total_distance += distance;
                previous_waypoint = Option::from(previous_waypoint_value.clone());
            }
        }
    }

    println!("Total distance of route {} is {:.2}KM", route_path, total_distance);
}
