struct Waypoint {
    name: String,
    latitude: f32,
    longitude: f32
}

impl Waypoint {
    fn to_display(&self) -> String {
        format!("{} ({},{})", 
            self.name,
            self.latitude,
            self.longitude
        )
    }
}

struct Segment {
    start: Waypoint,
    end: Waypoint
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self {
            start,
            end
        }
    }

    fn distance(&self) -> f32 {

        const EARTH_RADIUS_IN_KILOMETERS:f32 = 6371.0;
    
        // you need latitudes as radians
        let left_lat_rad = self.start.latitude.to_radians();
        let right_lat_rad = self.end.latitude.to_radians();
    
        // deltas
        let delta_lat = (self.start.latitude - self.end.latitude).to_radians();
        let delta_lng = (self.end.longitude - self.end.longitude).to_radians();
    
        let central_angle_inner = 
            (delta_lat / 2.0).sin().powi(2) 
            + left_lat_rad.cos() 
            * right_lat_rad.cos() 
            * (delta_lng / 2.0).sin().powi(2);
    
        let central_angle = 2.0 * central_angle_inner.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
        distance
    }
}


fn main() {

    let yul = Waypoint {
        name: "YUL".to_string(),
        latitude: 45.478056,
        longitude: -73.744167
    };

    let tlv = Waypoint {
        name: "TLV".to_string(),
        latitude: 32.011389,
        longitude: 34.886667
    };

    let yul_tlv = Segment::new(yul, tlv);
    let distance = yul_tlv.distance();
    
    println!("The distance between {} and {} is {:.1}", 
        yul_tlv.start.to_display(),
        yul_tlv.end.to_display(),
        distance);
}
