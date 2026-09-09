# use a base image which has node/npm on it
FROM node:latest AS frontend

# make a directory called frontend
WORKDIR /frontend

# copy files in my frontend folder into frontend folder in the docker image
COPY frontend/ .

# npm install the packages needed
RUN npm install

# npm build the frontend
RUN npm run build

# use a base image which has rust on it
FROM rust:latest AS backend

# make a directory called app
WORKDIR /backend

# copy Cargo.toml and Cargo.lock into the app directory
COPY Cargo.toml .
COPY Cargo.lock .

# copy the contents of src directory into /app/src
COPY src/ ./src

# build the backend
RUN cargo build --release

# use ubuntu as the os for this image
FROM ubuntu:latest

# need ca-certificates to work with reqwest
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# make a directory called app
WORKDIR /app

# copy the frontend/dist folder into this new folder
COPY --from=frontend /frontend/dist ./frontend/dist

# copy the rust executable into the working directory
COPY --from=backend /backend/target/release/rust_flight_tracker .

CMD [ "./rust_flight_tracker" ]