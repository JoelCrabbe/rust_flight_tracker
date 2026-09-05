import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet-draw";
import "leaflet-draw/dist/leaflet.draw.css";

import { MinMaxLatLong, AircraftData, AircraftInfo } from "./types";

export let map: L.Map;
let drawnItems: L.FeatureGroup;
let aircraftIdToMarker: Map<string, L.Marker> = new Map();
let aircraftIdToAircraftInfo: Map<string, AircraftInfo> = new Map();
let startTime = 0;
let minLatitude = Infinity;
let maxLatitude = -Infinity;
let minLongitude = Infinity;
let maxLongitude = -Infinity;
export let payload: MinMaxLatLong;

const earthRadius = 6_371_000;
export const timeBetweenApiCalls = 10_000;

export function setupMap() {
    map = L.map("map").setView([51.505, -0.09], 4);
    
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
            attribution: "&copy; OpenStreetMap contributors"
            }).addTo(map);
    
    drawnItems = new L.FeatureGroup();
    map.addLayer(drawnItems);

    let drawControl = new L.Control.Draw({
            edit: {
                featureGroup: drawnItems,
            }
        });
    
    map.addControl(drawControl);
}

export async function initializeArea(event: L.DrawEvents.Created) {
    let layer = event.layer as L.Polyline;
    drawnItems.addLayer(layer);

    for (let corners of layer.getLatLngs() as L.LatLng[][]) {
        for (let corner of corners) {
            minLatitude = Math.min(minLatitude, corner.lat);
            maxLatitude = Math.max(maxLatitude, corner.lat);
            minLongitude = Math.min(minLongitude, corner.lng);
            maxLongitude = Math.max(maxLongitude, corner.lng);
        }
    }

    payload = {
    minLatitude,
    maxLatitude,
    minLongitude,
    maxLongitude,
    }

    console.log(payload);

    let data: AircraftData = (await fetchData())!;
    if (data.states !== null) {
        for (let aircraft of data.states) {
            addAircraftToMap(aircraft);
        }
    }
}

function addAircraftToMap(aircraft: AircraftInfo) {
    // the ! ignores the null case, i cant imagine when lat/lng would be null
    // however this could cause bugs if this does happen
    let marker = L.marker([aircraft.latitude!, aircraft.longitude!])
    let info = getInfo(aircraft);
    marker.addTo(map);
    marker.bindPopup(info);

    // icao24 -> marker on map
    aircraftIdToMarker.set(aircraft.icao24, marker);

    // icao24 -> info of aircraft
    aircraftIdToAircraftInfo.set(aircraft.icao24, aircraft);
}

export function update(timestamp: number) {
    let dt = timestamp - startTime;
    for (let aircraft of aircraftIdToAircraftInfo.values()) {
        updateAircraftPosition(aircraft, dt);
        updateAircraftMarker(aircraft);
        // console.log(getInfo(aircraft))
    }
    startTime = timestamp;
    requestAnimationFrame(update);
}

// Updating aircraft's gps coordinates using the Haversine formula
function updateAircraftPosition(aircraft: AircraftInfo, dt: number) {
    let distanceM = dt * 0.001 * aircraft.velocity!;
    let latR = aircraft.latitude! * (Math.PI / 180);
    let longR = aircraft.longitude! * (Math.PI / 180);
    let angle = aircraft.true_track! * (Math.PI / 180);
    let angularDistance = distanceM / earthRadius;
    let newLatR = Math.asin(Math.sin(latR) * Math.cos(angularDistance) + Math.cos(latR) * Math.sin(angularDistance) * Math.cos(angle));
    let dLonR = Math.atan2(Math.sin(angle) * Math.sin(angularDistance) * Math.cos(latR), Math.cos(angularDistance) - Math.sin(latR) * Math.sin(newLatR));
    let newLongR = longR + dLonR;
    let newLatD = newLatR * (180 / Math.PI);
    let newLongD = newLongR * (180 / Math.PI);
    aircraft.latitude! = newLatD;
    aircraft.longitude! = newLongD;
}

function updateAircraftMarker(aircraft: AircraftInfo) {
    if (aircraft.latitude! < minLatitude ||
        aircraft.latitude! > maxLatitude ||
        aircraft.longitude! < minLongitude ||
        aircraft.longitude! > maxLongitude)
        {
            // remove marker from map
            map.removeLayer(aircraftIdToMarker.get(aircraft.icao24)!);

            // remove marker from hashmaps
            aircraftIdToMarker.delete(aircraft.icao24);
            aircraftIdToAircraftInfo.delete(aircraft.icao24);
        }
    else {
        let marker = aircraftIdToMarker.get(aircraft.icao24)!;
        marker.setLatLng([aircraft.latitude!, aircraft.longitude!]);
        marker.bindPopup(getInfo(aircraft));
    }
}

function getInfo(aircraft: AircraftInfo): string {
    let mph = aircraft.velocity! * 2.237;
    let baro_altitude_ft = aircraft.baro_altitude! * 3.281;
    return `\
        Callsign = ${aircraft.callsign}
        Latitude = ${aircraft.latitude!.toFixed(5)}
        Longitude = ${aircraft.longitude!.toFixed(5)}
        Ground Speed = ${mph.toFixed(2)} mph
        Barometric Altitude = ${baro_altitude_ft.toFixed(0)} ft
        Track = ${aircraft.true_track!} °`
}

// at some point would like to replace markes with actual icons of the aircraft based on what they actually are
// e.g. helicopter icon for helicopter, glider icon for gliders etc
// I thought i would be able to get this information via the AircraftCategory field but it looks like most responses
// have this set to 0 which means no information available, so we cannot identify the type of aircraft that way.

export async function fetchData(): Promise<AircraftData | null> {
    try {
        const response = await fetch("http://localhost:3000/coordinates", {
            method: "POST",
            headers: { "Content-Type": "Application/json" },
            body: JSON.stringify(payload),
        });

        if (!response.ok) {
            throw new Error("error receiving response from rust server");
        }

        const data: AircraftData = await response.json();
        return data;
    }
    catch (error) {
        console.log(error);
        return null;
    }
}

export async function updateAircraft() {
    let data: AircraftData = (await fetchData())!;
    if (data.states !== null) {
        for (let aircraft of data.states) {
            aircraftIdToMarker.has
            if (!aircraftIdToMarker.has(aircraft.icao24)) {
                addAircraftToMap(aircraft);
            }
            aircraftIdToAircraftInfo.set(aircraft.icao24, aircraft);
        }
    }
}
