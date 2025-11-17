# Game web configuration

The web crate now expects its runtime settings to be delivered through a TOML file.
The file is passed via the `--config-path` CLI flag (default: `config/game-web.toml`).

## File structure

```toml
[server]
# TCP interface where the HTTP server should listen.
host = "0.0.0.0"
# TCP port exposed by the server process.
port = 8080
# Size of the async worker pool that accepts requests.
workers = 8
# Upper bound (in seconds) for handling a single HTTP request.
request_timeout_secs = 60

[database]
# PostgreSQL connection string used by `game-core` services.
url = "postgres://vallheru:vallheru@localhost:5432/vallheru"
# Size of the shared connection pool.
pool_size = 16
# Timeout for establishing new connections.
connect_timeout_secs = 10

[templates]
# Absolute or relative path pointing to PHP/Tera templates.
template_root = "../templates"
# Enables hot reloads during development.
hot_reload = true

[features]
# Feature toggles controlling UX experiments.
enable_registration = true
enable_world_map = false
```

An example file is tracked as `game-web/example-config.toml`. Copy it to the
path referenced by `--config-path` and tweak the values to match your
environment.
