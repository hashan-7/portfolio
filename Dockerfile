FROM node:22-bookworm-slim AS frontend-builder

WORKDIR /app/frontend

COPY frontend/package*.json ./

RUN if [ -f package-lock.json ]; then npm ci; else npm install; fi

COPY frontend/ ./

RUN npm run build

FROM rust:1.95-bookworm AS backend-builder

WORKDIR /app

COPY . .

RUN cargo build --release --locked -p backend

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/release/backend /app/backend
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

RUN useradd -m -u 1000 user

USER user

ENV PORT=7860

EXPOSE 7860

CMD ["./backend"]