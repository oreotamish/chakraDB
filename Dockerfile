FROM rust:alpine

COPY Cargo.toml Cargo.lock /app/

WORKDIR /app

RUN cargo install

COPY src /app/src

RUN cargo build --release

WORKDIR /app/target/release

EXPOSE 8080

CMD ["./chakra_db"]
