use crate::errors::{assert_lat, assert_long};

#[derive(Debug)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_long: f64,
    pub max_long: f64,
}

// I also want the idea of a point and a circle of some radius around that point and capture everything in that area
// alongside the idea of the square box

impl BoundingBox {
    pub fn new(min_lat: f64, max_lat: f64, min_long: f64, max_long: f64) -> Self {
        assert_lat(min_lat);
        assert_lat(max_lat);
        assert_long(min_long);
        assert_long(max_long);
        Self {
            min_lat,
            max_lat,
            min_long,
            max_long,
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub lat: f64,
    pub long: f64,
}

impl Point {
    pub fn new(lat: f64, long: f64) -> Self {
        assert_lat(lat);
        assert_long(long);
        Self { lat, long }
    }
}

#[derive(Debug)]
pub enum Unit {
    m,
    km,
    mi,
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
    pub unit: Unit,
}

impl Circle {
    pub fn new(center: Point, radius: f64, unit: Unit) -> Self {
        assert_lat(center.lat);
        assert_long(center.long);
        assert!(radius > 0.0);
        Self {
            center,
            radius,
            unit,
        }
    }
}
