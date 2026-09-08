export interface MinMaxLatLong {
    minLatitude: number,
    maxLatitude: number,
    minLongitude: number,
    maxLongitude: number,
}

export enum PositionSource {
    Adsb = 0,
    Asterix = 1,
    Mlat = 2,
    Flarm = 3,
}

export enum AircraftCategory {
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

export interface AircraftInfo {
    icao24: string,
    callsign: string | null,
    origin_country: string,
    time_position: number | null,
    last_contact: number,
    longitude: number | null,
    latitude: number | null,
    baro_altitude: number | null,
    on_ground: boolean,
    velocity: number | null,
    true_track: number | null,
    vertical_rate: number | null,
    sensors: number[] | null,
    geo_altitude: number | null,
    squawk: string | null,
    spi: boolean,
    position_source: PositionSource,
    aircraft_category: AircraftCategory,
}

export interface AircraftData {
    states: AircraftInfo[] | null,
    time: number,
}

export interface AircraftUI {
    marker: L.Marker,
    path: L.Polyline,
    datapoints: L.FeatureGroup<L.CircleMarker>,
}