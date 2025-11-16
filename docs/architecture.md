# Architecture Overview

## Routing and entry points
- `index.php` is the public entry point. It loads configuration, instantiates Smarty, queries settings, and routes between the main landing page, rules, password recovery, and other actions using the `step` query parameter. Each branch renders a different template via Smarty after executing the necessary database queries.【F:index.php†L32-L200】
- Authenticated gameplay is split into many feature-specific controllers located at the repository root (for example `core.php`). Each controller includes `includes/head.php` to bootstrap the shared environment (session, database connection, player object, translations) before performing its own logic and rendering output. The controllers typically gate actions based on `$_GET` parameters such as `view`/`step` to simulate sub-routes within the page.【F:core.php†L33-L200】

## Session management
- `includes/sessions.php` is required by `includes/head.php` at startup and is solely responsible for calling `session_start()` and regenerating the session identifier on the first request, mitigating fixation attacks.【F:includes/head.php†L46-L51】【F:includes/sessions.php†L1-L8】

## Database layer
- The installer writes `includes/config.php` so that it connects to MySQL via the ADODB abstraction layer (`NewADOConnection('mysqli')`), sets the cache directory, and stores global configuration such as game metadata and localization. This file defines the `$db` handle used everywhere else in the code base.【F:install/install.php†L358-L378】
- Controllers and helpers issue SQL queries directly through the shared `$db` object. For example, the public index pulls configuration from the `settings` table, while gameplay files like `core.php` query and update creature and player state via `SELECT`, `UPDATE`, and `DELETE` statements.【F:index.php†L64-L200】【F:core.php†L73-L200】
- `includes/head.php` wraps `$db->Execute` with the `CountExecs` helper to track query counts and execution time, later surfaced in the footer, giving administrators performance insight.【F:includes/head.php†L57-L84】

## Game logic
- Each feature controller encapsulates its own gameplay rules. `core.php` is a representative example: it validates player location and licenses, presents arena ladders, handles breeding workflows, and charges in-game currency. The file mixes player state checks with multiple SQL operations and Smarty assignments to produce the interactive UI.【F:core.php†L41-L200】
- Shared gameplay routines live in `includes/functions.php`. The `drink()` function, for instance, enforces potion ownership, calculates effects on mana/HP/antidotes, persists the results to the database, and emits feedback through the messaging subsystem. This helper is invoked by several controllers dealing with potions and equipment management.【F:includes/functions.php†L37-L200】

## Templates and presentation
- `includes/head.php` instantiates the Smarty template engine, attaches localization, and handles login-related messaging before any controller renders templates.【F:includes/head.php†L46-L200】
- `includes/foot.php` gathers footer statistics (online players, performance metrics, memory usage), assigns them to Smarty, renders `footer.tpl`, and finalizes output (closing DB connection, flushing gzip buffers, and closing the session).【F:includes/foot.php†L35-L158】
- Each controller is responsible for assigning its own view variables (`$smarty->assign`) before including `includes/foot.php`, giving a consistent templating pipeline across the site.【F:core.php†L46-L200】

## Dependency sketch
```
index.php / feature controllers (e.g., core.php)
        |
        v
includes/head.php
    |- includes/config.php  -> creates $db via ADODB (mysqli)
    |- includes/sessions.php -> PHP session bootstrap
    |- libs/Smarty.class.php -> template engine
    |- class/player_class.php -> loads current player
        |
        v
Gameplay helpers (includes/functions.php, etc.)
        |
        v
includes/foot.php -> aggregates stats, renders footer via Smarty
```

This layered flow ensures every page shares the same bootstrap (config, DB, session, translations, template engine) while still allowing each controller to implement its own piece of the game's logic.
