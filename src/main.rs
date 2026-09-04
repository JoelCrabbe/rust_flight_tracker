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
pub struct Coordinates {
    pub latitudes: Vec<f64>,
    pub longitudes: Vec<f64>,
}

async fn handler(
    State(mut state): State<OpenSkyNetworkClient>,
    Json(coordinates): Json<Coordinates>,
) -> Json<AircraftData> {
    let data = state.find_aircraft(coordinates).await.unwrap();
    Json(data)
}

async fn test() -> &'static str {
    "hello from rust server"
}

async fn test_handler() -> Json<AircraftData> {
    println!("received request, sending example data");
    Json(AircraftData {
        states: Some(vec![AircraftInfo {
            icao24: "44029f".to_string(),
            callsign: Some("AUA20C  ".to_string()),
            origin_country: "Austria".to_string(),
            time_position: Some(1788117821),
            last_contact: 1788117821,
            longitude: Some(7.4847),
            latitude: Some(50.9601),
            baro_altitude: Some(3215.64),
            on_ground: false,
            velocity: Some(167.15),
            true_track: Some(125.35),
            vertical_rate: Some(4.55),
            sensors: None,
            geo_altitude: Some(3360.42),
            squawk: Some("7657".to_string()),
            spi: false,
            position_source: PositionSource::Adsb,
        }]),
        time: 123456,
    })
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
        // .route("/coordinates", post(handler))
        .route("/coordinates", post(test_handler))
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
