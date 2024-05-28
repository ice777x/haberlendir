FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD cargo run