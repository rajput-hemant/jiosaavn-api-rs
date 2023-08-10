FROM rust:1.55 as build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=build /app/target/release/jiosaavn .

EXPOSE 8080

CMD ["./jiosaavn"]
