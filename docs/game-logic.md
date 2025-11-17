# Game logic inventory

## HTTP-independent helper modules

| File | Function(s) or class | Responsibility |
| --- | --- | --- |
| `includes/head.php` | `CountExecs`, `catcherror`, `error`, `message`, `integercheck`, `isint`, `checkvalue` | Shared bootstrap utilities: wraps DB calls to count queries/time, captures PHP errors into `bugtrack`, renders fatal/flash messages, and centralizes request validation without touching HTTP routing.【F:includes/head.php†L46-L200】【F:includes/head.php†L744-L825】 |
| `includes/counttime.php` | `counttime` | Pure time math for announcing the next global reset tick based on server time, returning localized hour/minute strings for any caller.【F:includes/counttime.php†L32-L90】 |
| `includes/avatars.php` | `scaleavatar` | Scales uploaded avatar images down to fixed dimensions without depending on any controller logic.【F:includes/avatars.php†L33-L66】 |
| `includes/comments.php` | `displaycomments`, `addcomments`, `deletecomments` | Standalone comment pagination, creation, and moderation used by news/updates/polls; functions operate solely on DB state and localization, returning arrays for templates.【F:includes/comments.php†L32-L129】 |
| `includes/marketaddto.php` | `addtomin`, `addtoitem`, `addtoherb`, `addtoastral` | Mutates market tables when increasing existing offers and refunds stock to owners, encapsulating the math for each commodity type.【F:includes/marketaddto.php†L33-L95】 |
| `includes/marketdel.php` | `deletemin`, `deleteitem`, `deleteherb`, `deletepotion`, `deleteastral` | Reverses a single listing per market, crediting resources back to players or inventory while removing the offer atomically.【F:includes/marketdel.php†L33-L214】 |
| `includes/marketdelall.php` | `deleteallmin`, `deleteallherb`, `deleteallpotion`, `deleteallastral`, `deleteallcores` | Bulk-withdraws every offer a player has across markets, looping through each resource class without HTML coupling.【F:includes/marketdelall.php†L33-L146】 |
| `includes/resets.php` | `energyreset`, `smallreset`, `mainreset` | Executes scheduled ticks: refills energy, grows farms, refreshes markets, resolves jail timers, resets events, and rebuilds global counters, independent of HTTP requests.【F:includes/resets.php†L32-L240】 |
| `includes/battle.php` | `critical`, `attack1` | Core PvP combat math (critical chance, weapon/spell resolution, dodge, stamina) consumed by arena controllers without rendering.【F:includes/battle.php†L33-L200】 |
| `includes/functions.php` | `drink`, `equip`, `drinkfew` | Inventory logic for consuming potions and equipping gear, handling stats/effect calculations and DB updates without knowledge of UI flows.【F:includes/functions.php†L32-L220】【F:includes/functions.php†L242-L547】【F:includes/functions.php†L550-L689】 |
| `includes/findastral.php` | `Findastral` | RNG routine for discovering astral plans/maps/recipes and inserting them into the `astral` table.【F:includes/findastral.php†L32-L88】 |
| `includes/astralvault.php` | `showastral`, `mergeplans`, `showcomponents` | Aggregates astral pieces into arrays, exposes merge eligibility, and transfers components between storage tables independent of presentation.【F:includes/astralvault.php†L32-L305】 |
| `includes/astralsteal.php` | `astralsteal` | Governs astral theft attempts, including stat checks, vault bonuses, jail handling, and loot transfer, making no assumptions about the calling page.【F:includes/astralsteal.php†L33-L200】 |
| `includes/funkcje.php` | `hitlocation`, `checkpet`, `battlerecords`, `monsterloot`, `autofill`, `loststat`, `gainability`, `lostitem`, `showcritical`, `monsterattack2`, `playerattack`, `fightmonster` | Shared combat helpers: hit location RNG, pet survivability, loot distribution, quiver auto-fill, stat loss/gain, item destruction, and full PvM fast fights—all headless domain utilities.【F:includes/funkcje.php†L32-L260】【F:includes/funkcje.php†L142-L332】【F:includes/funkcje.php†L202-L330】 |
| `includes/monsters.php` | `encounter`, `randommonster` | Generates balanced monster stats based on player attributes and environment, returning structured arrays for fights.【F:includes/monsters.php†L31-L190】 |
| `includes/turnfight.php` | `turnfight`, `attack`, `castspell`, `monsterattack`, `fightmenu` | Implements the state machine for turn-based PvM combat, from setup to stamina management and AI turns; controllers simply call these functions.【F:includes/turnfight.php†L32-L200】 |
| `includes/bbcode.php` | `bbcodetohtml`, `htmltobbcode` | Text-format conversion between BBCode and sanitized HTML, including dice rolls/emotes, usable across chat, forums, and mailers.【F:includes/bbcode.php†L32-L226】 |
| `includes/verifypass.php` | `verifypass` | Password policy enforcement (length, alphanumeric, uppercase) called from registration/profile forms but independent of HTTP state.【F:includes/verifypass.php†L32-L91】 |
| `includes/steal.php` | `steal` | Calculates thievery checks, XP payouts, arrests, and inventory transfers when robbing shops, decoupled from UI flow.【F:includes/steal.php†L32-L164】 |
| `includes/ranks.php` | `selectrank` | Maps internal rank codes to localized titles based on gender without any controller context.【F:includes/ranks.php†L32-L60】 |
| `includes/checkastral.php` | `checkastral` | Evaluates tribe-owned components to mark an astral machine as buildable/not, updating a single table independent of pages.【F:includes/checkastral.php†L32-L71】 |
| `includes/counttime.php` | `counttime` | Provides countdown strings for reset timers, ready for reuse inside cron jobs or controllers.【F:includes/counttime.php†L32-L90】 |
| `class/player_class.php` | `Player` | Domain aggregate representing a player: constructor hydrates stats, equipment, bonuses, settings, and helper methods for exp/buffs independent of web rendering.【F:class/player_class.php†L1-L200】 |
| `class/quests_class.php` | `Quests` | Quest engine: enforces referrer checks, stores progress, renders texts, handles rewards, and invokes turn-fight helpers, all without HTTP awareness.【F:class/quests_class.php†L1-L200】 |
| `class/team_class.php` | `Team` | Aggregates party members, merges their stats/skills/HP, splits experience, and propagates deaths for group combat scenarios.【F:class/team_class.php†L32-L197】 |
| `class/bot_class.php` | `Bot` | ELIZA-like NPC chat responder that parses user text, performs DB lookups, and returns replies, enabling use in chat without embedding HTTP logic.【F:class/bot_class.php†L1-L190】 |

