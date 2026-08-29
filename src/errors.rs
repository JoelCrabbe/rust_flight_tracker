pub fn assert_lat(lat: f64) {
    assert!(
        -90.0 <= lat && lat <= 90.0,
        "latitude must be between -90 and 90 degrees. {} does not fit these requirements",
        lat
    );
}

pub fn assert_long(long: f64) {
    assert!(
        -180.0 <= long && long <= 180.0,
        "longitude must be between -180 and 180 degrees. {} does not fit these requirements",
        long
    );
}
