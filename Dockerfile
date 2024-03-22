# Builds a Container Image with intelligent caching using cargo chef and multi stage build.
# Image needs to receive the config file to use via a mount on container instantiation.
# Example: docker run -v {/path/to/prod.yml}:/app/prod.yml pastr:latest

# Start with the cargo chef base image
FROM lukemathwalker/cargo-chef:latest-rust-1.76.0 AS chef
WORKDIR /app
RUN apt update && apt install -y lld clang

# Create a recipe file from the project
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build Project Dependencies so they are being cached
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build the actual application
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin pastr

# Setup runtime on debian and start the app
FROM debian:bookworm-slim AS rt
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pastr pastr
ENV APP_ENV prod
ENTRYPOINT ["./pastr"]

