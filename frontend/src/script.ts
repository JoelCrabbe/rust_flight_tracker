import L from "leaflet";
import { setupMap, map, Area, timeBetweenApiCalls } from "./lib"

let time = 0;
// I think starting the time at 0 means the first dt will be the number of milliseconds the program has been running
// which means when we update position there could be a big jump as the data is as old as the program has been running

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
        for (let [_, aircraft] of area.monitoredAircraft.values()) {
            area.updateAircraftPosition(aircraft, dt);
            area.updateAircraftMarker(aircraft);
        }
    }
    time = timestamp;
    requestAnimationFrame(update);
}
