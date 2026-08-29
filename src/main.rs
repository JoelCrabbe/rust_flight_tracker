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

    // I don't like the fact that i have to pass in the http client and token_manager it doesnt seem right
    // There must be a better way to design this code, maybe have it in an impl block for TokenManager
    // but that also doesn't seem right
    let _ = requests::find_aircraft(&mut token_manager, &client, &area_of_interest);

    //TODO: An issue I have found is, if a token is expired the next time you make a request it wont work
    // the 2nd time it will work again
    // maybe it is trying to send the request before the token has updated
    // and maybe thats why we should use the async version of the client

    Ok(())
}
