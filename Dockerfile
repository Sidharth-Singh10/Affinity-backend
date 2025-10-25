FROM rust:bookworm AS builder

ARG FEATURES=""

RUN apt-get update && apt-get install -y \
    pkg-config \
    clang \
    lld \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY entity ./entity
COPY migration ./migration
COPY .env ./

RUN if [ -z "$FEATURES" ]; then \
      cargo build --release; \
    else \
      cargo build --release --features "$FEATURES"; \
    fi

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    docker.io \
    jq \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV DOCKER_HOST=tcp://docker:2376
ENV DOCKER_TLS_VERIFY=1
ENV DOCKER_CERT_PATH=/certs/client
ENV APP_ENVIRONMENT=production

COPY --from=builder /app/target/release/rusty_backend /app/rusty_backend

ENTRYPOINT ["./rusty_backend"]
