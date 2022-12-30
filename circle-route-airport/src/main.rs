fn main() {
    const EARTH_RADIUS_IN_KILOMETERS:f64 = 6371.0;

    // two points.
    let (left_lat, left_lng) = (41.4075_f64, -81.851111_f64);
    let (right_lat, right_lng) = (40.7861_f64, -111.9822_f64);

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

    println!(
        "Distance between left and right on the surface of Earth is {:.1} kilometers",
        distance
    );
}
