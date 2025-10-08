FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN ls -la backend/
RUN cargo build --release --manifest-path backend/Cargo.toml

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/backend/target/release/protettorato-backend .
EXPOSE 8080
CMD ["./protettorato-backend"]
