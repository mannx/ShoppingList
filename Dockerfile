#syntax:docker/dockerfile:1

#
# Build Stage
#

FROM rust:latest AS build

WORKDIR /src

# Copy source files over
COPY . .

# Setup build environment
#   Wasm Support
RUN rustup target add wasm32-unknown-unknown

#   Build backend
WORKDIR /src/
ENV SQLX_OFFLINE true
RUN cargo build -p backend --release

#   Build frontend
#       Outputs to /src/dist

#   Install trunk to build frontend
RUN cargo install trunk

WORKDIR /src/frontend
RUN trunk build --release

#
# Deploy Stage
#

# FROM alpine

# RUN apk update

# FROM ubuntu

# RUN apt update && apt upgrade -y
FROM debian:bullseye

# RUN apt-get update && apt-get install -y libssl1.1 && apt clean && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && apt-get install -y libssl1.1 

WORKDIR /

COPY --from=build /src/target/release/backend /backend
COPY --from=build /src/dist /dist

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "-l"]
# ENTRYPOINT ["/backend"]
