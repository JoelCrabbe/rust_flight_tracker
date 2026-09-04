// #![allow(unused)]

use crate::{
    aircraft_structures::{AircraftInfo, PositionSource},
    prelude::*,
};
use anyhow::Result;
use axum::{
    Json, Router, debug_handler,
    extract::State,
    routing::{get, post},
};
use serde_json::from_str;
use tower_http::cors::CorsLayer;

mod aircraft_structures;
mod errors;
mod open_sky_network_client;
mod prelude;
mod query_builder;
mod regions_of_interest;
mod token;
mod utils;

#[derive(Deserialize, Debug)]
pub struct MinMaxLatLng {
    #[serde(rename = "minLatitude")]
    pub min_latitude: f64,

    #[serde(rename = "maxLatitude")]
    pub max_latitude: f64,

    #[serde(rename = "minLongitude")]
    pub min_longitude: f64,

    #[serde(rename = "maxLongitude")]
    pub max_longitude: f64,
}

async fn handler(
    State(mut state): State<OpenSkyNetworkClient>,
    Json(coordinates): Json<MinMaxLatLng>,
) -> Json<AircraftData> {
    let data = state.find_aircraft(coordinates).await.unwrap();
    utils::save_data_to_file(&data, "response.json");
    Json(data)
}

async fn test() -> &'static str {
    "hello from rust server"
}

async fn test_handler() -> Json<AircraftData> {
    println!("received request, sending example data");
    let json_string = std::fs::read_to_string("response.json").unwrap();
    let example_data = from_str::<AircraftData>(&json_string).unwrap();
    Json(example_data)
}

#[tokio::main]
async fn main() -> Result<()> {
    let token_manager = match TokenManager::new() {
        Ok(token) => token,
        Err(e) => panic!("{}", e),
    };

    let http_client = reqwest::Client::new();

    let osnc = OpenSkyNetworkClient::new(token_manager, http_client);

    let app = Router::new()
        .route("/coordinates", post(handler))
        // .route("/coordinates", post(test_handler))
        .route("/", get(test))
        .layer(CorsLayer::permissive())
        .with_state(osnc);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/*
Due to api token limitations, updating aircraft's position by constantly requesting the OpenSkyNetwork api will burn through my tokens
very fast.
We could instead draw our area on the map and `monitor` that airspace.
We could query the api periodically e.g. every 15 seconds and see what is in the airspace
we could use some sort of hashset to keep track of what we have seen, this way we can check if we have seen each aircraft before
using an O(1) lookup in the set.
Maybe we could also go down the phone notification route of sending notifications of what is in the airspace.

I tried using twilio and it didn't work and kind of sucks
*/
