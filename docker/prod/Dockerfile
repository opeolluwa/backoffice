# Stage 1: Frontend build
FROM node:22-slim AS frontend-build
WORKDIR /app/frontend
COPY backoffice_web/package*.json ./
RUN npm ci
COPY backoffice_web/ ./
RUN npm run generate

# Stage 2: Backend build
FROM rust:1.94.0-slim-bullseye AS backend-build

ENV CARGO_TARGET_DIR=/cargo-target

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libgcc-s1 \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        zlib1g-dev \
        llvm-dev \
        build-essential; \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY backoffice/ ./backoffice/
COPY backoffice_macros/ ./backoffice_macros/

WORKDIR /app/backoffice

RUN cargo build --release

# Stage 3: Final image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-build /cargo-target/release/backoffice /app/backoffice
COPY --from=frontend-build /app/frontend/dist /app/dist

WORKDIR /app

EXPOSE 50051

CMD ["./backoffice"]
