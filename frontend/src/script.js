let map = L.map("map").setView([51.505, -0.09], 4);

L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
    attribution: "&copy; OpenStreetMap contributors"
}).addTo(map);


let drawnItems = new L.FeatureGroup();
map.addLayer(drawnItems);


let drawControl = new L.Control.Draw({
    edit: {
        featureGroup: drawnItems
    }
});

// Add toolbar to the map
map.addControl(drawControl);

// everything above this is setup boilerplate I think

function getCoordinates(event) {
    // event is like a struct with two fields, layer and layerType
    // layer is the shape you just drew
    // to see what you can do with shape look in the leaflet docs
    let layer = event.layer;
    drawnItems.addLayer(layer);

    // for a rectangle gets the lat and long of each of the 4 corners
    // tbh this confusing i cant tell what types im dealing with
    let coords = [];
    let latLongs = layer.getLatLngs();
    for(let x of latLongs) {
        for(let pair of x) {
            coords.push(pair.lat);
            coords.push(pair.lng);
        }
    }
    console.log(coords);

    // send http request
    fetch("http://localhost:3000/api/coordinates", {
        method: "POST",
        headers: {
            "Content-Type": "Application/json"
        },
        body: JSON.stringify(coords),
    });


}

map.on(L.Draw.Event.CREATED, getCoordinates);

/*

event
│
├── layer ──────> the Polyline you just drew
│
└── layerType ──> "polyline"
*/
