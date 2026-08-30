use reqwest::blocking::Client;

use crate::regions_of_interest::BoundingBox;
use crate::token::TokenManager;

// i want this function to take in some sort of area, not just a box
// circles aswell i.e. some type that implements some sort of trait
pub fn find_aircraft(
    token_manager: &mut TokenManager,
    client: &Client,
    area: &BoundingBox,
) -> Result<(), reqwest::Error> {
    // what about some sort of builder pattern for the url i.e. create it in steps of chained methods?

    // first need to construct the query
    // NOTE: the true base url should be https://opensky-network.org/api/ however
    // at the moment we are not using the flights/, states/, tracks/ etc endpoints
    let mut url = "https://opensky-network.org/api/states/all?".to_string();
    let filter = format!(
        "lamin={}&lomin={}&lamax={}&lomax={}",
        area.min_lat, area.min_long, area.max_lat, area.max_long
    );
    url.push_str(&filter);

    let response = client.get(url).headers(token_manager.header()).send()?;

    //TODO: Need to implement the parsing response into struct's logic in here
    if response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        println!("{}", body);
    } else {
        println!("{}", response.status());
    }

    Ok(())
}
