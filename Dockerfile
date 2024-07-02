FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build

EXPOSE 8095

CMD ["./target/debug/ytt-fe-mock"]