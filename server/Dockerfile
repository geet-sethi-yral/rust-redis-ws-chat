# Build stage
FROM rust:1.84 AS builder

WORKDIR /app

# Copy the Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
# RUN mkdir -p src && \
#   echo "fn main() {}" > src/main.rs && \
#   cargo build --release && \
#   rm -rf src

# Copy the real source code
COPY . .

# Build the application with static linking to reduce dependencies
RUN cargo build --release

# Use the same base image for runtime to ensure GLIBC compatibility
FROM rust:1.84-slim

WORKDIR /app

# Install only the necessary runtime dependencies
RUN apt-get update && \
  apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
  rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/server /app/server

# Expose the WebSocket port
EXPOSE 4000

# Set the environment variable for Redis (override with runtime values)
ENV REDIS_URL=redis://redis:6379

CMD ["/app/server"]