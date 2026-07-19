# Backoffice

A content management system and administrative platform for web applications.

## Stack

- [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum) — backend API
- [SQLx](https://github.com/launchbadge/sqlx) + [SeaORM](https://www.sea-orm.dev/) — PostgreSQL
- [Nuxt 4](https://nuxt.com/) (Vue 3) — frontend
- [Tailwind CSS](https://tailwindcss.com/) + [Nuxt UI](https://ui.nuxt.com/) — styling
- [Docker Compose](https://docs.docker.com/compose/) — infrastructure
- [just](https://just.systems/) — task runner

## Quick Start

```sh
git clone <repo-url> && cd backoffice
cp .env.example .env && cp .env.example backoffice/.env
just watch
just run-frontend   # separate terminal
```

## Commands

| Command               | Description               |
| --------------------- | ------------------------- |
| `just run`            | Backend with hot-reload   |
| `just run-frontend`   | Nuxt dev server           |
| `just watch`          | Full stack via Docker     |
| `just build-frontend` | Build & export frontend   |
| `just db`             | Migrations + SQLx prepare |
| `just test`           | Run tests                 |
| `just lint`           | Lint & format             |
| `just kill`           | Stop containers           |
| `just run-init`       | Seed super admin          |

Run `just` for all available commands.

## License

[Apache 2.0](LICENSE)
