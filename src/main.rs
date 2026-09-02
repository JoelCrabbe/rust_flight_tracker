// #![allow(unused)]

use std::sync::{Arc, Mutex};

use crate::prelude::*;
use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;
use anyhow::Result;

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

#[tokio::main]
async fn main() -> Result<()> {

    let token_manager = match TokenManager::new() {
        Ok(token) => token,
        Err(e) => panic!("{}", e),
    };

    let http_client = reqwest::Client::new();

    let mut osnc = Arc::new(Mutex::new(
        OpenSkyNetworkClient::new(token_manager, http_client)));

    let app = Router::new()
        // .route("/coordinates", post(osnc.find_aircraft)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();




    // let area_of_interest = BoundingBox::new(49.50, 51.50, 6.50, 8.50);

    // let data = match osnc.find_aircraft(&area_of_interest) {    
    //     Ok(data) => {
    //         utils::save_data_to_file(&data, "response.json");
    //         data
    //     },
    //     Err(e) => panic!("{}", e),
    // };

    // let filename = "response.json";
    // let contents = std::fs::read_to_string(filename)
    //     .with_context(|| format!("problem parsing `{filename}` into a string"))?;

    // let data = from_str::<AircraftData>(&contents)
    //     .context("problem parsing the json file into AircraftData struct")?;

    // println!("{:?}", data.states.unwrap()[0].callsign);

    Ok(())
}
    

/*

use python3 -m http.server 8080 to host index.hmtl
so index.html is running on localhost:8080
server is running on localhost:3000

*/