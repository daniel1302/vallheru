# Agent instructions

- Use English for all code, documentation, and comment unless a task explicitly calls for another language.
- When describing architecture or design choices, connect them to the current PHP codebase where it helps the reader understand the motivation.
- When porting PHP changes, just use PHP as reference, but generate idiomatic Rust code, change code where you think it fits better. 
- Prefer concise Markdown headings and bullet lists inside the `docs/` tree to keep guidance easy to scan.
- If you need to expand or tweak repository guidelines, update this file instead of creating another `AGENTS.md`—the goal is to keep one authoritative instruction list at the repo root.
- Architecture docs should cite the existing `docs/architecture.md` (legacy overview) or other PHP artefacts whenever the new Rust rewrite proposes changes, to make cross-referencing straightforward for reviewers.
- Append useful tips or workflow improvements to this file whenever they can help future agents.
- During the Rust migration we do not need to maintain runtime compatibility with the PHP stack; the Rust application will launch only after the PHP codebase is fully replaced.
- The new Rust game will launch with a fresh database, so avoid building PHP↔Rust data migration tooling or backfill scripts—design everything assuming clean Argon2 credentials and native Rust persistence from day one.