# Use a Rust base image for building
FROM rust:1.85 AS builder

# Create a new empty shell project
WORKDIR /app

# Copy your manifests and source code
COPY api-gateway /app/api-gateway

# Build the application
WORKDIR /app/api-gateway
COPY api-gateway/Cargo.toml  /app/api-gateway
COPY api-gateway/Cargo.lock  /app/api-gateway

# Build your application with release optimizations
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/app/target \
#     cargo build --release
RUN cargo build --release
# Use a smaller image for the runtime
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
# Install runtime dependencies

# Set work directory
WORKDIR /app
# Copy the binary from the builder stage
COPY --from=builder /app/api-gateway/target/release/api-gateway app/api-gateway

# Create a non-root user for security
RUN useradd -m appuser

USER appuser

# Set environment variables
ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 8500

# Run the binary
CMD ["api-gateway"]