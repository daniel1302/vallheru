# Routing plan

This document inventories every public PHP entry point described in `docs/architecture.md`, documents their dominant `view`/`step` switches, and maps them to Axum routes, middleware, and domain services for the Rust rewrite.【F:docs/architecture.md†L1-L42】  Each feature group below summarizes the related controllers before the alphabetical ledger details every file one by one.

## Feature-level summary

### City & core loop
- `city.php` orchestrates movement between Altara/Ardulith districts, forest travel, and quest deliveries through `step` handlers (`give`, `take`, `forest`).【F:city.php†L63-L158】  Axum should expose `/city` (GET) plus `/city/staff-quest` and `/city/forest` (POST) guarded by auth+locale middleware and CSRF on POST via `CityDirectoryService`, `QuestRewardService`, and `TravelService`.
- `core.php`, `grid.php`, `map.php`, `gory.php`, `las.php`, `wieza.php`, `tower.php`, `travel.php`, `train.php`, `mission.php`, `rest.php`, `temple.php`, and similar gameplay controllers multiplex `view`/`step` switches for arenas, breeding, exploration, map travel, mission pickup, training, and rest flows.【F:core.php†L80-L1240】【F:grid.php†L57-L247】【F:map.php†L42-L248】【F:gory.php†L30-L83】【F:las.php†L32-L81】【F:wieza.php†L35-L315】【F:tower.php†L33-L224】【F:travel.php†L108-L485】【F:train.php†L41-L171】【F:mission.php†L34-L298】【F:rest.php†L32-L207】【F:temple.php†L34-L312】  These should become dedicated Axum routers per activity (e.g., `/core/breeding`, `/exploration/forest`, `/missions/{id}/claim`) to keep GET informational and POST mutating with CSRF.
- Housing-related files (`house.php`, `room.php`, `roommsgs.php`, `warehouse.php`, `bank.php`) need nested routes under `/housing` and `/banking` with `HousingService`, `HousingMailService`, `WarehouseService`, and `BankLedgerService` splitting read vs. write actions, enforced by auth+locale plus CSRF for mutations.【F:house.php†L89-L866】【F:room.php†L142-L372】【F:warehouse.php†L46-L123】【F:bank.php†L48-L1168】

### Markets & economy
- Hub controllers (`market.php`, `map.php` shop listings, `msklep.php`, `smelter.php`, `equip.php`, `armor.php`, `bows.php`, `weapons.php`, `amarket.php`, `hmarket.php`, `imarket.php`, `lmarket.php`, `mmarket.php`, `pmarket.php`, `rmarket.php`, `cmarket.php`, `tribearmor.php`, `tribeware.php`, `tribeastral.php`, `tribeherbs.php`, `tribeminerals.php`, `warehouse.php`) rely on `view` and `step` toggles to sell, craft, and distribute items.【F:market.php†L32-L120】【F:msklep.php†L31-L152】【F:smelter.php†L33-L304】【F:equip.php†L32-L1571】【F:armor.php†L35-L249】【F:bows.php†L35-L189】【F:weapons.php†L35-L183】【F:amarket.php†L75-L543】【F:hmarket.php†L71-L343】【F:imarket.php†L68-L619】【F:lmarket.php†L61-L327】【F:mmarket.php†L65-L319】【F:pmarket.php†L71-L315】【F:rmarket.php†L68-L330】【F:cmarket.php†L50-L205】【F:tribearmor.php†L155-L560】【F:tribeware.php†L147-L320】【F:tribeastral.php†L272-L670】【F:tribeherbs.php†L57-L431】【F:tribeminerals.php†L55-L456】【F:warehouse.php†L46-L123】  Axum should consolidate repeated patterns into typed routers (e.g., `/markets/{category}`, `/clans/armory`) with shared middleware (auth+locale, CSRF on write) backed by `MarketService`, `CraftingService`, and clan inventory services.

