FROM rust:1.75-slim
WORKDIR /app
WORKDIR /app/backend
COPY . .
RUN cargo build --release --manifest-path Cargo.toml
CMD ["/app/backend/target/release/protettorato-backend"]
