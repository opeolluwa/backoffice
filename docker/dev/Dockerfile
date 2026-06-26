FROM rust:1.90.0-slim-bullseye

ENV CARGO_TARGET_DIR=/cargo-target


RUN set -e && \
    apt-get update -q && \
    apt-get install -yq \
    libgcc-s1 \
    libssl-dev \
    pkg-config \
    clang \
    libclang-dev \
    zlib1g-dev \
    llvm-dev \
    build-essential && \
    rm -rf /var/lib/apt/lists/*

    
RUN cargo install cargo-watch

WORKDIR /app

COPY ./backoffice .

RUN cargo build

VOLUME ["/app", "/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", \
     "--why", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-w", "migration", \
     "-x", "check", \
     "-x", "run"]
