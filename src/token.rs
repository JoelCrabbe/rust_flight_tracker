use reqwest::{
    blocking,
    header::{HeaderMap, HeaderValue},
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils;

const TOKEN_UPDATE_URL: &'static str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

// Tokens expire after 30 minutes. A 401 Unauthorized response means the token has expired - request a new one and retry.
pub struct TokenManager {
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
    pub time_token_was_made: f64,
}

impl TokenManager {
    pub fn new(
        client_id: String,
        client_secret: String,
        token: String,
        time_token_was_made: f64,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            token,
            time_token_was_made,
        }
    }

    pub fn get_token(&mut self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        if now >= self.time_token_was_made + 1770 as f64 {
            self.update_token().unwrap()
        } else {
            self.token.clone() // probably want to avoid clone
        }
    }

    pub fn update_token(&mut self) -> Result<String, reqwest::Error> {
        // update token with 30 seconds left to go on current token to allow for some room
        // do we need to make a client here or can we use the same on from main.rs and instead pass it in as a parameter?
        let client = blocking::Client::new();

        let data = HashMap::from([
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ]);

        let response = client.post(TOKEN_UPDATE_URL).form(&data).send()?;
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
        let _ = utils::update_env_file(self);
        Ok(self.token.clone()) // avoid clone if we can
    }

    pub fn header(&mut self) -> HeaderMap {
        let mut header = HeaderMap::new();
        let val = HeaderValue::from_str(&format!("Bearer {}", self.get_token())).unwrap();
        header.insert("Authorization", val);
        header
    }
}
