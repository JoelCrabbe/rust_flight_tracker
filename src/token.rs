use anyhow::Result;
use std::collections::HashMap;

use crate::prelude::*;
use crate::utils;

const TOKEN_UPDATE_URL: &str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

// Tokens expire after 30 minutes. A 401 Unauthorized response means the token has expired - request a new one and retry.
#[derive(Clone)]
pub struct TokenManager {
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
    pub time_token_was_made: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenJSONResponse {
    access_token: String,
}

impl TokenManager {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv_override().context("problem loading environment variables")?;

        let client_id = dotenvy::var("CLIENT_ID")
            .context("problem reading `CLIENT_ID` environment variable")?;

        let client_secret = dotenvy::var("CLIENT_SECRET")
            .context("problem reading `CLIENT_SECRET` environment variable")?;

        let token =
            dotenvy::var("TOKEN").context("problem reading `TOKEN` environment variable")?;

        let time_token_was_made = dotenvy::var("TIME_TOKEN_WAS_MADE")
            .context("problem reading `TIME_TOKEN_WAS_MADE` environment variable")?
            .parse::<f64>()
            .context("problem parsing `TIME_TOKEN_WAS_MADE` environment variable into `f64`")?;

        Ok(TokenManager {
            client_id,
            client_secret,
            token,
            time_token_was_made,
        })
    }

    pub async fn get_token(&mut self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // tokens expire 30 minutes after they are made
        // here if the token has less than 30 seconds remaining, it is updated
        if now >= self.time_token_was_made + 1770_f64 {
            match self.update_token().await {
                Ok(token) => token,
                Err(e) => {
                    eprintln!("{e}");
                    // TODO: we are currently returning the old token, not sure if we should do this
                    self.token.clone()
                }
            }
        } else {
            self.token.clone()
        }
    }

    pub async fn update_token(&mut self) -> Result<String> {
        let client = reqwest::Client::new();

        let data = HashMap::from([
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
        ]);

        let response = client
            .post(TOKEN_UPDATE_URL)
            .form(&data)
            .send()
            .await
            .context("problem sending request to OpenSkyNetwork server for new token")?;

        let json_data = response
            .json::<TokenJSONResponse>()
            .await
            .context("problem deserializing the token response into json")?;

        // get the current time this token was made
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // update TokenManager struct fields with new data
        self.token = json_data.access_token;
        self.time_token_was_made = now;

        // update .env file with new token value and time token was made
        if let Err(e) = utils::update_env_file(self) {
            eprintln!("{e}");
        }

        Ok(self.token.clone())
    }

    pub async fn header(&mut self) -> HeaderMap {
        let mut header = HeaderMap::new();
        // I am using expect here as a better alternative to unwrap
        // also I think this will always work so just handling the error for completeness
        let val = HeaderValue::from_str(&format!("Bearer {}", self.get_token().await))
            .expect("couldnt convert token value into a headervalue");
        header.insert("Authorization", val);
        header
    }
}
