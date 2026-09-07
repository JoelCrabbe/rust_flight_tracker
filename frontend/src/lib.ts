import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet-draw";
import "leaflet-draw/dist/leaflet.draw.css";

import { AircraftData, AircraftInfo, MinMaxLatLong } from "./types";

const earthRadius = 6_371_000;
export const timeBetweenApiCalls = 5_000;

export let map: L.Map;
let drawnItems: L.FeatureGroup;


export function setupMap() {
    map = L.map("map").setView([51.505, -0.09], 4);

    // revealing an api key, must we do this? does it matter? can we hide it in an environment variable?
    L.tileLayer("https://api.maptiler.com/maps/hybrid-v4/256/{z}/{x}/{y}.jpg?key=DoZE0UNdz0voU0cNhss2", {
        attribution: "&copy; OpenStreetMap contributors",
        }).addTo(map);
    
    drawnItems = new L.FeatureGroup();
    map.addLayer(drawnItems);

    let drawControl = new L.Control.Draw({
            draw: {
                polyline: false,
                polygon: false,
                circle: false,
                marker: false,
                circlemarker: false,
            },
            edit: {
                featureGroup: drawnItems,
            }
        });
    
    map.addControl(drawControl);
}

export class Area {
    monitoredAircraft: Map<string, [L.Marker, AircraftInfo, L.Polyline, L.CircleMarker[]]>;
    firstCallMade: boolean;
    minLatitude: number;
    maxLatitude: number;
    minLongitude: number;
    maxLongitude: number;
    payload: MinMaxLatLong;

    constructor() {
        this.monitoredAircraft = new Map();
        this.firstCallMade = false;
        this.minLatitude = Infinity;
        this.maxLatitude = -Infinity;
        this.minLongitude = Infinity;
        this.maxLongitude = -Infinity;
        this.payload = { minLatitude: 0, maxLatitude: 0, minLongitude: 0, maxLongitude: 0, };
    }

    async initializeArea(event: L.DrawEvents.Created) {
        switch (event.layerType) {
            case "polyline": {}
            case "polygon": {}
            case "circle": {}
            case "circlemarker": {}
            case "marker": {}
            case "rectangle": {
                this.firstCallMade = true;
                let rectangle = event.layer as L.Rectangle;
                drawnItems.addLayer(rectangle);

                let corners = rectangle.getLatLngs() as L.LatLng[][];
                [this.minLatitude, this.maxLatitude, this.minLongitude, this.maxLongitude] = getMinMax(corners);
                this.payload = {
                    minLatitude: this.minLatitude,
                    maxLatitude: this.maxLatitude,
                    minLongitude: this.minLongitude,
                    maxLongitude: this.maxLongitude,
                }

                let data = await fetchData(this.payload);
                if (data) {
                    if (data.states) {
                        for (let aircraft of data.states) {
                            if (aircraft.latitude && aircraft.longitude && aircraft.velocity && aircraft.true_track) {
                                this.addAircraftToMap(aircraft);
                            }
                        }
                    }
                }
            }
        }
    }

    addAircraftToMap(aircraft: AircraftInfo) {
        let marker = L.marker([aircraft.latitude!, aircraft.longitude!]);
        let info = getInfo(aircraft);
        marker.addTo(map);
        marker.bindPopup(info);

        let path = L.polyline([L.latLng(aircraft.latitude!, aircraft.longitude!)], { color: colorFromAltitude(aircraft.baro_altitude)});
        // path.redraw()

        let datapoints: L.CircleMarker[] = [];

        // add a small marker at the coordinates when real data from api came in
        datapoints.push(L.circleMarker(L.latLng(aircraft.latitude!, aircraft.longitude!), { color: "white", radius: 1}));

        marker.addEventListener("popupopen", () => {
            map.addLayer(path);
            for (let dp of datapoints) {
                map.addLayer(dp);
            }
        });

        marker.addEventListener("popupclose", () => {
            map.removeLayer(path);
            for (let dp of datapoints) {
                map.removeLayer(dp);
            }
        });

        this.monitoredAircraft.set(aircraft.icao24, [marker, aircraft, path, datapoints]);
    }

