pub fn assert_lat(lat: f64) {
    assert!(
        (-90.0..=90.0).contains(&lat),
        "latitude must be between -90 and 90 degrees. {lat} does not fit these requirements",
    );
}

pub fn assert_long(long: f64) {
    assert!(
        (-180.0..=180.0).contains(&long),
        "longitude must be between -180 and 180 degrees. {long} does not fit these requirements",
    );
}
