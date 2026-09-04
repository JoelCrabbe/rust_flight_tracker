import L from "leaflet";
import { map, setupMap, getCoordinates, update } from "./lib";

setupMap();

map.on(L.Draw.Event.CREATED, event => getCoordinates(event as L.DrawEvents.Created));
requestAnimationFrame(update);