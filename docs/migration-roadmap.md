# Migration Roadmap

This roadmap prioritizes Vallheru's PHP-to-Rust rewrite and coordinates the later frontend refresh. It references the legacy architecture summarized in `docs/architecture.md` to keep the new implementation grounded in the current bootstrap, routing, and gameplay flows.

## Module migration priorities
1. **Bootstrap & session core** (`index.php`, `includes/head.php`, `includes/sessions.php`, `includes/config.php`, `includes/foot.php`)
   - Provides the baseline runtime (routing, DB access, session lifecycle) described in the architecture overview, so the Rust stack must replicate it before any gameplay logic can run.
2. **Account, authentication, and player profile** (`register.php`, `reset.php`, `login` logic in `index.php`, `class/player_class.php`).
   - Enforces new credential storage (Argon2) and player loading semantics early to avoid rework across modules.
3. **Core gameplay hub** (`core.php`, `city.php`, `stats.php`).
   - These controllers orchestrate most in-game navigation and expose shared widgets; porting them early gives the Rust server a functional landing area for testers.
4. **Economic subsystems** (market controllers such as `market.php`, `amarket.php`, `mmarket.php`, `warehouse.php`).
   - Trading touches inventory, currency, and taxes; migrating them together keeps transaction rules consistent.
5. **Combat and exploration** (`battle.php`, `explore.php`, `gory.php`, `las.php`, `maze.php`).
   - Once the economy is stable, bring over PvE/PvP mechanics to validate stat progression, loot tables, and travel timers.
6. **Guilds and tribes** (`guilds.php`, `tribes.php`, supporting `tribe*` files).
   - Social features depend on earlier modules (inventory, combat rewards) and can be tested after players can reliably earn and trade resources.
7. **Peripheral activities & minigames** (crafting, temple, house, portals, events).
   - Migrate these iteratively after the main loop is battle-tested; they share many helpers but pose lower risk to overall playability.

## PHP → Rust migration stages
1. **Foundational runtime**
   - Design axum-style routing mirroring the PHP entry-point split (`index.php` vs. feature controllers) and port the session/bootstrap logic noted in `docs/architecture.md` to a Rust middleware stack.
   - Replace ADODB/MySQL access with an async ORM (e.g., SQLx) that encapsulates config loading, query counting, and localization lookup.
2. **Domain models and services**
   - Model players, inventory, quests, and guilds as typed structs; implement repositories/commands that correspond to the PHP helpers in `includes/functions.php`.
   - Bring over business rules module-by-module following the priority list, translating PHP data mutations into Rust service methods with unit tests.
3. **Feature controllers and routing**
   - For each migrated module, expose HTTP handlers that reproduce the PHP `view`/`step` behavior through explicit routes or state machines.
   - Ensure each handler composes shared middleware for authentication, localization, and rate limiting.
4. **Integration hardening**
   - Once a slice is ported, run scenario tests covering DB state transitions, currency balances, and combat math to validate parity with the PHP reference implementation.
   - Instrument tracing to replicate the PHP footer metrics (query counts, memory) so operators retain observability.

## Frontend migration stages
1. **Design system baseline**
   - Audit existing Smarty templates and CSS to extract shared layouts (header, left nav, content) into a modern component library (e.g., React + Tailwind) while keeping typography and iconography recognizable.
2. **Server-rendered to SPA/SSR bridge**
   - Introduce a Rust-friendly templating or SSR layer (Leptos, Yew + Trunk, or React via Next.js consuming Rust APIs) that can initially host static translations of the Smarty pages for the modules being ported.
3. **Feature-by-feature UI refresh**
   - As each backend module lands, implement the corresponding frontend view: start with login/account flows, then the city/core dashboard, followed by markets, combat panels, and finally guild/social interfaces.
   - Preserve legacy UX cues (button labels, color coding) to ease the transition while layering in responsive layouts and accessibility improvements.
4. **Progressive enhancement & polish**
   - Add client-side state management for chat, notifications, and timers; integrate WebSockets once the Rust backend exposes them.
   - Finalize visual polish (animations, icons) and run usability tests before decommissioning the PHP Smarty templates entirely.
