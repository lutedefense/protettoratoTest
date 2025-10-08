FROM rust:1.78-slim as builder

WORKDIR /build

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy everything from backend directory
COPY . .

# Build for release
RUN cargo build --release --bin protettorato-backend

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /build/target/release/protettorato-backend /app/protettorato-backend

# Expose port
EXPOSE 8080

# Set environment variable for Railway
ENV PORT=8080

# Run the binary
CMD ["/app/protettorato-backend"]
