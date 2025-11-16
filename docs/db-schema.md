# Vallheru database schema

## Sources
- `install/db/mysql.sql` – full schema and seed data used by the game installer.
- `install/db/update.sql` – migration script that restructures columns of key tables such as `players` and `monsters`.
- `adodb/session/adodb-sessions.mysql.sql` – definition of the session table used by the ADOdb session handler.

## Core entities
### 1. Players and teams
- **`players`** – central table that stores every account attribute (login, email, resources, combat stats, serialized stats/skills/bonuses, interface preferences, and tribe/team membership). The `id` and `user` columns act as keys reused across the schema (`owner`, `seller`, `prisoner`, etc.). The `settings`, `stats`, `skills`, `bonuses`, `rinvite`, `tinvite`, `team`, and `reputation` fields were added or updated by the migration script, and numeric columns such as `strength` or `level` were moved into text blobs describing the character’s growth.【F:install/db/mysql.sql†L2375-L2445】【F:install/db/update.sql†L1-L39】
- **`bonuses`** – lookup table for passive perks (description, cost, trigger, race/class requirements). Players reference the entries via the serialized `players.bonuses` field to enable features such as mining, herbalism, or magic modifiers.【F:install/db/mysql.sql†L2063-L2094】
- **`teams`** – simple five-member party table. The `leader` and `slot1–slot5` columns contain player identifiers, which lets the game relate teams to `players.id` without an additional join table.【F:install/db/mysql.sql†L40-L48】

### 2. Equipment, resources, and housing
- **`equipment`** – inventory items. Each row keeps the owner (`owner` → `players.id`), name, combat parameters, status (`S` – shop, `U` – equipped), item type, magic flags, repair data, and geographic location. A full-text index on `name` speeds up marketplace search and filtering.【F:install/db/mysql.sql†L953-L979】
- **`potions`** – recipes and produced potions. The `owner` key links to a player and the additional fields (`type`, `efect`, `power`, `status`, `amount`, `cost`) describe the potion behavior and availability (private vs shop).【F:install/db/mysql.sql†L2512-L2525】
- **`herbs`** – herb and seed quantities assigned to a player (`gracz` → `players.id`). Separate columns track both harvest yields and seeds per species, which simplifies the herbalist workshop logic.【F:install/db/mysql.sql†L1146-L1159】
- **`houses`** – real estate owned by players (`owner`). The table keeps information about size, value, furniture status (bed, wardrobe), building costs, tenant info, and geographic region. Houses can therefore act as inns or storage and spend construction points (`build`, `used`).【F:install/db/mysql.sql†L1183-L1199】
- **`warehouse`** – global stock of raw materials (e.g., minerals). Each entry stores buy/sell prices and quantities available in the current economic “reset,” which powers shops and wholesale markets.【F:install/db/mysql.sql†L4093-L4102】

### 3. Combat, missions, and exploration
- **`battlelogs`** – duel log. The `pid` (attacker), `did` (defender), `wid` (winner), and `bdate` timestamp form a raw combat record that can be joined with `players` for PvP statistics.【F:install/db/mysql.sql†L269-L279】
- **`monsters`** – catalog of PvE enemies with combat stats (HP, agility, strength, speed, endurance), descriptions, loot tables, and resistances. The update script removes deprecated experience/currency columns and seeds new entries.【F:install/db/mysql.sql†L1922-L1939】【F:install/db/update.sql†L40-L80】
- **`missions`** – multi-step scenario definitions (e.g., thief storylines). Each record describes a stage (narrative text), available transitions (`exits`), branching probabilities (`chances`, `chances2`, `chances3`), monsters (`mobs`), and rewards (`items`). The `name` identifier allows decision trees without extra database logic.【F:install/db/mysql.sql†L1635-L1648】
- **`quests`** – adventure maps in locations such as the maze. Every record links a step identifier (`qid`, `name`, `option`) with an in-game location and event content. Indexes on `qid`, `location`, and `name` speed up retrieving the next step of a quest.【F:install/db/mysql.sql†L2600-L2612】

### 4. Community and communication
- **`tribes`** – guild structure. Stores the name, owner (`owner` → `players.id`), funds, public/private messages, war parameters (wins/losses, soldiers, fortifications, traps, agents), and clan prefixes/suffixes used when displaying members.【F:install/db/mysql.sql†L3769-L3795】
- **`mail`** – in-game mail with sender, recipient (`owner`), subject, body, and flags for unread/archived status. Indexes facilitate mailbox filtering, while `senderid` and `owner` link entries to `players` without storing historical usernames.【F:install/db/mysql.sql†L1467-L1485】
- **`log` and `logs`** – personal event logs. `log` stores descriptive entries (system messages) with type and read status, whereas `logs` holds short dated notices. Both reference players via the `owner` column.【F:install/db/mysql.sql†L1346-L1371】

### 5. Infrastructure and sessions
- **`adodb_sessions.sessions`** – table used to store PHP sessions. The `sesskey` column is the primary key, while `expiry` and `expireref` define expiration. The `data` column keeps the serialized session state that can be shared across application instances.【F:adodb/session/adodb-sessions.mysql.sql†L3-L16】

## Relationships and dependencies
- Most tables use `players.id` as a foreign key (`equipment.owner`, `houses.owner`, `herbs.gracz`, `mail.owner`, `tribes.owner`, `battlelogs.pid/did/wid`). This makes it possible to model business operations by joining on player identifiers.
- `install/db/update.sql` standardizes player and bestiary data, which impacts the application layer: skills and stats are stored as serialized strings, so modules such as training, combat, or reporting must parse the blobs instead of relying on individual columns.
- Lookup tables (`bonuses`, `tribe_*`, `missions`, `quests`) provide configuration data that the game combines with player records at runtime, enabling new features without schema changes.
