FROM rust:1.70 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/tokio_hello*
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/tokio_hello /usr/local/bin/

CMD ["tokio_hello"]