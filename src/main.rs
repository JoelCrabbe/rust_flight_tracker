use reqwest::blocking::Client;
use std::error::Error;

mod errors;
mod regions_of_interest;
mod requests;
mod token;
mod utils;

use regions_of_interest::{BoundingBox, Circle, Point, Unit};
use token::TokenManager;

fn main() -> Result<(), Box<dyn Error>> {
    let credentials = utils::load_credentials()?;

    let mut token_manager = TokenManager::new(
        credentials.client_id,
        credentials.client_secret,
        credentials.token,
        credentials.time_token_was_made,
    );

    // create client
    let client = Client::new();

    let area_of_interest = BoundingBox::new(-0.91, 2.14, -1.39, 1.88);

    let _ = requests::find_aircraft(&mut token_manager, &client, &area_of_interest);

    Ok(())
}
