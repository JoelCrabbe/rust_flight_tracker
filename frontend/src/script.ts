import L from "leaflet";
import { map, setupMap, getCoordinates, updateMap } from "./lib";

setupMap();

map.on(L.Draw.Event.CREATED, event => getCoordinates(event as L.DrawEvents.Created));
// requestAnimationFrame(updateMap);