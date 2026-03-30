# Backoffice

A content management system and administrative platform for web applications. Built with a Rust backend and a Nuxt frontend.

## Technologies

### Backend
- **Rust** 1.93.1 — core application language
- **Axum** 0.8 — HTTP framework
- **SQLx** 0.8 — async PostgreSQL driver with compile-time query checking
- **Tokio** 1 — async runtime
- **JWT** (jsonwebtoken) — authentication
- **bcrypt** — password hashing
- **Clap** — CLI tooling for admin commands

### Frontend
- **Node.js** v24.7.0
- **Nuxt** 4 (Vue 3) — full-stack framework
- **Nuxt UI** 4 — component library
- **Pinia** — state management
- **TailwindCSS** 4 — styling
- **TypeScript** — type safety
- **Zod / Valibot** — schema validation
- **TanStack Table** — data tables

### Infrastructure
- **PostgreSQL** 15 — primary database
- **Docker / Docker Compose** — containerized services
- **just** — task runner

## Prerequisites

- [Rust](https://rustup.rs/) 1.93.1+
- [Node.js](https://nodejs.org/) v24.7.0+
- [Docker](https://www.docker.com/) with Docker Compose
- [just](https://just.systems/) task runner
- [sqlx-cli](https://github.com/launchbadge/sqlx) (`cargo install sqlx-cli`)
- [cargo-watch](https://crates.io/crates/cargo-watch) (`cargo install cargo-watch`)

## Setup

### 1. Clone the repository

```sh
git clone <repo-url>
cd backoffice
```

### 2. Configure environment

```sh
cp .env.example .env.local
```

Edit `.env.local` and fill in the required values (at minimum `JWT_SIGNING_KEY`, and any external service keys you need).

### 3. Install dependencies

```sh
just install
```

This installs both Cargo and frontend npm dependencies.

### 4. Start the database

```sh
docker compose up -d database
```

### 5. Run migrations and prepare SQLx

```sh
just db
```

### 6. Seed initial data (first run only)

```sh
just run-init
```

This creates the super-admin user defined in your `.env.local`.

### 7. Start the development server

**Backend + database:**
```sh
just run
```

**Frontend (separate terminal):**
```sh
just run-frontend
```

The API listens on the port set by `PORT` (default `5006`) and the frontend on `http://localhost:7000`.

## Common Commands

| Command | Description |
|---|---|
| `just run` | Start backend with hot-reload |
| `just run-frontend` | Start Nuxt dev server |
| `just watch` | Start full stack via Docker Compose |
| `just build-frontend` | Build and export frontend static assets |
| `just db` | Run migrations + prepare SQLx |
| `just test` | Run backend test suite |
| `just lint` | Lint frontend and format Rust code |
| `just logs` | Tail Docker Compose app logs |
| `just kill` | Stop and remove Docker containers |
| `just restart` | Kill then watch |
| `just run-cli` | Run CLI — create user |
| `just run-init` | Run CLI — initialize super admin |

Run `just` with no arguments to see all available commands.
