FROM rust:1.67

WORKDIR /btrade

COPY . .

RUN cargo build --release

CMD ["./target/release/btrade"]



