pub mod geo {
    use spotcheck_core::GeoLocation;
    //use risc0_zkvm::serde::{Deserialize, Serialize};
    //use serde::{Serialize, Deserialize};
    use serde_json;
    use std::f64::consts::PI;


    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let earth_radius_km = 6371.0;

        let d_lat = degrees_to_radians(lat2 - lat1);
        let d_lon = degrees_to_radians(lon2 - lon1);

        let a = (d_lat / 2.0).sin().powi(2) +
            lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        earth_radius_km * c
    }

    pub fn is_within_area(location: &GeoLocation, target_lat: f64, target_lon: f64, radius_km: f64) -> bool {
        let distance = haversine_distance(
            location.latitude, location.longitude,
            target_lat, target_lon
        );

        distance <= radius_km
    }
}