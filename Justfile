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


run:
    docker compose up -d database
    cargo watch -x run