## Gameplay feature catalog

### Character & account management
- `account.php` lets players manage quick links, report bugs, and adjust settings (avatar, email, password, nick) entirely through server-side validation and template assignments.【F:account.php†L32-L170】
- `temple.php` handles deity-specific work, prayers, and blessings, enforcing race/deity combinations and energy costs for service jobs or stat buffs.【F:temple.php†L24-L140】

### Economy & trading
- `bank.php` supports gold withdrawal/deposits, inter-player donations (gold or mithril), and transaction logging, guarding against invalid recipients or insufficient funds.【F:bank.php†L24-L140】
- `market.php` centralizes access to item/mineral/herb/potion/astral/core markets, including deleting all offers, counting per-market listings, and routing to specialized pages.【F:market.php†L24-L140】
- `kowal.php` provides blacksmith crafting, with success/bonus rolls, stat-based item bonuses, and experience gains for each recipe, independent of HTTP presentation.【F:kowal.php†L1-L120】
- `mines.php` manages prospecting and mining for specific ores: tracking personal deposits, hiring geologists, checking resource availability, and charging mithril/gold per search.【F:mines.php†L1-L120】

### Combat & progression
- `battle.php` (controller) orchestrates arena PvP by bootstrapping player/enemy stats, loading spells, applying bonuses, and invoking shared combat helpers when `battle` is requested.【F:battle.php†L1-L140】
- `core.php` runs the pet arena, including license purchases, hall-of-fame listings, breeding workflows, and breeding cost calculations, all before combat even begins.【F:core.php†L24-L160】
- `travel.php` governs moving between Altara, Ardulith, forests, and mountains, exposes portals to astral planes, and injects random bandit fights through `turnfight` when walking.【F:travel.php†L1-L140】

