use anyhow::Result;

use crate::MinMaxLatLng;
use crate::prelude::*;

#[derive(Clone)]
pub struct OpenSkyNetworkClient {
    pub token_manager: TokenManager,
    pub http_client: reqwest::Client,
}

impl OpenSkyNetworkClient {
    pub fn new(token_manager: TokenManager, http_client: reqwest::Client) -> Self {
        Self {
            token_manager,
            http_client,
        }
    }

    pub async fn find_aircraft(&mut self, coordinates: MinMaxLatLng) -> Result<AircraftData> {
        let area = BoundingBox::new(
            coordinates.min_latitude,
            coordinates.max_latitude,
            coordinates.min_longitude,
            coordinates.max_longitude,
            );

        let mut url = "https://opensky-network.org/api/states/all?".to_string();
        let filter = format!(
            "lamin={}&lomin={}&lamax={}&lomax={}",
            area.min_latitude, area.min_longitude, area.max_latitude, area.max_longitude
        );

        url.push_str(&filter);

        let headers = self.token_manager.header().await;
        let response = self.http_client.get(url).headers(headers).send().await?;

        if response.status().is_success() {
            let area_data = response.json::<AircraftData>().await?;
            return Ok(area_data);
        } else {
            panic!("response from OpenSkyNetwork was not successfull");
        }
    }
}
