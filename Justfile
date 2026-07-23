alias w := watch
alias k := kill
alias b := build
alias l := logs
alias install := install-dependencies
alias run-fe := run-frontend
alias r := restart
alias cfg := copy-env
alias start  := watch 
alias dev := watch 

set dotenv-required := false
set dotenv-load := true
set dotenv-path := "./backoffice/.env"
set export := true

FRONTEND_DIR := 'backoffice_web'
DOCKER_CMD := "docker compose -f docker-compose.yaml"
DEV_DB_URL := "postgres://backoffice:backoffice@localhost:6543/backoffice"

@default:
    @just --list --list-heading $'Available commands\n'


[doc('Install all application dependencies (backend + frontend)')]
@install-dependencies:
    @echo "Installing frontend dependencies..."
    {{ if os_family() == "windows" { "Set-Location frontend; if ($?) { npm install }" } else { "cd frontend && npm install" } }}
    @echo "Done. Copy .env.example to .env.local if you haven't already."


@fmt:
    {{ if os_family() == "windows" { "cargo fmt; if ($?) { cargo fix }" } else { "cargo fmt && cargo fix" } }}


@watch:
    {{ DOCKER_CMD }} up -d
    @just l

    

@logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app


build:
    cargo run build --release


@kill:
    {{ DOCKER_CMD }} down -v


@restart:
    @just kill
    @just watch


@copy-env:
    cp .env.example .env    
    cp .env.example ./backoffice/.env



[working-directory: 'backoffice_web']
@run-frontend:
    npm run dev


[working-directory: 'backoffice_web']
@build-frontend:
    npm run generate
    {{ if os_family() == "windows" { "Remove-Item -Recurse -Force ../assets" } else { "rm -rf ../assets" } }}
    {{ if os_family() == "windows" { "Copy-Item -Recurse .output/public ../assets" } else { "cp -r .output/public ../assets" } }}


run:
    {{ DOCKER_CMD }} up -d
    {{ DOCKER_CMD }} logs -f --tail='30' app



lint:
    {{ if os_family() == "windows" { "Set-Location " + FRONTEND_DIR + "; if ($?) { npm run lint }" } else { "cd " + FRONTEND_DIR + " && npm run lint" } }}
    cargo sort -w
    cargo group-imports --fix
    cargo fmt

test:
    cargo test


db:
    sqlx migrate run
    cargo sqlx prepare -- --bin cli

    
[working-directory: 'backoffice']
run-cli:
    DATABASE_URL={{DEV_DB_URL}} cargo run --bin cli create-user

run-init:
    cargo run --bin cli init

[working-directory: 'backoffice']
migrate-add target:
    sea-orm-cli migrate generate {{target}}

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



[working-directory: 'backoffice']
@run-migrations:
    sea-orm-cli migrate up --database-url {{DEV_DB_URL}}

[working-directory: 'backoffice']
export-bindings:
    cargo test --workspace

db-pull:
    just run-migrations
    just generate-entities
    just export-bindings
    node scripts/ts-export.js

# Production
PROD_CMD := "docker compose -f docker-compose.prod.yaml"

[doc('Build production Docker image')]
prod-build:
    docker build -t backoffice-prod -f docker/prod/Dockerfile .

[doc('Run infrastructure services')]
prod-up:
    {{ PROD_CMD }} up -d

[doc('Run app container')]
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

[doc('Stop everything')]
prod-down:
    docker stop backoffice-prod && docker rm backoffice-prod || true
    {{ PROD_CMD }} down

[doc('View production logs')]
prod-logs:
    docker logs -f --tail='50' backoffice-prod &
    {{ PROD_CMD }} logs -f --tail='50'


