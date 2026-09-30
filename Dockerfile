# syntax=docker/dockerfile:1.7

FROM node:22-bookworm-slim AS frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./
RUN --mount=type=cache,target=/root/.npm npm ci --no-audit --no-fund

COPY frontend/ ./
RUN npm run build

FROM rust:1.95.0-bookworm AS backend-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml

RUN mkdir -p backend/src \
    && printf 'fn main() {}\n' > backend/src/main.rs
RUN --mount=type=cache,id=portfolio-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=portfolio-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=portfolio-cargo-target,target=/app/target \
    cargo build --release --locked -p backend

RUN rm -rf backend/src
COPY backend/src backend/src
RUN --mount=type=cache,id=portfolio-cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=portfolio-cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=portfolio-cargo-target,target=/app/target \
    touch backend/src/main.rs \
    && cargo build --release --locked -p backend \
    && cp target/release/backend /app/portfolio-backend

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN groupadd --gid 1000 portfolio \
    && useradd --uid 1000 --gid portfolio --create-home portfolio \
    && install -d -o portfolio -g portfolio \
        /app/data/profile \
        /app/data/assets/projects/images \
        /app/data/assets/projects/videos \
        /app/data/.upload-staging

COPY --from=backend-builder --chown=portfolio:portfolio --chmod=0555 \
    /app/portfolio-backend /app/backend
COPY --from=frontend-builder --chown=portfolio:portfolio \
    /app/frontend/dist /app/frontend/dist

USER portfolio:portfolio

ENV PORT=7860 \
    RUST_LOG=backend=info,tower_http=info

EXPOSE 7860
STOPSIGNAL SIGTERM

CMD ["./backend"]
