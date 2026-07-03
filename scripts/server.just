

[doc('Install all application dependencies (backend + frontend)')]
@install-dependencies:
    @echo "Installing frontend dependencies..."
    {{ if os_family() == "windows" { "Set-Location frontend; if ($?) { npm install }" } else { "cd frontend && npm install" } }}
    @echo "Done. Copy .env.example to .env.local if you haven't already."


@fmt:
    {{ if os_family() == "windows" { "cargo fmt; if ($?) { cargo fix }" } else { "cargo fmt && cargo fix" } }}

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
