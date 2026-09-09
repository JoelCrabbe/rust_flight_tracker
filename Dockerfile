# use a base image which has node/npm on it
FROM node:latest AS frontend

# make the working directory called frontend
WORKDIR /frontend

# copy files in my frontend folder into the working directory of the docker image
COPY frontend/ .

# npm install the packages needed
RUN npm install

# npm build the frontend
RUN npm run build

# use a base image which has rust on it
FROM rust:latest AS backend

# make the working directory src folder
WORKDIR /app

# copy files in src into working directory
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ ./src

# build the backend
RUN cargo build --release

# use ubuntu as the os for this image
FROM ubuntu:latest

# need ca-certificates to work with reqwest
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# set /app as the working directory of final image
WORKDIR /app

# copy the frontend/dist folder into this new folder
COPY --from=frontend /frontend/dist ./frontend/dist

# copy the rust executable into the working directory
COPY --from=backend /app/target/release/rust_flight_tracker .


CMD [ "./rust_flight_tracker" ]