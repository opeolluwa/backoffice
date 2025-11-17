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
set dotenv-path := "./.env.local"
set export :=  true

@default: 
    @just --list --list-heading $'Available commands\n'



[working-directory :'ui']
@run-ui:
    npm run dev



[working-directory :'ui']
@build-ui:
    npm run generate

    cp  -r .output/public ../assets


run:
    @just build-ui
    docker compose up -d database
    cargo watch -x run


lint:
    cd ui && npm run lint 
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