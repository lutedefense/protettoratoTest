FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path backend/Cargo.toml

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/backend/target/release/protettorato-backend /app/
EXPOSE 8080
CMD ["./protettorato-backend"]
