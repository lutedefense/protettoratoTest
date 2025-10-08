FROM rust:1.78-slim
WORKDIR /app
COPY . .
RUN cargo build --release --bin protettorato-backend
CMD ["./target/release/protettorato-backend"]
