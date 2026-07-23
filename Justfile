alias w := watch
alias k := kill
alias b := build
alias l := logs
alias install := install-dependencies
alias run-fe := run-frontend
alias r := restart
alias cfg := copy-env
alias start := watch
alias dev := watch

set dotenv-required := false
set dotenv-load := true
set dotenv-path := "./backoffice/.env"
set export := true

FRONTEND_DIR := 'backoffice_web'
DOCKER_CMD := "docker compose -f docker-compose.yaml"
STAGING_CMD := "docker compose -f docker-compose.staging.yaml"
PROD_CMD := "docker compose -f docker-compose.prod.yaml"
DEV_DB_URL := "postgres://backoffice:backoffice@localhost:6543/backoffice"

# ──────────────────────────────────────────────
# Default
# ──────────────────────────────────────────────

@default:
    @just --list --list-heading $'Available commands\n'

# ──────────────────────────────────────────────
# Setup
# ──────────────────────────────────────────────

[doc('Copy example env files to their respective .env files')]
@copy-env:
    cp .env.example .env
    cp .env.staging.example .env.staging
    cp .env.prod.example .env.prod || true
    cp .env.example ./backoffice/.env
    cp .env.staging.example ./backoffice/.env.staging
    cp .env.prod.example ./backoffice/.env.prod || true

[doc('Install all application dependencies (backend + frontend)')]
@install-dependencies:
    @echo "Installing frontend dependencies..."
    {{ if os_family() == "windows" { "Set-Location frontend; if ($?) { npm install }" } else { "cd frontend && npm install" } }}
    @echo "Done. Copy .env.example to .env.local if you haven't already."

# ──────────────────────────────────────────────
# Development
# ──────────────────────────────────────────────

[doc('Start dev environment and follow logs')]
@watch:
    {{ DOCKER_CMD }} up -d
    @just l

[doc('Start dev environment in background')]
@run:
    {{ DOCKER_CMD }} up -d
    {{ DOCKER_CMD }} logs -f --tail='30' app

[doc('Follow dev app logs')]
@logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app

[doc('Tear down dev environment and volumes')]
@kill:
    {{ DOCKER_CMD }} down -v

[doc('Restart dev environment')]
@restart:
    @just kill
    @just watch

# ──────────────────────────────────────────────
# Staging
# ──────────────────────────────────────────────

[doc('Build staging Docker image')]
staging-build:
    docker build -t backoffice-staging -f docker/staging/Dockerfile .

[doc('Start staging infrastructure services')]
staging-up:
    {{ STAGING_CMD }} up -d

[doc('Run staging app container')]
staging-run:
    just staging-up
    docker rm -f backoffice-staging 2>/dev/null || true
    docker run -d --name backoffice-staging \
        --network backoffice_staging \
        --env-file backoffice/.env.staging \
        -p 8000:8000 \
        --restart unless-stopped \
        backoffice-staging

[doc('Stop staging environment')]
staging-down:
    docker stop backoffice-staging && docker rm backoffice-staging || true
    {{ STAGING_CMD }} down

[doc('View staging logs')]
staging-logs:
    docker logs -f --tail='50' backoffice-staging &
    {{ STAGING_CMD }} logs -f --tail='50'

# ──────────────────────────────────────────────
# Production
# ──────────────────────────────────────────────

[doc('Build backend binary in release mode')]
build:
    cargo build --release

[doc('Build production Docker image')]
prod-build:
    docker build -t backoffice-prod -f docker/prod/Dockerfile .

[doc('Start production infrastructure services')]
prod-up:
    {{ PROD_CMD }} up -d

[doc('Run production app container')]
prod-run:
    just prod-up
    docker rm -f backoffice-prod 2>/dev/null || true
    docker run -d --name backoffice-prod \
        --network backoffice_prod \
        --env-file backoffice/.env.test \
        -p 8000:8000 \
        --restart unless-stopped \
        --cpus 2 --memory 1g \
        backoffice-prod

[doc('Stop production environment')]
prod-down:
    docker stop backoffice-prod && docker rm backoffice-prod || true
    {{ PROD_CMD }} down

[doc('View production logs')]
prod-logs:
    docker logs -f --tail='50' backoffice-prod &
    {{ PROD_CMD }} logs -f --tail='50'

# ──────────────────────────────────────────────
# Frontend
# ──────────────────────────────────────────────

[doc('Run frontend dev server')]
[working-directory: 'backoffice_web']
@run-frontend:
    npm run dev

[doc('Build frontend and copy assets to backend')]
[working-directory: 'backoffice_web']
@build-frontend:
    npm run generate
    {{ if os_family() == "windows" { "Remove-Item -Recurse -Force ../assets" } else { "rm -rf ../assets" } }}
    {{ if os_family() == "windows" { "Copy-Item -Recurse .output/public ../assets" } else { "cp -r .output/public ../assets" } }}

# ──────────────────────────────────────────────
# Database
# ──────────────────────────────────────────────

[doc('Run SQLx migrations and prepare queries')]
db:
    sqlx migrate run
    cargo sqlx prepare -- --bin cli

[doc('Add a new SeaORM migration')]
[working-directory: 'backoffice']
migrate-add target:
    sea-orm-cli migrate generate {{target}}

[doc('Run SeaORM migrations against dev DB')]
[working-directory: 'backoffice']
@run-migrations:
    sea-orm-cli migrate up --database-url {{DEV_DB_URL}}

[doc('Regenerate SeaORM entities from dev DB')]
@generate-entities:
    RUST_BACKTRACE=full sea-orm-cli generate entity \
        --database-url {{DEV_DB_URL}} \
        --with-serde both \
        --enum-extra-derives 'ts_rs::TS' \
        --model-extra-attributes 'serde(rename_all="camelCase")' \
        --model-extra-attributes 'backoffice_macros::ts_rs_export_sea_orm_entity_name' \
        --enum-extra-attributes 'ts(export)' \
        --ignore-tables backoffice_server_migrations \
        -o backoffice/crates/backoffice-domain/src/models --seaography

[doc('Pull DB schema, regenerate entities, and export bindings')]
db-pull:
    just run-migrations
    just generate-entities
    just export-bindings
    node scripts/ts-export.js

[doc('Export TypeScript bindings from Rust tests')]
[working-directory: 'backoffice']
export-bindings:
    cargo test --workspace

# ──────────────────────────────────────────────
# CLI
# ──────────────────────────────────────────────

[doc('Create a new admin user via CLI')]
[working-directory: 'backoffice']
run-cli:
    DATABASE_URL={{DEV_DB_URL}} cargo run --bin cli create-user

[doc('Initialize the application via CLI')]
[working-directory: 'backoffice']
run-init:
    cargo run --bin cli init

# ──────────────────────────────────────────────
# Code Quality
# ──────────────────────────────────────────────

[doc('Lint frontend and format backend code')]
lint:
    {{ if os_family() == "windows" { "Set-Location " + FRONTEND_DIR + "; if ($?) { npm run lint }" } else { "cd " + FRONTEND_DIR + " && npm run lint" } }}
    cargo sort -w
    cargo group-imports --fix
    cargo fmt

[doc('Format and auto-fix Rust code')]
@fmt:
    {{ if os_family() == "windows" { "cargo fmt; if ($?) { cargo fix }" } else { "cargo fmt && cargo fix" } }}

[doc('Run backend test suite')]
test:
    cargo test
