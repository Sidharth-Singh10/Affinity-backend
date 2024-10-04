# Build stage
FROM rust AS builder
WORKDIR /app

# Install necessary dependencies for building
RUN apt-get update && apt-get install -y \
    lld \
    clang \
    docker.io \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Copy source code and set necessary environment variables
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# Runtime stage
FROM rust AS runtime
WORKDIR /app

# Set Docker environment variables if you're using Docker in Docker
ENV DOCKER_HOST=tcp://docker:2376
ENV DOCKER_TLS_VERIFY=1
ENV DOCKER_CERT_PATH=/certs/client
ENV APP_ENVIRONMENT production

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/rusty_backend /app/rusty_backend

# Set the entry point to run the application
ENTRYPOINT ["./rusty_backend"]
