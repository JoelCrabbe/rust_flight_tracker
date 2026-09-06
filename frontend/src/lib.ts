import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet-draw";
import "leaflet-draw/dist/leaflet.draw.css";

import { MinMaxLatLong, AircraftData, AircraftInfo, } from "./types";

export let map: L.Map;
let drawnItems: L.FeatureGroup;

let monitoredAircraft: Map<string, [L.Marker, AircraftInfo]> = new Map();

let startTime = 0;

let minLatitude = Infinity;
let maxLatitude = -Infinity;
let minLongitude = Infinity;
let maxLongitude = -Infinity;

export let payload: MinMaxLatLong;
let firstCallMade = false;

const earthRadius = 6_371_000;
export const timeBetweenApiCalls = 5_000;

export function setupMap() {
    map = L.map("map").setView([51.505, -0.09], 4);

    // revealing an api key, must we do this? does it matter? can we hide it in an environment variable?
    L.tileLayer("https://api.maptiler.com/maps/hybrid-v4/256/{z}/{x}/{y}.jpg?key=DoZE0UNdz0voU0cNhss2", {
        attribution: "&copy; OpenStreetMap contributors",
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
    switch (event.layerType) {
        case "polyline": {}
        case "polygon": {}
        case "circle": {}
        case "marker": {}
        case "rectangle": {
            firstCallMade = true;
            let rectangle = event.layer as L.Rectangle;
            // I'm unsure as to what drawnItems actually does, I know we add the toolbar to it but do I need to add everything I add
            // to the map the the drawnItems FeatureGroup?
            drawnItems.addLayer(rectangle); 
            for (let corners of rectangle.getLatLngs() as L.LatLng[][]) {
                for (let corner of corners) {
                    minLatitude = Math.min(minLatitude, corner.lat);
                    maxLatitude = Math.max(maxLatitude, corner.lat);
                    minLongitude = Math.min(minLongitude, corner.lng);
                    maxLongitude = Math.max(maxLongitude, corner.lng);
                }
            }
            payload = { minLatitude, maxLatitude, minLongitude, maxLongitude, };
            console.log(payload);

            let data = await fetchData();
            if (data) {
                if (data.states) {
                    for (let aircraft of data.states) {
                        addAircraftToMap(aircraft);
                    }
                }
            }
        }
    }
}

function addAircraftToMap(aircraft: AircraftInfo) {
    // if an aircraft is lacking any of these fields we will simply not render it to the map
    // as these are required for knowing and updating position
    if (!aircraft.latitude || !aircraft.longitude || !aircraft.velocity || !aircraft.true_track) {
        return;
    }
    let marker = L.marker([aircraft.latitude, aircraft.longitude])
    let info = getInfo(aircraft);
    marker.addTo(map);
    marker.bindPopup(info);

    // Note: we only reach the code down here if the aircraft has a latitude, longitude, velocity, true_track and is on the map
    // this guarantees that for every entry in these hashmaps, it is safe to access these fields using !

    // associate this aircrafts id with a marker on the map on its information
    monitoredAircraft.set(aircraft.icao24, [marker, aircraft]);
}

export function update(timestamp: number) {
    let dt = timestamp - startTime;
    for (let [_, aircraft] of monitoredAircraft.values()) {
        updateAircraftPosition(aircraft, dt);
        updateAircraftMarker(aircraft);
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
    let [marker, _] = monitoredAircraft.get(aircraft.icao24)!;

    if (aircraft.latitude! < minLatitude ||
        aircraft.latitude! > maxLatitude ||
        aircraft.longitude! < minLongitude ||
        aircraft.longitude! > maxLongitude)
        {
            // remove marker from map and hashmap of aircraft we are monitoring
            // the aircraft has left the area we are monitoring, we are no longer interested in it
            map.removeLayer(marker);
            monitoredAircraft.delete(aircraft.icao24);
        }
    else {
        marker.setLatLng([aircraft.latitude!, aircraft.longitude!]);
        marker.bindPopup(getInfo(aircraft));
    }
}

function getInfo(aircraft: AircraftInfo): string {
    // when we call this function we know aircraft has a latitude and longitude
    // hence we can ignore the null cases and use !
    let callsign = aircraft.callsign ? aircraft.callsign : "N/A";
    let latitude = aircraft.latitude!.toFixed(5);
    let longitude = aircraft.longitude!.toFixed(5);
    let mph = aircraft.velocity ? (aircraft.velocity * 2.237).toFixed(2).toString() + " mph" : "N/A";
    let baro_altitude_ft = aircraft.baro_altitude ? (aircraft.baro_altitude * 3.281).toFixed(0).toString() + " ft" : "N/A";
    let true_track = aircraft.true_track ? aircraft.true_track.toString() + " °" : "N/A";
    return `\
        Callsign = ${callsign}
        Latitude = ${latitude}
        Longitude = ${longitude}
        Ground Speed = ${mph}
        Barometric Altitude = ${baro_altitude_ft}
        Track = ${true_track}`
}

export async function fetchData(): Promise<AircraftData | null> {
    try {
        const response = await fetch("http://localhost:3000/coordinates", {
            method: "POST",
            headers: { "Content-Type": "Application/json" },
            body: JSON.stringify(payload),
        });

        if (!response.ok) {
            // when to throw an error vs print it
            console.error(`error receiving response from the rust server, status code: ${response.status}`);
            return null;
        }
        try {
            const data: AircraftData = await response.json();
            return data;
        } catch (error) {
            console.error("error parsing the response into the AircraftData struct");
            return null;
        }
    } catch (error) {
        console.error(error);
        return null;
    }
}

export async function updateAircrafts() {
    if (!firstCallMade) {
        return;
    }
    let data = await fetchData();
    if (data) {
        if (data.states) {
            for (let aircraft of data.states) {
                if (!monitoredAircraft.has(aircraft.icao24)) {
                    addAircraftToMap(aircraft);
                }
                // update this aircrafts info to the data we just retrieved
                let [marker, _] = monitoredAircraft.get(aircraft.icao24)!;
                monitoredAircraft.set(aircraft.icao24, [marker, aircraft]);
            }

        }
    }
}

/*
each api query cost is calculated by the area covered by the box you make on the map
if the box covers more than 400 square degrees (calculated by bounding box area in sq° = latitude range × longitude range)
then this request costs 4 tokens, so I can make 1000 of these requests in a day.
It is not dependent on how many aircraft are in the box, which I thought it would have depended on.
if latitude and longitude values are not supplied in the query I think it also costs 4 tokens
increasing how often we request data will make everything more accurate
but will also use tokens more often

Tomorrow maybe we should focus on error handling, starting with the rust code especially all of the unwraps
look into if it is possible to draw lines representing the path each plane/marker makes once it is in the airspace

TODO: try to see if there is a better way to handle errors for async await
*/