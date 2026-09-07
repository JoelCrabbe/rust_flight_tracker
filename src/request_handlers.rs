use axum::{Json, extract::State};

use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct MinMaxLatLong {
    #[serde(rename = "minLatitude")]
    pub min_latitude: f64,

    #[serde(rename = "maxLatitude")]
    pub max_latitude: f64,

    #[serde(rename = "minLongitude")]
    pub min_longitude: f64,

    #[serde(rename = "maxLongitude")]
    pub max_longitude: f64,
}

pub async fn coordinates_handler(
    State(mut state): State<OpenSkyNetworkClient>,
    Json(payload): Json<MinMaxLatLong>,
) -> Json<AircraftData> {
    println!("request for data come in");
    match state.find_aircraft(payload).await {
        Ok(data) => Json(data),
        Err(e) => {
            eprintln!("{e}");
            Json(AircraftData::default())
        }
    }
}

pub async fn test_handler() -> Json<AircraftData> {
    println!("received request, sending example data");
    let json_string = std::fs::read_to_string("response.json").unwrap();
    let example_data = from_str::<AircraftData>(&json_string).unwrap();
    Json(example_data)
}