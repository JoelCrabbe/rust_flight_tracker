use std::{error::Error, io};

use crate::{aircraft_structures::AircraftData, token::TokenManager};

// Project wide I should handle errors better
// e.g. if some environment variable isnt set, I should try to create it
pub fn load_credentials() -> Result<TokenManager, Box<dyn Error>> {
    dotenvy::dotenv_override()?;
    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;
    let token = dotenvy::var("TOKEN")?;
    let time_token_was_made = dotenvy::var("TIME_TOKEN_WAS_MADE")?.parse::<f64>()?;

    Ok(TokenManager {
        client_id,
        client_secret,
        token,
        time_token_was_made,
    })
}

pub fn update_env_file(token_manager: &TokenManager) -> io::Result<()> {
    let contents = std::fs::read_to_string(".env")?;
    let mut new_contents = vec![];

    for line in contents.lines() {
        if line.starts_with("TOKEN") {
            let new_line = format!("TOKEN={}", token_manager.token);
            new_contents.push(new_line);
        } else if line.starts_with("TIME_TOKEN_WAS_MADE") {
            let new_line = format!("TIME_TOKEN_WAS_MADE={}", token_manager.time_token_was_made);
            new_contents.push(new_line);
        } else {
            new_contents.push(line.to_string());
        }
    }
    let mut updated_file_contents = String::with_capacity(new_contents.len());
    for line in new_contents {
        updated_file_contents.push_str(&line);
        updated_file_contents.push('\n');
    }
    std::fs::write(".env", updated_file_contents)?;

    Ok(())
}

pub fn save_data_to_file(data: &AircraftData, filename: &str) {
    let json_string = serde_json::to_string_pretty(data).unwrap();
    std::fs::write(filename, json_string).unwrap();
}
