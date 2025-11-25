ARG APP_NAME=backoffice
ARG UI_DIR=frontend
ARG RUST_VERSION=1.90.0

FROM node:24 AS frontend
WORKDIR /app

COPY ./frontend/package*.json ./
RUN npm install

COPY ./frontend .
RUN npm run generate

FROM rust:${RUST_VERSION} AS backend
WORKDIR /app

RUN apt-get update && apt-get install -y musl-dev
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS final
ARG APP_NAME

WORKDIR /app
COPY --from=frontend /app/.output/public ./assets
COPY --from=backend /app/target/release/backoffice ./backoffice
COPY --from=backend /app/target/release/cli ./cli
COPY --from=backend /app/migrations ./migrations



EXPOSE 50051
CMD ["./backoffice"]
