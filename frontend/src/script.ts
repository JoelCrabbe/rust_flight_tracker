import L from "leaflet";
import { map, setupMap, initializeArea, update, timeBetweenApiCalls, updateAircrafts } from "./lib";

setupMap();

map.on(L.Draw.Event.CREATED, event => initializeArea(event as L.DrawEvents.Created));
requestAnimationFrame(update);
setInterval(updateAircrafts, timeBetweenApiCalls);