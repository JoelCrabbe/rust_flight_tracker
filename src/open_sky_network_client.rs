use crate::{aircraft_structures::AircraftData, regions_of_interest::BoundingBox, token::TokenManager};

pub struct OpenSkyNetworkClient {
    token_manager: TokenManager,
    http_client: reqwest::blocking::Client,
}

impl OpenSkyNetworkClient {
    pub fn new(token_manager: TokenManager, http_client: reqwest::blocking::Client) -> Self {
        Self {
            token_manager,
            http_client,
        }
    }

    pub fn find_aircraft(&mut self, area: &BoundingBox) -> Result<AircraftData, reqwest::Error> {
        let mut url = "https://opensky-network.org/api/states/all?".to_string();
        let filter = format!(
            "lamin={}&lomin={}&lamax={}&lomax={}",
            area.min_lat, area.min_long, area.max_lat, area.max_long
        );

        url.push_str(&filter);

        let headers = self.token_manager.header();
        let response = self.http_client.get(url).headers(headers).send()?;

        if response.status().is_success() {
            let data = response.json::<AircraftData>()?;
            Ok(data)
        } else {
            panic!("{}", &response.status())
        }
    }
}