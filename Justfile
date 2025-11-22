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

FRONTEND_DIR:='frontend'
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
    # @just build-ui
    docker compose up -d database
    cargo watch -x run


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