# Vallheru RS Setup

The Rust rewrite is executed directly on the host, while supporting services run in Docker containers.

## Prerequisites

- Docker with the Compose plugin for the database and tooling dependencies.
- A Rust toolchain (via `rustup`) available on the host for building and running the application.

## Start the dependencies

```bash
cd vallheru-rs
docker compose up -d
```

This command boots a PostgreSQL 18 database and an Adminer instance listening on port `8080`. When you finish working, stop the containers with:

```bash
docker compose down
```

## Build and run the application on the host

```bash
cd vallheru-rs
cargo build
cargo run
```

Adjust the `cargo run` arguments to point to the proper binary if the workspace exposes more than one entry point.

## Helpful development commands

- Format the workspace before committing:
  ```bash
  cargo fmt
  ```
- Lint and catch common mistakes:
  ```bash
  cargo clippy --all-targets --all-features
  ```
- Run the automated tests:
  ```bash
  cargo test
  ```
- Rebuild and re-run automatically during development:
  ```bash
  cargo watch -x 'check' -x 'test'
  ```
