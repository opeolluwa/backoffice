FROM rust:1.94.0-slim-bullseye

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

RUN cargo install cargo-watch

WORKDIR /app

COPY backoffice/ ./backoffice/
COPY backoffice_macros/ ./backoffice_macros/

WORKDIR /app/backoffice

RUN cargo build

VOLUME ["/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", \
     "--why", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-w", "migration", \
     "-x", "check", \
     "-x", "run"]