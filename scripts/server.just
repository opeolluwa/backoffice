

[working-directory:'.']
test:
    DATABASE_URL=postgres://${DATABASE_USER}:${DATABASE_PASSWORD}@${DATABASE_HOST}:${EXTERNAL_DATABASE_PORT}/${DATABASE_NAME} cargo test


[doc('Install the application dependencies')]
@install-dependencies:
    @echo "Installing backend dependencies"


@fmt:
    cargo fmt && cargo fix

@watch:
    docker compose up -d
    @just l

@logs:
    docker compose logs -f --tail='30' app

build:
    cargo run build --release

@kill:
    docker compose down -v

@restart:
    @just kill
    @just watch
