# Use a Rust base image for building
FROM rust:1.85 AS builder

# Create a working directory
WORKDIR /app

# Copy only the dependency files first to leverage Docker cache
COPY api-gateway/Cargo.toml api-gateway/Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this will be cached unless Cargo files change)
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Now copy the actual source code
COPY api-gateway/src ./src

# Build the application with release optimizations
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

# Use a smaller image for the runtime
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api-gateway ./api-gateway

# Set environment variables
ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 8500

# Run the binary
CMD ["./api-gateway"]