FROM rust:1.75-slim
WORKDIR /app
COPY . .
RUN cargo build --release --manifest-path ./backend/Cargo.toml
CMD ["./backend/target/release/protettorato-backend"]
