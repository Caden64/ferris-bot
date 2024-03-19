FROM rust:1.76-buster as build


WORKDIR /src

COPY . .

RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM debian:buster as release

WORKDIR /app

COPY --from=BUILD /src/target/x86_64-unknown-linux-gnu/release/ferris-bot .

CMD ["./ferris-bot"]
