FROM rust:1.75-slim as builder
WORKDIR /app
COPY ./backend/Cargo.toml ./backend/Cargo.toml
COPY ./backend/src ./backend/src
RUN cd backend && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/backend/target/release/protettorato-backend .
EXPOSE 8080
CMD ["./protettorato-backend"]
