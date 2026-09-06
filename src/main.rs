// #![allow(unused)]

use crate::prelude::*;
use anyhow::Result;
use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

use request_handlers::{coordinates_handler, test_handler};

mod aircraft_structures;
mod open_sky_network_client;
mod prelude;
mod regions_of_interest;
mod request_handlers;
mod token;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let token_manager = match TokenManager::new() {
        Ok(token) => token,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1); // not about this error handling
        }
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
        .expect("problem binding the TcpListener to port 3000");

    axum::serve(listener, app)
        .await
        .expect("problem starting axum server");

    Ok(())
}