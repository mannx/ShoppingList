# Build Stage
FROM rust:latest

WORKDIR /usr/src/sl

COPY backend/ common/ frontend/ ./

# Common
RUN cargo install --path . common
