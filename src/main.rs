use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

mod errors;
mod regions_of_interest;
mod requests;
mod token;
mod utils;

use regions_of_interest::{BoundingBox, Circle, Point, Unit};
use requests::{AicraftTuple, AircraftInfo, PositionSource};

#[derive(Deserialize, Debug)]
struct AircraftData {
    states: Vec<AircraftInfo>,
    time: i64,
}


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
    // let _ = requests::find_aircraft(&mut token_manager, &client, &area_of_interest);

    //TODO: Working on parsing data now into json objects
    // we also need to trim strings of whitespace
    let json_string = std::fs::read_to_string("data.json").unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    let states = json_value["states"].as_array().unwrap();
    let mut results: Vec<AicraftTuple> = vec![];
    for entry in states {
        // cloning here is very bad as we could have lots of entries and each one is a struct
        // so it is not cheap to clone  
        let aircraft_tuple = serde_json::from_value::<AicraftTuple>(entry.clone()).unwrap();
        results.push(aircraft_tuple);
    }

    let mut aicraft_infos: Vec<AircraftInfo> = vec![];
    for elem in results {
        aicraft_infos.push(AircraftInfo::from(elem));
    }

    // for aircraft in aicraft_infos.iter() {
    //     println!("{:?}", aircraft);

    // }

    // also parse the time from the response
    let time = json_value["time"].as_i64().unwrap();

    // println!("{:?}", aicraft_infos[0].position_source);

    let aircraft_data = AircraftData { states: aicraft_infos, time };
    println!("{}", aircraft_data.time);

    // at this point the parsing into structs kind of works but is all over the place
    // I don't entirely understand how everything exactly works

    Ok(())
}
