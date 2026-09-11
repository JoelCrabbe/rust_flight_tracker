use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{prelude::*, request_handlers::MinMaxLatLong};

#[derive(Clone)]
pub struct OpenSkyNetworkClient {
    pub token_manager: Arc<Mutex<TokenManager>>,
    pub http_client: Arc<reqwest::Client>,
}

impl OpenSkyNetworkClient {
    pub fn new(token_manager: TokenManager, http_client: reqwest::Client) -> Self {
        Self {
            token_manager: Arc::new(Mutex::new(token_manager)),
            http_client: Arc::new(http_client),
        }
    }

    pub async fn find_aircraft(&mut self, payload: MinMaxLatLong) -> Result<AircraftData> {
        let area = BoundingBox::new(
            payload.min_latitude,
            payload.max_latitude,
            payload.min_longitude,
            payload.max_longitude,
        );

        let mut url = "https://opensky-network.org/api/states/all?".to_string();
        let filter = format!(
            "extended=1&lamin={}&lomin={}&lamax={}&lomax={}",
            area.min_latitude, area.min_longitude, area.max_latitude, area.max_longitude
        );

        url.push_str(&filter);

        let headers = self.token_manager.lock().await.header().await;
        let response = self
            .http_client
            .get(url)
            .headers(headers)
            .send()
            .await
            .context("problem retrieving data from OpenSkyNetwork server")?;


        if response.status().is_success() {
            let area_data = response
                .json::<AircraftData>()
                .await
                .context("problem deserializing the OpenSkyNetwork response into json")?;
            return Ok(area_data);
        }
        // TODO: change this, although im not sure what to do here
        panic!("response from OpenSkyNetwork was not successful");
    }
}