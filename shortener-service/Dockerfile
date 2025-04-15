# Use Rust official image for building
FROM rust:1.85 AS builder

# Install dependencies (use apt for Debian-based Rust images)
RUN apt-get update && apt-get install -y libpq-dev pkg-config openssl libssl-dev && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app
# Copy the common module and the shortener service
ADD common /app/common
COPY shortener-service /app/shortener-service

# Build the application
WORKDIR /app/shortener-service
COPY shortener-service/Cargo.toml  /app/shortener-service
COPY shortener-service/Cargo.lock  /app/shortener-service
RUN cargo build --release


# Use a minimal runtime image
FROM debian:bookworm-slim  

# Install runtime dependencies
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/shortener-service/target/release/shortener-service /app/shortener-service

# Expose port
EXPOSE 8080

# Command to run the service
CMD ["/app/shortener-service"]