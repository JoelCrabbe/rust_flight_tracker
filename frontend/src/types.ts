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

}

export interface AircraftData {
    states: AircraftInfo[] | null,
    time: number,
}