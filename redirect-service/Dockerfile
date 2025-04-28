# Use Rust official image for building
FROM rust:1.85 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y libpq-dev pkg-config openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy the common module and the redirect service
ADD common /app/common
COPY redirect-service /app/redirect-service

# Build the application
WORKDIR /app/redirect-service
COPY redirect-service/Cargo.toml  /app/redirect-service
COPY redirect-service/Cargo.lock  /app/redirect-service
RUN cargo check
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim  

# Install runtime dependencies
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/redirect-service/target/release/redirect-service /app/redirect-service

# Expose port
EXPOSE 8081

# Command to run the service
CMD ["/app/redirect-service"]
