# Stage 1: Frontend build
FROM node:22-slim AS frontend
WORKDIR /app
COPY backoffice_web/package*.json ./
RUN npm install
COPY backoffice_web/ ./
RUN npm run generate

# Stage 2: Rust build
FROM rust:1.94.0-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev clang libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY backoffice_macros/ ./backoffice_macros/
COPY backoffice/ ./backoffice/
WORKDIR /app/backoffice
RUN cargo build --release \
    && strip target/release/backoffice

# Stage 3: Final image
FROM debian:bookworm-slim

RUN groupadd -r app && useradd -r -g app -d /app -s /sbin/nologin app \
    && apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/backoffice/target/release/backoffice ./backoffice
COPY --from=frontend /app/.output/public ./dist

RUN chown -R app:app /app

USER app

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/api/health || exit 1

CMD ["./backoffice"]
