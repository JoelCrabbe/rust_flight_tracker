use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking;
use std::collections::HashMap;

use crate::utils;

const URL: &'static str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

// Tokens expire after 30 minutes. A 401 Unauthorized response means the token has expired - request a new one and retry.
pub struct TokenManager {
    pub token: String,
    pub time_token_was_made: f64,
}

impl TokenManager {
    pub fn new(token: String, time_token_was_made: f64) -> Self {
        Self {
            token,
            time_token_was_made,
        }
    }

    pub fn update(&mut self, client_id: &str, client_secret: &str) -> Result<(), reqwest::Error> {
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

            // update TokenManager struct fields with new data
            let token = &json_data["access_token"];
            self.token = token.to_string();
            self.time_token_was_made = now;

            // update .env file with new token value and time token was made
            let _ = utils::update_env_file(token, now);
        }
        Ok(())
    }
}