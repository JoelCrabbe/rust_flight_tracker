/*
* The AicraftCategory field in the API response is only provided
* if the `extended` property is provided in the request URL.
* At the moment, I have not included this field.
*/

use serde_repr::{Serialize_repr, Deserialize_repr};
use serde::{Serialize, Deserialize};

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
);

// implementing this trait tells us how to convert
// from an AircraftTuple to an AircraftInfo struct
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

#[derive(Deserialize, Debug)]
pub struct AircraftData {
    pub states: Vec<AircraftInfo>,
    pub time: i64,
}

