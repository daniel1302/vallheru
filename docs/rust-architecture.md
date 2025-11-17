# Rust rewrite architecture

## Application layers

### HTTP layer (web interface)
- Axum routes map HTTP verbs, form submissions, JSON bodies, and WebSocket events to explicit game use cases such as starting a fight, trading, or guild chat.
- The layer stays thin: it validates input, resolves the current player/session, and calls domain services instead of talking to the database directly.
- Extractors and middleware (`tower::Layer`) keep cross-cutting concerns—authentication, rate limiting, localization context—out of the handlers themselves.

### Game-logic layer
- Hosts all domain rules (combat math, crafting, events, rankings) and exposes them through services operating on rich types.
- The code depends only on traits that describe persistence and infrastructure ports, so we can test the domain without HTTP or a real database.
- HTTP models never leak inward, and the domain never imports Axum or MiniJinja symbols, maintaining a clean architecture boundary.

### Data-access layer
- Implements repository traits using SQLx queries and manages migrations for PostgreSQL.
- Responsible for mapping between domain aggregates and relational tables, batching reads/writes, and providing transactional guarantees when needed.
- Includes schema management tooling and any caching/adapters (e.g., Redis) the domain layer requires.

## Technology choices

### Web framework: **Axum**
- Built on Tokio/Tower, so async HTTP, WebSocket, and background work share the same runtime.
- Declarative routing plus extractor traits simplify handler signatures and make it easy to enforce prerequisites (logged-in player, chosen language, etc.).
- Native WebSocket support and tight integration with middleware make it a good fit for Vallheru's chat, combat, and notification flows.

### Template engine: **MiniJinja**
- Its Jinja2-compatible syntax mirrors how the current PHP templates in `templates/` are structured, so designers can port markup with minimal retraining.
- Supports macro inheritance, filters, and custom globals, allowing us to reuse the helpers the PHP version exposes through Smarty while benefiting from Rust's safety and precompilation.
- Plays well with Axum because templates can be rendered ahead of time and streamed as responses without blocking the async runtime.

### Database access: **SQLx (PostgreSQL)**
- SQLx offers async, compile-time checked SQL without introducing an ORM layer, giving us full control over the complex queries that power markets, rankings, and reports.
- PostgreSQL becomes the single supported database engine (we are migrating away from MySQL), unlocking powerful types (JSONB, ARRAY, partial indexes) that map well to Vallheru's dynamic data.
- SQLx migrations keep schema evolution transparent while letting DBAs continue to review raw SQL just like today.

### Internationalization: **Fluent + `fluent-i18n`**
- The current PHP app stores translations in `languages/<lang>/<page>.php` files filled with `define()` calls (for example, `languages/pl/account.php`), meaning every UI text already has a stable key.
- Fluent lets us keep the key-based approach but adds rich formatting, variants, and fallbacks—capabilities we currently emulate by duplicating PHP constants per page.
- The `fluent-i18n` crate wraps `fluent-bundle` with a lighter API than `fluent-templates`, so we can load `.ftl` files once, expose a `t!("key")`-style macro to MiniJinja filters, and share the same bundle with Axum middleware that selects the player’s language from the session.

## Crate layout

1. **`game-core`**
   - Domain models, combat/economy systems, quest logic, and validation.
   - Defines repository/service traits (`PlayersRepository`, `InventoryRepository`, `Clock`, etc.).
   - No dependency on Axum, MiniJinja, SQLx, or localization libraries beyond trait definitions.

2. **`game-db`**
   - SQLx connection management, migrations, query builders, and repository trait implementations targeting PostgreSQL.
   - Encapsulates data mappers plus any cache/queue clients used by persistence.
   - Provides a constructor (e.g., `DbContext`) wiring pools and running migrations at startup.

3. **`game-web`**
   - Axum application (routes, middleware, WebSockets) and MiniJinja template integration.
   - Bridges HTTP handlers to `game-core` services and injects `game-db` repositories plus the Fluent localization bundle.
   - Handles authentication, session storage, CSRF, and exposes REST/WebSocket APIs for clients and admin tooling.

4. **`game-jobs`** *(optional)*
   - Background workers/cron tasks (economy ticks, leaderboard resets, newsletter mailers) running on the same domain logic but without HTTP dependencies.
   - Uses SQLx repositories from `game-db` and can reuse localization bundles for email content.

5. **`game-cli`** *(optional)*
   - Developer/operator utilities for imports, exports, combat simulations, or backfills.
   - Reuses `game-core` + `game-db` to avoid duplicating business rules.

This structure keeps the domain pure, isolates persistence, and ensures the web layer focuses strictly on delivery concerns while sharing building blocks (templates, localization, database pools) across binaries.
