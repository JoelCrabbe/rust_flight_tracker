#[derive(Debug)]
pub struct BoundingBox {
    pub min_latitude: f64,
    pub max_latitude: f64,
    pub min_longitude: f64,
    pub max_longitude: f64,
}

impl BoundingBox {
    pub fn new(
        min_latitude: f64,
        max_latitude: f64,
        min_longitude: f64,
        max_longitude: f64,
    ) -> Self {
        Self {
            min_latitude,
            max_latitude,
            min_longitude,
            max_longitude,
        }
    }
}
