use anyhow::Result;
use std::collections::HashMap;

use crate::prelude::*;
use crate::utils;

const TOKEN_UPDATE_URL: &str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

// Tokens expire after 30 minutes. A 401 Unauthorized response means the token has expired - request a new one and retry.
pub struct TokenManager {
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
    pub time_token_was_made: f64,
}

/*
#[serde(deny_unknown_fields)]
Always error during deserialization when encountering unknown fields.
When this attribute is not present, by default unknown fields are ignored for self-describing formats like JSON.
This struct only needs the one field i actually care about in the response, all others are ignored by default
*/
#[derive(Serialize, Deserialize, Debug)]
struct TokenJSONResponse {
    access_token: String,
}

impl TokenManager {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv_override().context("loading environment variables")?;

        let client_id =
            dotenvy::var("CLIENT_ID").context("reading `CLIENT_ID` environment variable")?;

        let client_secret = dotenvy::var("CLIENT_SECRET")
            .context("reading `CLIENT_SECRET` environment variable")?;

        let token = dotenvy::var("TOKEN").context("reading `TOKEN` environment variable")?;

        let time_token_was_made = dotenvy::var("TIME_TOKEN_WAS_MADE")
            .context("reading `TIME_TOKEN_WAS_MADE` environment variable")?
            .parse::<f64>()
            .context("problem parsing `TIME_TOKEN_WAS_MADE` environment variable into `f64`")?;

        Ok(TokenManager {
            client_id,
            client_secret,
            token,
            time_token_was_made,
        })
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
        let client = Client::new();

        let data = HashMap::from([
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ]);

        let response = client.post(TOKEN_UPDATE_URL).form(&data).send()?;
        let json_data: TokenJSONResponse = response.json()?;

        // get the current time this token was made
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // update TokenManager struct fields with new data
        self.token = json_data.access_token;
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
