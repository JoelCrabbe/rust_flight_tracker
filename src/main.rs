use reqwest::{Response, blocking::Client};
use std::error::Error;

mod aircraft_structures;
mod errors;
mod regions_of_interest;
mod requests;
mod token;
mod utils;
mod query_builder;

use regions_of_interest::BoundingBox;

use crate::aircraft_structures::AircraftData;


fn main() -> Result<(), Box<dyn Error>> {
    let mut token_manager = utils::load_credentials()?;

    // create client
    let client = Client::new();

    let area_of_interest = BoundingBox::new(49.50, 51.50, 6.50, 8.50);
    
    // I don't like the fact that i have to pass in the http client and token_manager it doesnt seem right
    // There must be a better way to design this code, maybe have it in an impl block for TokenManager
    // but that also doesn't seem right

    // Uncomment this to send a request
    // Keep commented out to save credits
    // let data = requests::find_aircraft(&mut token_manager, &client, &area_of_interest)?;
    // utils::save_data_to_file(&data, "response.json");

    let contents = std::fs::read_to_string("response.json")?;
    let data: AircraftData = serde_json::from_str(&contents)?;
    println!("{:?}", data.states.unwrap()[5].position_source);


    Ok(())
}
