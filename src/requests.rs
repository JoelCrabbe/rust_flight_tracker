use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::regions_of_interest::BoundingBox;
use crate::token::TokenManager;

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum PositionSource {
    ADSB = 0,
    ASTERIX = 1,
    MLAT = 2,
    FLARM = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum AircraftCategory {
    NoInfo = 0,
    NoADSB = 1,
    Light = 2,
    Small = 3,
    Large = 4,
    HighVortexLarge = 5,
    Heavy = 6,
    HighPerformance = 7,
    Rotorcraft = 8,
    Glider = 9,
    LighterThanAir = 10,
    Parachutist = 11,
    Ultralight = 12,
    Reserved = 13,
    Unmanned = 14,
    Space = 15,
    EmergencyVehicle = 16,
    ServiceVehicle = 17,
    PointObstacle = 18,
    ClusterObstacle = 19,
    LineObstacle = 20,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AircraftInfo {
    pub icao24: String,
    pub callsign: Option<String>,
    pub origin_country: String,
    pub time_position: Option<i32>,
    pub last_contact: i32,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub baro_altitude: Option<f64>,
    pub on_ground: bool,
    pub velocity: Option<f64>,
    pub true_track: Option<f64>,
    pub vertical_rate: Option<f64>,
    pub sensors: Option<Vec<i32>>,
    pub geo_altitude: Option<f64>,
    pub squawk: Option<String>,
    pub spi: bool,
    pub position_source: PositionSource,
    // pub category: AircraftCategory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AicraftTuple(
    pub String,
    pub Option<String>,
    pub String,
    pub Option<i32>,
    pub i32,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<f64>,
    pub bool,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<f64>,
    pub Option<Vec<i32>>,
    pub Option<f64>,
    pub Option<String>,
    pub bool,
    pub PositionSource,
    // pub AircraftCategory, // only added if extended flag is give in url query
);

impl From<AicraftTuple> for AircraftInfo {
    fn from(value: AicraftTuple) -> Self {
        Self {
            icao24: value.0,
            callsign: value.1,
            origin_country: value.2,
            time_position: value.3,
            last_contact: value.4,
            longitude: value.5,
            latitude: value.6,
            baro_altitude: value.7,
            on_ground: value.8,
            velocity: value.9,
            true_track: value.10,
            vertical_rate: value.11,
            sensors: value.12,
            geo_altitude: value.13,
            squawk: value.14,
            spi: value.15,
            position_source: value.16,
        }
        
    }
}

// i want this function to take in some sort of area, not just a box
// circles aswell i.e. some type that implements some sort of trait
pub fn find_aircraft(
    token_manager: &mut TokenManager,
    client: &Client,
    area: &BoundingBox,
) -> Result<(), reqwest::Error> {
    // what about some sort of builder pattern for the url i.e. create it in steps of chained methods?

    // first need to construct the query
    // NOTE: the true base url should be https://opensky-network.org/api/ however
    // at the moment we are not using the flights/, states/, tracks/ etc endpoints
    let mut url = "https://opensky-network.org/api/states/all?".to_string();
    let filter = format!(
        "lamin={}&lomin={}&lamax={}&lomax={}",
        area.min_lat, area.min_long, area.max_lat, area.max_long
    );
    url.push_str(&filter);

    let response = client.get(url).headers(token_manager.header()).send()?;

    if response.status().is_success() {
        let body: serde_json::Value = response.json()?;
        println!("{}", body);
    } else {
        println!("{}", response.status());
    }

    Ok(())
}
