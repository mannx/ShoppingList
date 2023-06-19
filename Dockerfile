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

FROM alpine

RUN apk update

WORKDIR /

COPY --from=build /src/target/release/backend /backend
COPY --from=build /src/dist /dist

EXPOSE 8080
