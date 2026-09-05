import L from "leaflet";
import { map, setupMap, initializeArea, update, timeBetweenApiCalls, updateAircraft } from "./lib";

setupMap();

// let plane_icon = L.icon({
//     iconUrl: "../assets/aircraft_icon_sprite_sheet.webp",
//     iconSize: [250, 250],
//     iconAnchor: [30, -0.09],
// });

// let marker = L.marker([80, 0], {icon: plane_icon}).addTo(map);
// marker.bindPopup("hello");


map.on(L.Draw.Event.CREATED, event => initializeArea(event as L.DrawEvents.Created));
requestAnimationFrame(update);
setInterval(updateAircraft, timeBetweenApiCalls);