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

async function getCoordinates(event) {
    // event is like a struct with two fields, layer and layerType
    // layer is the shape you just drew
    // to see what you can do with shape look in the leaflet docs
    let layer = event.layer;
    drawnItems.addLayer(layer);

    // for a rectangle gets the lat and long of each of the 4 corners
    // tbh this confusing i cant tell what types im dealing with
    let lats = [];
    let longs = [];
    let latLongs = layer.getLatLngs();
    for(let x of latLongs) {
        for(let pair of x) {
            lats.push(pair.lat);
            longs.push(pair.lng);
        }
    }
    const payload = {
        lats,
        longs,
    }

    // console.log(coords);

    // send http request
    try {
        const response = await fetch("http://localhost:3000/coordinates", {
        method: "POST",
        headers: { "Content-Type": "Application/json" },
        body: JSON.stringify(payload),
        });

        if (!response.ok) {
            throw new Error("error receiving response from rust application");
        }

        
    } 
    catch(error) {
        console.log(error);
    }


    // fetch("http://localhost:3000/coordinates", {
    //     method: "POST",
    //     headers: {
    //         "Content-Type": "Application/json",
    //     },
    //     body: JSON.stringify(coords),
    // })
    // .then((response) => response.text())
    // .then((text) => console.log(text));


}

map.on(L.Draw.Event.CREATED, getCoordinates);

/*

event
│
├── layer ──────> the Polyline you just drew
│
└── layerType ──> "polyline"
*/
