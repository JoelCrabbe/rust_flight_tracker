use anyhow::Result;

use crate::Coordinates;
use crate::prelude::*;
use crate::utils;

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

    pub async fn find_aircraft(&mut self, coordinates: Coordinates) -> Result<AircraftData> {
        let (min_lat, max_lat) = utils::get_min_max(&coordinates.latitudes);
        let (min_long, max_long) = utils::get_min_max(&coordinates.longitudes);
        let area = BoundingBox::new(min_lat, max_lat, min_long, max_long);

        let mut url = "https://opensky-network.org/api/states/all?".to_string();
        let filter = format!(
            "lamin={}&lomin={}&lamax={}&lomax={}",
            area.min_lat, area.min_long, area.max_lat, area.max_long
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
