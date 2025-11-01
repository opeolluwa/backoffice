FROM rust:1.90.0-slim-bullseye

ENV CARGO_TARGET_DIR=/cargo-target

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

RUN cargo build

VOLUME ["/app", "/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", \
     "--why", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-w", "migrations", \
     "-x", "check", \
     "-x", "run"]
