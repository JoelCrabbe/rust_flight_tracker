use anyhow::Result;
use std::collections::HashMap;
use std::env;

use crate::prelude::*;

const TOKEN_UPDATE_URL: &str =
    "https://auth.opensky-network.org/auth/realms/opensky-network/protocol/openid-connect/token";

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

pub async fn update_token(client_id: &str, client_secret: &str) -> Result<(String, f64)> {
    let client = reqwest::Client::new();

    let data = HashMap::from([
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
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

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    Ok((json_data.access_token, now))
}

impl TokenManager {
    pub async fn new() -> Result<Self> {
        dotenvy::dotenv_override().unwrap();
        let client_id =
            env::var("CLIENT_ID").context("problem reading `CLIENT_ID` environment variable")?;

        let client_secret = env::var("CLIENT_SECRET")
            .context("problem reading `CLIENT_SECRET` environment variable")?;

        // need to send request to get new token
        let (token, time_token_was_made) = match update_token(&client_id, &client_secret).await {
            Ok((tok, time)) => (tok, time),
            Err(e) => panic!("{e}"),
        };

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
            match update_token(&self.client_id, &self.client_secret).await {
                Ok((token, time_token_was_made)) => {
                    println!("Updated OpenSkyNetwork API TOKEN");
                    self.token = token;
                    self.time_token_was_made = time_token_was_made;
                    self.token.clone()
                }
                Err(e) => {
                    eprintln!("{e}");
                    self.token.clone() // return old token, this doesn't make sense but idk what else to do atm
                }
            }
        } else {
            self.token.clone() // return old token, this doesn't make sense but idk what else to do atm
        }
    }

    pub async fn header(&mut self) -> HeaderMap {
        let mut header = HeaderMap::new();
        let val = HeaderValue::from_str(&format!("Bearer {}", self.get_token().await))
            .expect("couldnt convert token value into a headervalue");
        header.insert("Authorization", val);
        header
    }
}