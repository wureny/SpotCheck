use risc0_zkvm::guest::env;
use spotcheck_core::TargetInfo;
use spotcheck_core::GeoLocation;
use guest_code_for_SpotCheck::geo;
fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: GeoLocation = env::read::<GeoLocation>();
    // TODO: do something with the input
    let target = TargetInfo{
        target_latitude: 42.0,
        target_longitude: -87.0,
        target_radius_km: 10.0,
    };

    let is_within:bool = geo::is_within_area(&input, target.target_latitude, target.target_longitude, target.target_radius_km);
    // write public output to the journal
    let reF = &is_within;
    env::commit(reF);
}
