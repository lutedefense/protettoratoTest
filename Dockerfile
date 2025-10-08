FROM rust:1.75-slim
WORKDIR /app
COPY ./backend ./backend
WORKDIR /app/backend
RUN cargo build --release
CMD ["/app/backend/target/release/protettorato-backend"]
