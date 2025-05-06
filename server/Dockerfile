FROM rust:1.85

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/server"]