### Combat & exploration
- `battle.php`, `explore.php`, `maze.php`, `outpost.php`, `outposts.php`, `hunters.php`, `thieves.php`, `tower.php`, `wieza.php`, `grid.php`, `mission.php`, `quests/*`, `travel.php`, and `train.php` mix PvP, PvE, quests, and training with `action` or `step` toggles.【F:battle.php†L30-L640】【F:explore.php†L165-L576】【F:maze.php†L163-L700】【F:outpost.php†L34-L615】【F:outposts.php†L643-L2134】【F:hunters.php†L57-L381】【F:thieves.php†L72-L347】【F:tower.php†L33-L224】【F:wieza.php†L35-L315】【F:mission.php†L34-L298】【F:travel.php†L108-L485】【F:train.php†L41-L171】  Dedicated Axum routers under `/combat`, `/dungeons`, `/outposts`, `/hunters`, `/thieves`, and `/training` keep matchmaking, fight resolution, and loot as POST endpoints with CSRF while GET surfaces rosters or maps.

### Guilds, clans, and community
- Leaderboards (`guilds.php`, `guilds2.php`, `hof.php`, `hof2.php`, `stats.php`, `chronicle.php`) are GET-only but should live under `/leaderboards/*` with optional auth for personalization.【F:guilds.php†L1-L90】【F:guilds2.php†L1-L90】【F:hof.php†L1-L248】【F:hof2.php†L1-L247】【F:stats.php†L1-L379】【F:chronicle.php†L32-L172】
- Clan gameplay spans `tribes.php`, `tribeadmin.php`, `tribearmor.php`, `tribeware.php`, `tribeastral.php`, `tribeherbs.php`, `tribeminerals.php`, `tribearmor.php`, `tribeware.php`, `tribeastral.php`, `tribeadmin.php`, and supporting armory/vault files, all of which multiplex `view`/`step` to manage sabotage, recruitment, ranks, inventories, and astral machines.【F:tribes.php†L100-L980】【F:tribeadmin.php†L32-L1125】【F:tribearmor.php†L155-L560】【F:tribeware.php†L147-L320】【F:tribeastral.php†L272-L670】【F:tribeherbs.php†L57-L431】【F:tribeminerals.php†L55-L456】  Axum must provide `/clans/*` routers with granular POST endpoints and CSRF for sabotage, donations, and upgrades.
- Social tooling (`chat.php`, `chatmsgs.php`, `mail.php`, `room.php`, `roommsgs.php`, `notatnik.php`, `news.php`, `newspaper.php`, `polls.php`, `forums.php`, `tforums.php`, `roleplay.php`, `stafflist.php`, `team.php`, `memberlist.php`) should become `/social/*` routes with pagination GET endpoints and POST for writes, using `ChatService`, `MailService`, `ForumService`, `NewsroomService`, and `RoleplayService` plus CSRF.

### Onboarding, account, and meta
- Public entry points (`index.php`, `register.php`, `aktywacja.php`, `reset.php`, `referrals.php`, `account.php`, `logout.php`, `log.php`, `rss.php`, `view.php`, `updates.php`, `news.php`, `newspaper.php`, `source.php`) handle registration, activation, password resets, referral tracking, login/logout, and patch notes.【F:index.php†L32-L303】【F:register.php†L32-L121】【F:aktywacja.php†L3-L80】【F:reset.php†L32-L190】【F:referrals.php†L35-L180】【F:account.php†L3-L1440】【F:logout.php†L1-L36】【F:log.php†L32-L140】【F:rss.php†L3-L120】【F:view.php†L1-L180】【F:updates.php†L32-L204】【F:news.php†L32-L205】【F:newspaper.php†L32-L571】【F:source.php†L2-L200】  They should map to `/auth/*`, `/account/*`, `/news/*`, and `/meta/*` routes with appropriate middleware (some anonymous, some auth) and CSRF for state changes.

