

[doc('Install all application dependencies (backend + frontend)')]
@install-dependencies:
    @echo "Installing frontend dependencies..."
    cd frontend && npm install
    @echo "Done. Copy .env.example to .env.local if you haven't already."


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
