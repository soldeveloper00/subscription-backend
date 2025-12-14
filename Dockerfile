# Builder stage
FROM rust:1.86.0-slim AS builder

# Install ONLY essential build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build release and clean cache in same layer
RUN cargo build --release && \
    rm -rf /app/target/release/build/ /app/target/release/deps/ /app/target/release/.fingerprint/ && \
    strip /app/target/release/$(grep 'name =' Cargo.toml | head -1 | cut -d'"' -f2) 2>/dev/null || true

# Runtime stage - SUPER minimal
FROM debian:bookworm-slim

# Install ONLY runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Get binary name from Cargo.toml
ARG BINARY_NAME
RUN if [ -z "$BINARY_NAME" ]; then \
    echo "Building without binary name arg"; \
    else echo "Binary: $BINARY_NAME"; fi

# Copy ONLY the built binary
COPY --from=builder /app/target/release/* /usr/local/bin/

# Use PORT from Railway
ENV PORT=8080
EXPOSE 8080

# Simple startup
CMD ["sh", "-c", "exec /usr/local/bin/$(ls /usr/local/bin/ | head -1)"]
