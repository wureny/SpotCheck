use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,  // 精度，单位为米
    pub timestamp: u64, // UNIX 时间戳
}

pub struct TargetInfo {
    pub target_latitude: f64,
    pub target_longitude: f64,
    pub target_radius_km: f64,
}