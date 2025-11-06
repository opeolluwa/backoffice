FROM node:24 AS frontend
WORKDIR /app

COPY ./ui/package*.json ./
RUN npm ci
COPY ./ui .
RUN npm run build


ARG RUST_VERSION=1.90.0
ARG APP_NAME=backoffice

FROM rust:1.90.0 AS backend
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y musl-dev
COPY . .


RUN cargo build --release

FROM ubuntu AS final

COPY --from=frontend /app/.output/public ./public
COPY --from=backend /app/target/release/backoffice ./backoffice
COPY --from=backend /app/migrations ./migrations
COPY --from=backend /app/scripts ./scripts

EXPOSE 50051

CMD ["./backoffice"]
