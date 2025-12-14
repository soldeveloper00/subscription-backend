FROM rust:1.75-slim

WORKDIR /app

# Pehle OpenSSL aur dependencies install karo
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

# Build karo
RUN cargo build --release

CMD ["./target/release/trading-signals-backend"]
