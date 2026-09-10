# Rust Flight Tracker

## Demo

[![rust flight tracker demo](https://img.youtube.com/vi/NZEWJpPLk2s/maxresdefault.jpg)](https://www.youtube.com/watch?v=NZEWJpPLk2s)

## What is it?
Rust flight tracker is a rust / typescript project which uses the [opensky-network](https://opensky-network.org/) API to track  aircraft in realtime, in user defined areas of the world.
Users have access to realtime data about aircraft such as latitude, longitude, velocity, altitude, callsign etc and receive frequent updates roughly every 5 seconds.

## General Info
The app makes requests to the opensky-network API every 5 seconds. This means in between api calls, aircraft's positions are being interpolated.
I used the [Haversine Formula](https://en.wikipedia.org/wiki/Haversine_formula) to work out how to update aircraft's positions in between real datapoints from the API. This works ok, however it assumes that aircraft are travelling at a constant speed which is not true in reality and when a new datapoint comes in, (symbolised by the white dot along the aircraft's path) aircraft can suddenly jump to a new position. This is something I would like to improve in the future.

## External Tools used in this project
- [Leaflet JS](https://leafletjs.com/)

    - This is the javascript framework I used to provide the interactive map the app uses.

- [Leaflet Draw](https://leaflet.github.io/Leaflet.draw/docs/leaflet-draw-latest.html)

    - This is a plugin to the Leaflet JS framework which allows you to draw shapes on the map. You can choose to add a toolbar to the left side of the map which provides you with shapes you can draw, edit, delete etc.

## Project Architecture

### Backend
On the backend, I am using Rust and particularly the [axum](https://crates.io/crates/axum) crate to act as the server for this project. The rust backend acts as the middle man between the frontend and the opensky-network API. The backend is responsible for listening out for requests sent from the frontend, translating that request into a suitable post request which can be sent to the opensky-network API and then relaying the data received back to the frontend, in a structured format, so that the frontend can use this data to update all of the aircrafts positions and information.

## Frontend
On the frontend, I am using typescript. I tried using javascript at the start but soon realised I couldn't reliably tell what the type of my variables were and so switched to typescript. The frontend is responsible for listening for events the user might input e.g. drawing a shape on the screen, capturing key information from that event, such as the min/max latitude and longitude values from that shape and passing it to the backend, to be sent off to the API. The frontend is constantly updating aircraft positions, in between API calls to simulate aircraft moving in realtime.


## Things which still need to be done / improved
- Writing unit tests
- improving rust error handling
- improving typescript async await error handling
- replace marker icons with aircraft icons
- Deployment to internet

## Problems I faced during this project

### Deployment Issues
After I got the project working locally, I wanted to make it available as a website on the internet and so I needed a cloud instance to run the server. I chose to use an AWS EC2 instance. I also needed to package up my project in a container to run on the ec2 instance and so used docker. However I soon found out that the opensky-network API blocks the AWS IP range and so none of my api requests were making it through, hence I could not use AWS or any other cloud providers to host my server.