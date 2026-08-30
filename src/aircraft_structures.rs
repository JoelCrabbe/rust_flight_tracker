/*
* The AicraftCategory field in the API response is only provided
* if the `extended` property is provided in the request URL.
* At the moment, I have not included this field.
*/

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
}

#[derive(Serialize, Deserialize, Debug)]
// Wrapping the vec in an Option as it is possible the response returns null for states
// this happens in the case no aircraft were found in a particular area
pub struct AircraftData {
    pub states: Option<Vec<AircraftInfo>>,
    pub time: i64,
}
