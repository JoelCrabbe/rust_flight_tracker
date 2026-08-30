// #![allow(unused)]

use crate::prelude::*;
use anyhow::Result;

mod aircraft_structures;
mod errors;
mod open_sky_network_client;
mod query_builder;
mod regions_of_interest;
mod token;
mod utils;
mod prelude;


fn main() -> Result<()> {
    /*
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
    */


    let filename = "response.json";
    let contents = std::fs::read_to_string(filename)
        .with_context(|| format!("problem parsing `{filename}` into a string"))?;

    let data: AircraftData = from_str(&contents)
        .context("problem parsing the json file into AircraftData struct")?;

    println!("{:?}", data.states.unwrap()[0].callsign);

    Ok(())
}
