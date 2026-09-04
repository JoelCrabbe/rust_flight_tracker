use axum::{Json, extract::State};

use crate::prelude::*;
use crate::utils;

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
    Json(coordinates): Json<MinMaxLatLong>,
) -> Json<AircraftData> {
    let data = state.find_aircraft(coordinates).await.unwrap();
    utils::save_data_to_file(&data, "response.json");
    Json(data)
}

pub async fn test_handler() -> Json<AircraftData> {
    println!("received request, sending example data");
    let json_string = std::fs::read_to_string("response.json").unwrap();
    let example_data = from_str::<AircraftData>(&json_string).unwrap();
    Json(example_data)
}
