import 'scripts/build.just'
import 'scripts/package.just'
import 'scripts/server.just'
import 'scripts/lint.just'


alias w:= watch
alias k:= kill
alias b:= build
alias l:= logs
alias install := install-dependencies
alias r:= restart


set dotenv-required
set dotenv-load := true
set dotenv-path := "./.env"
set export :=  true

FRONTEND_DIR:='frontend'
DOCKER_CMD := "docker compose -f docker-compose.yaml"
DEV_DB_URL:="postgres://backoffice:backoffice@localhost:5400/backoffice"

@default: 
    @just --list --list-heading $'Available commands\n'



[working-directory :'frontend']
@run-frontend:
    npm run dev


[working-directory :'frontend']
@build-frontend:
    npm run generate
    rm -rf ../assets
    cp  -r .output/public ../assets


run:
    {{ DOCKER_CMD }} up -d 
    {{ DOCKER_CMD }} logs -f --tail='30' app



lint:
    cd {{FRONTEND_DIR}} && npm run lint 
    cargo sort -w 
    cargo group-imports --fix
    cargo fmt 

test:
    cargo test


db:
    sqlx migrate run 
    cargo sqlx prepare -- --bin cli

run-cli:
    cargo run --bin cli create-user 

run-init:
    cargo run --bin cli init


migrate-add target:
    sea-orm-cli migrate generate {{target}}

@generate-entities:
	sea-orm-cli generate entity \
		--database-url {{DEV_DB_URL}} \
		--with-serde both \
		--model-extra-attributes 'serde(rename_all="camelCase")' \
		-o src/entities --seaography



db-pull:
    just generate-entities
