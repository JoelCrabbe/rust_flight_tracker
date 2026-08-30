use reqwest::blocking::Client;
use std::error::Error;

mod aircraft_structures;
mod errors;
mod query_builder;
mod regions_of_interest;
mod open_sky_network_client;
mod token;
mod utils;

use regions_of_interest::BoundingBox;

use crate::{
    aircraft_structures::AircraftData, open_sky_network_client::OpenSkyNetworkClient, token::TokenManager,
};

fn main() -> Result<(), Box<dyn Error>> {
    let token_manager = match TokenManager::new() {
        Ok(token) => token,
        Err(e) => panic!("{}", e),
    };

    let http_client = Client::new();

    let mut osnc = OpenSkyNetworkClient::new(token_manager, http_client);

    let area_of_interest = BoundingBox::new(49.50, 51.50, 6.50, 8.50);

    let data = match osnc.find_aircraft(&area_of_interest) {
        Ok(data) => {
            utils::save_data_to_file(&data, "response.json");
            data
        },
        Err(e) => panic!("{}", e),
    };


    // let contents = std::fs::read_to_string("response.json")?;
    // let data: AircraftData = serde_json::from_str(&contents)?;
    // println!("{:?}", data.states.unwrap()[5].position_source);

    Ok(())
}
