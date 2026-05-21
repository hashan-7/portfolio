FROM rust:1.95-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --locked -p backend

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

RUN useradd -m -u 1000 user

USER user

ENV PORT=7860

EXPOSE 7860

CMD ["./backend"]