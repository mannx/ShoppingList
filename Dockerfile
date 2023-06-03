# Build Stage
FROM rust:latest

WORKDIR /src

# Copy source files over
# COPY backend/ ./backend
# COPY common ./common
# COPY frontend ./frontend
# COPY Cargo* ./
COPY . .

# Setup build environment
#   Wasm Support
RUN rustup target add wasm32-unknown-unknown

#   Trunk for frontend building
RUN cargo install trunk

#   Build frontend
#       Outputs to /src/dist
WORKDIR /src/frontend

RUN trunk build

#   Build backend
WORKDIR /src/backend
RUN cargo build --release
