import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet-draw";
import "leaflet-draw/dist/leaflet.draw.css";

import { MinMaxLatLong, AircraftData, AircraftInfo } from "./types";

export let map: L.Map;
let drawnItems: L.FeatureGroup;
let aircraftIdToMarker: Map<string, L.Marker> = new Map();
let aircraftIdToAircraftInfo: Map<string, AircraftInfo> = new Map();
let startTime: number = 0;
let minLatitude = Infinity;
let maxLatitude = -Infinity;
let minLongitude = Infinity;
let maxLongitude = -Infinity;

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

export async function getCoordinates(event: L.DrawEvents.Created) {
    let layer = event.layer as L.Polyline;
    drawnItems.addLayer(layer);

    for (let corners of layer.getLatLngs() as L.LatLng[][]) {
        for (let corner of corners) {
            minLatitude = Math.min(minLatitude, corner.lat);
            maxLatitude = Math.max(maxLatitude, corner.lat);
            minLongitude = Math.min(minLongitude, corner.lng);
            maxLongitude = Math.max(minLongitude, corner.lng);
        }
    }

    const payload: MinMaxLatLong = {
        minLatitude,
        maxLatitude,
        minLongitude,
        maxLongitude,
    }

    // send http request to rust server
    try {
        const response = await fetch("http://localhost:3000/test", {
            method: "POST",
            headers: { "Content-Type": "Application/json" },
            body: JSON.stringify(payload),
        });

        if (!response.ok) {
            throw new Error("error receving response from rust server");
        }

        const data: AircraftData = await response.json();
        if (data.states !== null) {
            for (let aircraft of data.states) {
                addAircraftToMap("foo", aircraft);
            }
        }
    }
    catch (error) {
        console.log(error);
    }
}

function addAircraftToMap(imageUrl: string, aircraft: AircraftInfo) {
    // the ! ignores the null case, i cant imagine when lat/lng would be null
    // however this could cause bugs if this does happen
    let marker = L.marker([aircraft.latitude!, aircraft.longitude!])
    let info = `icao24: ${aircraft.icao24}, Callsign: ${aircraft.callsign}, Origin Country: ${aircraft.origin_country}`;

    marker.addTo(map);
    marker.bindPopup(info);

    // icao24 -> marker on map
    aircraftIdToMarker.set(aircraft.icao24, marker);

    // icao24 -> info of aircraft
    aircraftIdToAircraftInfo.set(aircraft.icao24, aircraft);
}

export function update(timestamp: number) {
    let dt = timestamp - startTime;
    for (let id of aircraftIdToMarker.keys()) {
        let aircraft = aircraftIdToAircraftInfo.get(id)!;

        updateAircraftPosition(aircraft, dt);
        updateAircraftMarker(aircraft);
        log(aircraft);
    }   
    startTime = timestamp;
    requestAnimationFrame(update);
}

// now have the task of updating the aircrafts position realistically
// at the moment all the aircraft move in the same direction for some reason
// TODO: update aircraft positions realistically
function updateAircraftPosition(aircraft: AircraftInfo, dt: number) {
    aircraft.latitude! += (dt * aircraft.velocity!) / 1_000_000;
    aircraft.longitude! += (dt * aircraft.velocity!) / 1_000_000;
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
    }
    
}

function log(aircraft: AircraftInfo) {
    console.log(`\
            latitude = ${aircraft.latitude},
            longitude = ${aircraft.longitude},
            velocity = ${aircraft.velocity} m/s,
            true_track = ${aircraft.true_track} °`
            );
}