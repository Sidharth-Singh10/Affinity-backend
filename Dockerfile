FROM rust

WORKDIR /rust

COPY . .

RUN cargo install --path .

CMD ["rusty_backend"]
