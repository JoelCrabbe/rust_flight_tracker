import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet-draw";
import "leaflet-draw/dist/leaflet.draw.css";

import { Coordinates, AircraftData, AircraftInfo } from "./types";

export let map!: L.Map;
export let drawnItems!: L.FeatureGroup;
export let markers!: L.Marker[];


export function setupMap() {
    map = L.map("map").setView([51.505, -0.09], 4);
    
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
            attribution: "&copy; OpenStreetMap contributors"
            }).addTo(map);
    
    drawnItems = new L.FeatureGroup();
    map.addLayer(drawnItems);

    let drawControl = new L.Control.Draw({
            edit: {
                featureGroup: drawnItems
            }
        });
    
    map.addControl(drawControl);
}

export async function getCoordinates(event: L.DrawEvents.Created) {
    let layer = event.layer as L.Polyline;
    drawnItems.addLayer(layer);

    let latitudes = [];
    let longitudes = [];
    for (let corners of layer.getLatLngs()) {
        for (let corner of corners as L.LatLng[]) {
            latitudes.push(corner.lat);
            longitudes.push(corner.lng);
        }
    }
    const payload: Coordinates = {
        latitudes: latitudes,
        longitudes: longitudes,
    };

    // send http request to rust server
    try {
        const response = await fetch("http://localhost:3000/coordinates", {
            method: "POST",
            headers: { "Content-Type": "Application/json" },
            body: JSON.stringify(payload),
        });

        if (!response.ok) {
            throw new Error("error receving response from rust server");
        }

        const data: AircraftData = await response.json();
        console.log(data.states);
        if (data.states !== null) {
            for (let aircraft of data.states) {
                addImageToMap("foo", aircraft);
            }
        }
    }
    catch (error) {
        console.log(error);
    }
}

export function addImageToMap(imageUrl: string, aircraft: AircraftInfo) {
    // the ! ignores the null case, i cant imagine when lat/lng would be null
    let marker = L.marker([aircraft.latitude!, aircraft.longitude!]).addTo(map);
    let info = `icao24: ${aircraft.icao24}, Callsign: ${aircraft.callsign}, Origin Country: ${aircraft.origin_country}`;
    marker.bindPopup(info);
}

/*
export function updateMarker(timestamp) {
    console.log("reached");
    for(let dataPoint of markers) {
        console.log(dataPoint);
    }
    requestAnimationFrame(updateMarker)
}
*/

