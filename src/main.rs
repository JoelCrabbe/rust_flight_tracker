use std::error::Error;

mod utils;
mod token;

use token::TokenManager;


fn main() -> Result<(), Box<dyn Error>> {
    let credentials = utils::load_credentials()?;

    let mut token_manager = TokenManager::new(credentials.token, credentials.time_token_was_made);
    let _ = token_manager.update(&credentials.client_id, &credentials.client_secret);
    println!("{}", token_manager.token);
    println!("{}", token_manager.time_token_was_made);


    Ok(())
}
