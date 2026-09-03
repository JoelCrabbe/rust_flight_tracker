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

function addImageToMap(imageUrl, aircraft) {
    // let imageBounds = [[latitude, longitude], [latitude, longitude]];
    // L.imageOverlay(imageUrl, imageBounds).addTo(map)
    let marker = L.marker([aircraft.latitude, aircraft.longitude]).addTo(map);
    let info = `icao24: ${aircraft.icao24}, Callsign: ${aircraft.callsign}, Origin Country: ${aircraft.origin_country}`;
    marker.bindPopup(info);
}

async function getCoordinates(event) {
    // event is like a struct with two fields, layer and layerType
    // layer is the shape you just drew
    // to see what you can do with shape look in the leaflet docs
    let layer = event.layer;
    drawnItems.addLayer(layer);

    // for a rectangle gets the lat and long of each of the 4 corners
    // tbh this confusing i cant tell what types im dealing with
    let latitudes = [];
    let longitudes = [];
    let latLongs = layer.getLatLngs();
    for(let x of latLongs) {
        for(let pair of x) {
            latitudes.push(pair.lat);
            longitudes.push(pair.lng);
        }
    }
    const payload = {
        latitudes,
        longitudes,
    }

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
        
        const data = await response.json();
        console.log(data);
        if (data.states !== null) {
            for (let aircraft of data.states) {
                addImageToMap("foo", aircraft);
            }
        } else {
            console.log("no aircraft in this area at this moment in time");
        }
    }
    catch(error) {
        console.log(error);
    }

}

map.on(L.Draw.Event.CREATED, getCoordinates);

/*

event
│
├── layer ──────> the Polyline you just drew
│
└── layerType ──> "polyline"
*/
