FROM rust:1.75-slim
WORKDIR /app
COPY . .
RUN cargo build --release --bin protettorato-backend
CMD ["./target/release/protettorato-backend"]
