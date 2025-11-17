# Authentication Migration Plan

## Legacy PHP behavior

### Login and session bootstrap
- `includes/head.php` handles login when the landing page (`$title == 'Wieści'`) receives the login form. It hashes the submitted password with `MD5()` and queries the `players` table for a matching `email`+hash pair before populating `$_SESSION['email']` and `$_SESSION['pass']` with the raw email and MD5 digest.【F:includes/head.php†L135-L218】
- `includes/sessions.php` simply calls `session_start()` and regenerates the PHP session identifier only once per session, so the app relies on PHP's default `PHPSESSID` cookie with no additional flags or rotation.【F:includes/sessions.php†L1-L8】
- Every feature controller includes `includes/head.php`, so the request lifecycle assumes that a valid `$_SESSION['email']`/`$_SESSION['pass']` pair exists once login succeeds, and access checks query the database with those values on each request.【F:docs/architecture.md†L3-L9】【F:logout.php†L32-L88】

### Password lifecycle
- Registration stores the MD5 digest inside the `aktywacja` table before the account is activated, meaning no per-user salt or stretching is applied at any stage.【F:register.php†L200-L218】
- The "Account → Change password" form overwrites the `players.pass` column with another MD5 digest and mirrors the digest inside the session, keeping the legacy hashing scheme alive indefinitely.【F:account.php†L772-L795】
- `install/install.php` seeds the administrator account with the same hashing routine, so no higher-privilege account benefits from stronger storage.【F:install/install.php†L400-L410】

### Cookies and tokens
- Aside from the implicit `PHPSESSID` cookie, the only cookies set by first-party code relate to Smarty's debugging toggle. There are no HTTP-only, Secure, SameSite attributes configured anywhere in the PHP session bootstrap.
- The login and high-risk POST handlers do not issue CSRF tokens or signed action tokens; authorization relies entirely on session state and referer checks.

## Shortcomings and opportunities
1. **MD5 password storage and transport** – MD5 lacks salting, stretching, and GPU resistance, and storing the digest directly in the session means any disclosure of `$_SESSION['pass']` is equivalent to password compromise.
2. **Weak session protection** – PHP defaults leave the cookie unsigned, unencrypted, and without `SameSite` or rotation, making session fixation/replay easier.
3. **No CSRF/anti-replay tokens** – Form submissions can be forged, and privileged actions rely on brittle referer comparisons.
4. **Inconsistent auditing** – Login failures incrementally log to `bugtrack` but there is no central log for suspicious logins or token refresh events.

## Rust-native design goals
The new implementation should plug into the layering described in `docs/rust-architecture.md`, so authentication lives in the Axum HTTP layer and exposes strongly typed session/context objects to the domain services.【F:docs/rust-architecture.md†L3-L64】

1. **Modern password hashing**
   - Adopt `argon2id` via the `argon2` crate plus `password-hash` traits for algorithm agility.
   - Store a per-user random 16–32 byte salt, `argon2id` parameters (time/memory/parallelism), and the resulting PHC string in PostgreSQL.
   - Since the Rust launch will start with a clean player database, enforce Argon2-only storage from the very first registration and keep PHP-era MD5 digests completely out of the new schema.

2. **Session management**
   - Use a PostgreSQL-backed store (e.g., `tower-sessions` + `sqlx-postgres` or a bespoke repository) so session IDs are random 256-bit values persisted in the same database tier the game already depends on—no Redis deployment required.
   - Configure cookies as `HttpOnly`, `Secure`, and `SameSite=Lax`, with rolling session IDs on login and after privilege changes.
   - Store only the player identifier, last authentication timestamp, locale, and CSRF secret in the session. Never mirror password hashes.

3. **Tokens and CSRF**
   - Issue per-session CSRF secrets stored server-side; derive request-specific tokens (e.g., HMAC of secret + form nonce) for every HTML form.
   - For APIs/WebSockets, use signed short-lived access tokens (e.g., `paseto` or `jsonwebtoken`) scoped to player ID and session ID so chat/combat streams can re-authenticate without sharing cookies.

4. **Observability**
   - Emit structured audit events (`login_succeeded`, `login_failed`, `session_rotated`) through the existing tracing stack so `game-web` handlers can feed dashboards or alerts.

## Migration steps
1. **Schema preparation**
   - Add columns to `players` (e.g., `password_phc TEXT`, `password_algo SMALLINT`, `password_updated_at TIMESTAMPTZ`) plus a dedicated `auth_sessions` table keyed by a 256-bit identifier with `player_id`, expiry timestamps, and CSRF secret material.
   - Seed the database with an administrator account hashed via Argon2 so every stored credential already matches the new policy.

2. **Rust session service**
   - Implement an Axum middleware that loads the `Session` from PostgreSQL (using pooled connections), enforces inactivity/absolute timeouts, and exposes a `CurrentPlayer` extractor to downstream handlers.

3. **Login flow rewrite**
   - Build `/auth/login` handler: validate credentials, verify the Argon2 digest, rotate the session ID, persist the session row in PostgreSQL, and set cookies with hardened attributes.
   - Mirror essential PHP behavior (ban checks, freeze windows) inside a Rust service so existing moderation data keeps working.

4. **Registration and password reset**
   - Update registration/reset handlers to exclusively issue Argon2 hashes and drop MD5 usage from new code paths.
   - Generate activation tokens using at least 128 bits of entropy (e.g., `rand::rngs::OsRng`) and store them hashed (SHA-256 + salt) in PostgreSQL instead of plaintext codes in `aktywacja`.

5. **CSRF + token rollout**
   - Inject CSRF middleware into every HTML form route, expose a MiniJinja helper that renders `<input type="hidden" name="csrf" ...>`, and enforce rejection entirely within the Rust controllers once the rewrite is complete.
   - Introduce signed WebSocket/API tokens scoped to the session ID to prevent reuse if a session is revoked.

6. **Cutover and cleanup**
   - Perform final verification (integration tests, smoke logins) against the Rust stack while the PHP site remains offline so no new PHP-side sessions or MD5 hashes are created.
   - Once Axum handlers cover every authentication touchpoint, point traffic at the Rust deployment and archive the legacy PHP login/session code for historical reference only.

This plan prepares the data store and services so we can flip directly from PHP to idiomatic Rust backed by modern cryptography once the rewrite is complete, paving the way for the broader overhaul captured in `docs/rust-architecture.md` and `docs/architecture.md`.
