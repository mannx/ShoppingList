# Build Stage
FROM rust:latest

WORKDIR /src

# Copy source files over
COPY backend/ ./backend
COPY common ./common
COPY frontend ./frontend
COPY Cargo* ./

# Setup build environment
#   Wasm Support
RUN rustup target add wasm32-unknown-unknown

#   Trunk for frontend building
RUN cargo install trunk
