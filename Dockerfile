FROM rust

# Set environment variables to use Docker in Docker
ENV DOCKER_HOST=tcp://docker:2376
ENV DOCKER_TLS_VERIFY=1
ENV DOCKER_CERT_PATH=/certs/client

WORKDIR /rust

COPY . .

# Install dependencies, Docker CLI, and jq
RUN apt-get update && apt-get install -y \
    docker.io \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install Rust application
RUN cargo install --path .

# CMD to start the application
CMD ["rusty_backend"]
