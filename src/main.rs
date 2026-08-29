use reqwest::{self, blocking};
use serde_json::{self, Value};
use std::{
    collections::HashMap,
    error::Error,
    io,
    time::{SystemTime, UNIX_EPOCH},
};

const URL: &'static str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

struct Credentials {
    client_id: String,
    client_secret: String,
    token: String,
    time_token_was_made: f64,
}

fn load_credentials() -> Result<Credentials, Box<dyn Error>> {
    dotenvy::dotenv_override()?;
    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;
    let token = dotenvy::var("TOKEN")?;
    let time_token_was_made = dotenvy::var("TIME_TOKEN_WAS_MADE")?.parse::<f64>()?;

    Ok(Credentials {
        client_id,
        client_secret,
        token,
        time_token_was_made,
    })
}

// Tokens expire after 30 minutes. A 401 Unauthorized response means the token has expired - request a new one and retry.
struct TokenManager {
    token: String,
    time_token_was_made: f64,
}

fn update_env_file(token: &Value, time: f64) -> io::Result<()> {
    let contents = std::fs::read_to_string(".env")?;
    let mut new_contents = vec![];

    for line in contents.lines() {
        if line.starts_with("TOKEN") {
            let new_line = format!("TOKEN={}", token);
            new_contents.push(new_line);
        } else if line.starts_with("TIME_TOKEN_WAS_MADE") {
            let new_line = format!("TIME_TOKEN_WAS_MADE={}", time);
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

impl TokenManager {
    fn new(token: String, time_token_was_made: f64) -> Self {
        Self {
            token,
            time_token_was_made,
        }
    }

    fn update_token(&mut self, client_id: &str, client_secret: &str) -> Result<(), reqwest::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // update token with 30 seconds left to go on current token to allow for some room
        if now >= self.time_token_was_made + 1770 as f64 {
            let client = blocking::Client::new();

            let data = HashMap::from([
                ("grant_type", "client_credentials"),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ]);

            let response = client.post(URL).form(&data).send()?;
            let body = response.text()?;
            let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();

            // get the current time this token was made
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();

            let token = &json_data["access_token"];
            self.token = token.to_string();

            // update .env file with new token value and time token was made
            let _ = update_env_file(token, now);
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let credentials = load_credentials()?;

    let mut token_manager = TokenManager::new(credentials.token, credentials.time_token_was_made);
    let _ = token_manager.update_token(&credentials.client_id, &credentials.client_secret);


    Ok(())
}