### Crafting, gathering, and world simulation
- `farm.php`, `lumberjack.php`, `smelter.php`, and related files (not exhaustively listed here) follow the same pattern as `kowal.php`/`mines.php`, using the shared helper modules to advance resources between resets.【F:resets.php†L32-L240】
- `guilds.php` calculates top artisans per craft by parsing player skill blobs and generating leaderboard data for display in the artisans' guild.【F:guilds.php†L1-L100】

### Social systems
- `tribes.php` implements clan creation, membership, permissions, shared storage, wars, and astral machine collaboration, starting from a detailed menu describing the political role of clans.【F:tribes.php†L1-L110】
- `class/quests_class.php` (paired with `quests/*.php` content files) controls quest progression, rewards, and battle integration, ensuring state transitions without duplicating logic in each quest page.【F:class/quests_class.php†L32-L200】

## Function and class index

The table in “HTTP-independent helper modules” doubles as a function/class index: each row lists every callable and its file path, enabling quick mapping between the PHP codebase and future Rust ports. Refer to that table when migrating individual behaviors.

## Migration priorities

1. **Combat kernel (`includes/battle.php`, `includes/funkcje.php`, `includes/turnfight.php`)** — These files encapsulate nearly all combat math, pet handling, and turn logic, and they are imported by many controllers (arena, quests, travel). Porting them early unlocks PvP/PvM parity in the Rust domain layer described in `docs/rust-architecture.md`.【F:includes/battle.php†L33-L200】【F:includes/funkcje.php†L32-L260】【F:includes/turnfight.php†L32-L200】【F:docs/rust-architecture.md†L7-L50】
2. **Inventory & item systems (`includes/functions.php`, `includes/market*.php`, `includes/steal.php`)** — Equipment and market helpers form the backbone of the economy and are already HTTP-agnostic, making them ideal to translate into Rust services that HTTP handlers can reuse without duplicating rules.【F:includes/functions.php†L32-L547】【F:includes/marketaddto.php†L33-L95】【F:includes/marketdel.php†L33-L214】【F:includes/marketdelall.php†L33-L146】【F:includes/steal.php†L32-L164】
3. **Resets and world simulation (`includes/resets.php`)** — The reset cron touches farms, jails, random events, and resource regeneration. Migrating it to a background job (`game-jobs` in the Rust plan) ensures consistent server ticks before less critical UI rewrites.【F:includes/resets.php†L32-L240】【F:docs/rust-architecture.md†L66-L122】
4. **Astral systems (`includes/astralvault.php`, `includes/astralsteal.php`, `includes/checkastral.php`, `includes/findastral.php`)** — Astral components underpin late-game features like the astral machine and portal travel. Consolidating their logic into Rust early prevents duplication when porting clans and portals later.【F:includes/astralvault.php†L32-L305】【F:includes/astralsteal.php†L33-L200】【F:includes/checkastral.php†L32-L71】【F:includes/findastral.php†L32-L88】
5. **Player aggregate (`class/player_class.php`)** — Once combat and inventory systems move, the `Player` model should become a Rust domain aggregate to avoid repeatedly hydrating PHP arrays. This class already centralizes stats, equipment, bonuses, and settings, making it the linchpin for the future repository interfaces sketched in the Rust architecture doc.【F:class/player_class.php†L1-L200】【F:docs/rust-architecture.md†L90-L150】
