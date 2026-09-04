// #![allow(unused)]

use crate::prelude::*;
use anyhow::Result;
use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

use request_handlers::{coordinates_handler, test_handler};

mod aircraft_structures;
mod errors;
mod open_sky_network_client;
mod prelude;
mod query_builder;
mod regions_of_interest;
mod request_handlers;
mod token;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let token_manager = match TokenManager::new() {
        Ok(token) => token,
        Err(e) => panic!("{}", e),
    };

    let http_client = reqwest::Client::new();

    let osnc = OpenSkyNetworkClient::new(token_manager, http_client);

    let app = Router::new()
        .route("/coordinates", post(coordinates_handler))
        .route("/test", post(test_handler))
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