### Staff and administration
- Moderation and tooling controllers (`admin.php`, `addnews.php`, `addupdate.php`, `sedzia.php`, `bugtrack.php`, `staff.php`, `stafflist.php`, `team.php`, `proposals.php`, `polls.php`, `court.php`, `room.php` admin view) need `/staff/*` routers that enforce auth plus role-based middleware and CSRF for writes while consolidating repeated switch-based flows.【F:admin.php†L32-L1781】【F:addnews.php†L32-L118】【F:addupdate.php†L32-L110】【F:sedzia.php†L32-L141】【F:bugtrack.php†L32-L110】【F:staff.php†L32-L130】【F:stafflist.php†L2-L120】【F:team.php†L2-L120】【F:proposals.php†L32-L200】【F:polls.php†L32-L334】【F:court.php†L120-L380】

## Complete PHP entry point audit (alphabetical)
Each row lists a controller, its feature group, the description pulled from its docblock, and the `view`/`step`/`action` switches that drive its behavior so we can keep parity when designing Axum handlers.
| File | Feature group | Legacy summary | `view` switches | `step` switches | `action` switches |
| --- | --- | --- | --- | --- | --- |
| `account.php` | City/core | Account options - change avatar, email, password and nick | links (L51), bugtrack (L175), bugreport (L286), changes (L335), options (L378), freeze (L561), eci (L848), style (L945), vallars (L1023), forums (L1075), roleplay (L1180), ignored (L1236), contacts (L1339), proposals (L1421) | edit (L109), delete (L158), report (L307), options (L490), freeze (L566), yes (L588), make (L614), usun (L651), dodaj (L670), style (L992), graph (L1008), set (L1151), set (L1214) | change (L123), delete (L238) |
| `addnews.php` | Staff | Adding news in game | — | — | add (L61) |
| `addupdate.php` | Staff | Add and modify game updates | — | — | add (L63), modify (L99) |
| `admin.php` | Staff | Admin panel | bforum (L56), vallars (L67), playerquest (L114), changelog (L134), meta (L162), forums (L186), mill (L288), donator (L325), monster2 (L348), jailbreak (L437), poll (L463), censorship (L550), register (L596), ban (L623), delplayers (L680), mail (L767), bridge (L797), del (L814), add (L894), srank (L932), clearf (L952), equipment (L970), donate (L1027), monster (L1056), kowal (L1096), poczta (L1132), czary (L1164), potions (L1195), alchemy (L1225), close (L1258), pdescriptions (L1284), pitems (L1335), pmonsters (L1470), pbridge (L1598), pmdesc (L1650), rmission (L1702), slog (L1728), slogconf (L1781) | add (L72), add (L119), add (L144), modify (L175), add (L265), mill (L302), add (L333), next (L372), monster (L415), next (L444), second (L480), add (L529), modify (L571), close (L602), modify (L655), send (L771), add (L802), del (L818), add (L918), add (L937), Y (L959), add (L992), donated (L1035), monster (L1078), kowal (L1117), send (L1137), add (L1180), add (L1205), add (L1235), close (L1264), clear (L1772) | — |
| `aktywacja.php` | Onboarding | Activation account | — | — | — |
| `alchemik.php` | Markets & economy | Alchemy mill - making potions | — | — | — |
| `alley.php` | Social/community | List of donators to game | — | — | — |
| `amarket.php` | Markets & economy | Astral market - add, buy astral components from other players | market (L75), add (L297), del (L412), all (L422) | piece (L311), component (L311), piece (L320), buy (L543) | — |
| `ap.php` | City/core | Distribution of Astral Poinst | — | — | — |
| `armor.php` | Markets & economy | Armory shop - buying armors, legs, helmets and shields | — | — | — |
| `bank.php` | Markets & economy | Bank - deposit gold and give item to another player | — | plan (L1085), all (L1119), piece (L1168), component (L1168) | withdraw (L53), deposit (L76), donation (L99), mithril (L160), minerals (L216), herbs (L303), potions (L390), items (L486), pets (L672), steal (L736), safe (L860), astral (L920) |
| `battle.php` | Combat & exploration | Battle Arena - figth between players and player vs monsters | — | — | showalive (L437), monster (L517) |
| `bows.php` | Markets & economy | Fletcher shop - buy arrows and bows | — | buy (L104) | — |
| `bugtrack.php` | Staff | Bugtrack - automated errors and warnings | — | — | delete (L82) |
| `chat.php` | Social/community | Main file of chat - bot Innkeeper and private talk to other players | — | give (L272), ban (L309), clearc (L338), delete (L346) | — |
| `chatmsgs.php` | Social/community | Show text in chat | — | — | — |
| `chronicle.php` | Guilds & clans | Game chronicle - quests and history of game | — | go (L100) | — |
| `city.php` | City/core | City menu and resets without Cron | — | give (L98), take (L107) | — |
| `cmarket.php` | Markets & economy | Core market | market (L50), add (L130), del (L199) | — | add (L167) |
| `core.php` | City/core | Core arena | best (L85), breed (L123), mycores (L303), library (L626), arena (L827), train (L1122), explore (L1219) | breed (L179), name (L562), give (L601), battles (L866), heal (L1079), train (L1163) | — |
| `court.php` | Social/community | Court of law - information about court, law | — | rules (L137), cases (L137), verdicts (L137), rules (L141), cases (L147), verdicts (L153), admin (L202) | add (L233), change (L268), add (L336), delete (L357) |
| `crafts.php` | Markets & economy | Crafts guild - random missions for craftsmen | — | first (L54), register (L773) | — |
| `czary.php` | City/core | Spell book - activation and deactivafion of spells and echance items | — | items (L143) | — |
| `deity.php` | City/core | Change deity of player | — | wybierz (L46), wybierz (L69), change (L84) | — |
| `equip.php` | City/core | Player's equip - wear and drop items, repair, sell and more | — | poison (L1389), drink (L1571) | — |
| `explore.php` | Combat & exploration | Explore forest and mountains | — | battle (L165), run (L181), first (L432), second (L441), third (L457), forth (L493) | moutains (L256), moutains (L430), forest (L576) |
| `farm.php` | Markets & economy | Players farms - herbs | — | herbsinfo (L57), house (L69), plantation (L171) | dry (L101), upgrade (L191), sow (L310), chop (L442) |
| `forums.php` | Social/community | Forums in game | categories (L44), newposts (L109) | — | deltopics (L173), addtopic (L434), Y (L655), move (L670), Y (L716), search (L907) |
| `gory.php` | Combat & exploration | Mountains menu | — | — | back (L66), hermit (L71) |
| `grid.php` | Combat & exploration | Labyrynth - explore and quests | — | quest (L229) | explore (L57) |
| `guilds.php` | Guilds & clans | Guilds - the best players in various crafts. | — | — | — |
| `guilds2.php` | Guilds & clans | Guilds2 - the best players in various fight skills. | — | — | — |
| `hmarket.php` | Markets & economy | Herbs market - add, buy herbs from other players | market (L71), add (L197), del (L274), all (L284) | add (L216), buy (L343) | — |
| `hof.php` | Guilds & clans | Hall of Fame - list of all Heroes in game | — | — | — |
| `hof2.php` | Guilds & clans | Hall of Machines - list of all tribes which build astral machines | — | — | — |
| `hospital.php` | City/core | Hospital - heal and resurrect players | — | — | heal (L88), ressurect (L113) |
| `house.php` | City/core | Players houses | — | buy (L100), new (L162), add (L201), bedroom (L231), wardrobe (L260), upgrade (L290), leave (L571), sell (L607), locator (L631), name (L710), bedroom (L737), wardrobe (L816) | land (L89), build (L149), list (L370), rent (L431), my (L559) |
| `hunters.php` | Combat & exploration | Hunters guild - bestiary | — | bestiary (L67), table (L116), quest (L181) | — |
| `imarket.php` | Markets & economy | Items market | market (L68), add (L340), del (L521), all (L694) | add (L414), buy (L619) | — |
| `index.php` | Onboarding | Main site of game | — | rules (L162), lostpasswd (L175), newemail (L260), donate (L288), promote (L303) | haslo (L178) |
| `jail.php` | Combat & exploration | Jail | — | confirm (L290) | — |
| `jeweller.php` | Markets & economy | Jeweller - make rings | — | plans (L64), make (L143), make2 (L220), make3 (L638) | continue (L368), create (L469), continue (L739), create (L893) |
| `jewellershop.php` | Markets & economy | Jeweller shop | — | — | — |
| `klasa.php` | City/core | Select player class | — | — | — |
| `kopalnia.php` | Combat & exploration | Mines in moutains | — | — | dig (L43) |
| `kowal.php` | Markets & economy | Blacksmith - making items - weapons, armors, shields, helmets, plate legs, arrowsheads | — | — | — |
| `landfill.php` | Markets & economy | Clean city and earn money | — | — | — |
| `las.php` | Combat & exploration | Forest menu | — | — | back (L67), hermit (L72) |
| `library.php` | Social/community | Library with players texts | — | add (L85), addtext (L149), tales (L273), poetry (L273), tales (L275), poetry (L280), comments (L443), rules (L519) | modify (L182), add (L245), delete (L245), add (L255), delete (L460) |
| `lmarket.php` | Markets & economy | Loots market | market (L61), add (L180), del (L275), all (L373) | add (L203), buy (L327) | — |
| `log.php` | Onboarding | Player log - events | — | deleteold (L52), send (L100) | selected (L140) |
| `logout.php` | Onboarding | Logout from game | — | — | — |
| `lumberjack.php` | Markets & economy | Chop trees | — | — | chop (L51) |
| `lumbermill.php` | Markets & economy | Lumbermill - making arrows and bows | — | buy (L199) | — |
| `mail.php` | Social/community | Messages to other players | search (L58), inbox (L262), saved (L409), write (L509) | mail (L113), deleteold (L165), clear (L264), clear (L411), send (L561), send (L751) | — |
| `map.php` | City/core | Show world map | — | — | — |
| `market.php` | Markets & economy | Markets menu | myoferts (L71) | — | — |
| `maze.php` | Combat & exploration | Labyrynth in forrest city | — | battle (L163), run (L179), quest (L700) | explore (L236) |
| `memberlist.php` | Social/community | Players list | — | — | — |
| `mines.php` | Markets & economy | Mines - digging for minerals | — | search (L120), dig (L216) | — |
| `mission.php` | Combat & exploration | Random missions | — | — | — |
| `mmarket.php` | Markets & economy | Potions market | market (L65), add (L187), del (L279), all (L362) | add (L215), buy (L319) | — |
| `msklep.php` | Markets & economy | Potions shop in city | — | buy (L105) | — |
| `news.php` | Social/community | Show game news | all (L82) | comments (L120), add (L196) | delete (L137) |
| `newspaper.php` | Social/community | City newspaper | — | new (L64), new (L97), new (L148), archive (L183), redaction (L285), mail (L452), new (L571) | delete (L226) |
| `notatnik.php` | Social/community | Player notes | — | send (L95), edit (L138) | — |
| `outpost.php` | Combat & exploration | Outpost - random missions for fighters, barbarians and mages | — | first (L173), fight (L615) | — |
| `outposts.php` | Combat & exploration | Players' outposts and all functions related with these (except taxes). | gold (L684), veterans (L739), myoutpost (L957), taxes (L1089), shop (L1151), listing (L1755), battle (L1805), guide (L2134) | player (L697), outpost (L718), modify (L910), add (L965), gain (L1113), list (L1767) | buy (L643), battle (L1835) |
| `pmarket.php` | Markets & economy | Minerals market | market (L71), add (L187), del (L278), all (L385) | add (L207), buy (L315) | — |
| `polls.php` | Social/community | Polls in game | — | delete (L334) | vote (L162), last (L188), comments (L286) |
| `portal.php` | City/core | Magic portal - special location | — | — | — |
| `portals.php` | City/core | Astral plans | — | — | — |
| `preset.php` | City/core | Reset account by player | — | — | — |
| `proposals.php` | Social/community | Proposals in game | — | — | — |
| `rasa.php` | City/core | Select player race | — | — | — |
| `referrals.php` | Onboarding | Show referrals link and amount of referrals | — | — | — |
| `register.php` | Onboarding | Register new players | — | — | register (L121) |
| `reset.php` | Onboarding | Game resets by Cron | — | — | — |
| `rest.php` | City/core | Rest - regenerate mana for a energy | — | — | — |
| `rmarket.php` | Markets & economy | Jewellers market | market (L68), add (L183), del (L277), all (L376) | add (L205), buy (L330) | — |
| `roleplay.php` | Social/community | Roleplay profile | — | — | — |
| `room.php` | City/core | Main file of room - room info and administration | — | quit (L291), admin (L335) | chat (L142) |
| `roommsgs.php` | City/core | Show text in room | — | — | — |
| `rss.php` | Onboarding | RSS Channel | — | — | — |
| `sedzia.php` | Staff | Judge Panel - change rank lawyers and members | — | add (L59) | — |
| `smelter.php` | Markets & economy | Smelter - smelt minerals | — | upgrade (L51), smelt2 (L128), smelt (L304) | — |
| `source.php` | Onboarding | Page source | — | — | — |
| `staff.php` | Staff | Staff panel - give immunited, send players to jailetc | bforum (L51) | — | — |
| `stafflist.php` | Social/community | Game staff list | — | — | — |
| `stats.php` | Guilds & clans | Player statistics and general informations about account | — | — | gender (L39), newbie (L379) |
| `team.php` | Social/community | Team management | — | — | — |
| `temple.php` | City/core | Temple | — | — | — |
| `tforums.php` | Social/community | Clans forums | newposts (L48), topics (L205) | — | deltopics (L123), addtopic (L155), Y (L409), search (L605) |
| `thieves.php` | Combat & exploration | Thieves den, items, monuments and missions for thieves | — | monuments (L72), shop (L171), missions (L204), confirm (L347) | — |
| `tower.php` | Combat & exploration | Game time | — | — | — |
| `train.php` | City/core | School - train stats | — | — | train (L119) |
| `travel.php` | City/core | Travel to other locations and magic portal | — | caravan (L338), caravan (L424), magic (L424) | — |
| `tribeadmin.php` | Guilds & clans | Manage clans | — | — | buy (L637), kup (L727), tags (L940), www (L952), edit (L1021), kick (L1054), loan (L1119) |
| `tribearmor.php` | Guilds & clans | Tribe armor - weapons and armors | — | zobacz (L168), daj (L527) | — |
| `tribeastral.php` | Guilds & clans | Tribe astral vault - plans, maps, recipes | — | plan (L275), all (L318), piece (L357), component (L357), all (L493), piece (L507), component (L507), plan (L566), all (L614) | view (L132), add (L208), give (L412), safe (L670) |
| `tribeherbs.php` | Guilds & clans | Tribe herbs | — | — | — |
| `tribeminerals.php` | Guilds & clans | Clans minerals | — | — | — |
| `tribes.php` | Guilds & clans | Clans - info, manage, herbs, minerals and battles | all (L109), view (L149), make (L742), my (L798) | traps (L358), agents (L378), traps (L399), agents (L399), espionage (L440), sabotage (L440), sabotage (L442), steal (L548), traps (L564), agents (L583), espionage (L602), sabotage (L622), members (L654), make (L766), astral (L883), members (L936), quit (L961) | — |
| `tribeware.php` | Guilds & clans | Tribe magazine - add potions to clan, give potions to clan members | — | zobacz (L150), daj (L294) | — |
| `updates.php` | Onboarding | Show main news in game | — | comments (L187) | delete (L204) |
| `view.php` | Onboarding | View other players, steal money, astral components from other players | — | — | — |
| `warehouse.php` | City/core | Warehouse - sell minerals and herbs | — | — | sell (L46), buy (L46), sell (L55), sell (L99), sell (L123) |
| `weapons.php` | Markets & economy | Weapons shop | — | — | — |
| `wieza.php` | Combat & exploration | Magic tower - buy spells, staffs and capes | — | — | — |
| `zloto.php` | Markets & economy | Show players minerals, herbs, gold and maps | — | — | — |