    updateAircraftPosition(aircraft: AircraftInfo, dt: number) {
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

    updateAircraftMarkerAndPath(aircraft: AircraftInfo) {
        let [marker, _, path, datapoints] = this.monitoredAircraft.get(aircraft.icao24)!;
    
        if (aircraft.latitude! < this.minLatitude ||
            aircraft.latitude! > this.maxLatitude ||
            aircraft.longitude! < this.minLongitude ||
            aircraft.longitude! > this.maxLongitude)
            {
                // remove marker from map and hashmap of aircraft we are monitoring
                // the aircraft has left the area we are monitoring, we are no longer interested in it
                map.removeLayer(marker);
                map.removeLayer(path);
                for (let dp of datapoints) {
                    map.removeLayer(dp);
                }

                this.monitoredAircraft.delete(aircraft.icao24);
            }
        else {
            marker.setLatLng([aircraft.latitude!, aircraft.longitude!]);
            marker.bindPopup(getInfo(aircraft));
            path.addLatLng(L.latLng(aircraft.latitude!, aircraft.longitude!));
        }
    }

    async updateAircrafts() {
        if (!this.firstCallMade) {
            return;
        }
        let data = await fetchData(this.payload);
        if (data) {
            if (data.states) {
                for (let aircraft of data.states) {
                    if (!this.monitoredAircraft.has(aircraft.icao24)) {
                        if (aircraft.latitude && aircraft.longitude && aircraft.velocity && aircraft.true_track) {
                            this.addAircraftToMap(aircraft);
                        }
                    } else {
                        // update this aircrafts info to the data we just retrieved
                        let [marker, _, path, datapoints] = this.monitoredAircraft.get(aircraft.icao24)!;
                        this.monitoredAircraft.set(aircraft.icao24, [marker, aircraft, path, datapoints]);
                        datapoints.push(L.circleMarker(L.latLng(aircraft.latitude!, aircraft.longitude!), { color: "white", radius: 1 }));
                    }                
                }
            }
        }
    }
}

async function fetchData(payload: MinMaxLatLong): Promise<AircraftData | null> {
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

function getInfo(aircraft: AircraftInfo): string {
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

function getMinMax(latlngArray: L.LatLng[][]): number[] {
    let minLatitude = Infinity;
    let maxLatitude = -Infinity;
    let minLongitude = Infinity;
    let maxLongitude = -Infinity;
    for (let corners of latlngArray) {
        for (let corner of corners) {
            minLatitude = Math.min(minLatitude, corner.lat);
            maxLatitude = Math.max(maxLatitude, corner.lat);
            minLongitude = Math.min(minLongitude, corner.lng);
            maxLongitude = Math.max(maxLongitude, corner.lng);
        }
    }
    return [
        minLatitude,
        maxLatitude,
        minLongitude,
        maxLongitude,
    ];
}

function colorFromAltitude(altitude: number | null): string {
    if (altitude == null) { return "#000000" };
    if (altitude < 100) { return "#ffffff" };
    if (altitude < 300) { return "#f0ff00" };
    if (altitude < 2_000) { return "#00ff9c" };
    if (altitude < 3_500) { return "#00eaff" };
    if (altitude < 5_500) { return "#0077ff" };
    if (altitude < 8_500) { return "#2200ff" };
    if (altitude < 10_500) { return "#7700ff" };
    if (altitude < 12_500) { return "#ae00ff" };
    return "#ff0000";
}

/*
issues at the moment are:
aircraft can still jump around quite a bit when new data comes in which makes it seem like the interpreted position of the aircraft
in the 5 seconds we don't have data isn't very accurate
*/

