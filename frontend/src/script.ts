import L from "leaflet";
import { setupMap, map, Area, timeBetweenApiCalls } from "./lib"

let time = 0;

setupMap();

let areas: Area[] = [];

// every time a rectangle is drawn, run this code
map.on(L.Draw.Event.CREATED, async (event) => {
    let area = new Area();
    await area.initializeArea(event as L.DrawEvents.Created);
    areas.push(area);
})

// every 5 seconds run this function
setInterval(periodicUpdate, timeBetweenApiCalls);

// every frame run this function
requestAnimationFrame(update);

async function periodicUpdate() {
    for (let area of areas) {
        await area.updateAircrafts();
    }
}

function update(timestamp: number) {
    let dt = timestamp - time;
    for (let area of areas) {
        for (let [aircraft, _] of area.monitoredAircraft.values()) {
            area.updateAircraftPosition(aircraft, dt);
            area.updateAircraftMarkerAndPath(aircraft);
        }
    }
    time = timestamp;
    requestAnimationFrame(update);
}
