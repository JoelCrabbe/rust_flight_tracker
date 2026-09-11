use crate::prelude::*;
use anyhow::Result;
use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use request_handlers::coordinates_handler;

mod aircraft_structures;
mod open_sky_network_client;
mod prelude;
mod regions_of_interest;
mod request_handlers;
mod token;

#[tokio::main]
async fn main() -> Result<()> {
    let token_manager = match TokenManager::new().await {
        Ok(token) => token,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1); // not sure about this error handling
        }
    };

    let http_client = reqwest::Client::new();

    let osnc = OpenSkyNetworkClient::new(token_manager, http_client);

    let app = Router::new()
        .route("/coordinates", post(coordinates_handler))
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(CorsLayer::permissive())
        .with_state(osnc);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("problem binding the TcpListener to port 3000");

    axum::serve(listener, app)
        .await
        .expect("problem starting axum server");

    Ok(())
}