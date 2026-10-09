# arsip — Design Summary

## 1. Core Concept

- **arsip** = a terminal-first, file-driven CMS written in Rust.
- Files (`.md` with `+++` TOML frontmatter) are the **source of truth**.
- Postgres is a **materialized, rebuildable index** over the files.
- Redis is a **render cache** for HTML output.
- Every DB mutation is **human-commanded**, never automatic.
- Name: "arsip" (Indonesian for "archive").

---

## 2. Architecture Principles

- **Files → DB is one-way.** The CLI never writes back to `./content/`.
- **DB is disposable.** Drop it, re-run `commit`, everything rebuilds.
- **Redis is disposable.** Flush it, it repopulates on read.
- **No silent defaults.** Missing metadata → rejected and reported.
- **No silent archival.** Missing file → `missing_since` timestamp, not immediate status change.
- **Explicit commands only.** Read commands are safe; write commands are deliberate.
- **TyDD/DDD throughout.** Domain is pure; ingestion, cache, enrichment are infrastructure behind traits.

---

## 3. Source of Truth Decision

- **Chosen: files as source of truth.**
- Reason: git already solves content history, diff, revert, and disaster recovery.
- DB-as-truth would require building revision tables, backups, and migrations — cost not worth paying now.
- **Escape hatch pre-built:** `source` column on each row (`'file'` or `'db'`), so DB-as-truth can be adopted later without a rewrite.
- **`source_snapshot` column** (optional) preserves original raw file text for lossless restore and `diff`.
- **Rule:** `commit` only touches `source='file'` rows. Rows with `source='db'` are invisible to file sync.

---

## 4. File Format

- Markdown files with **TOML frontmatter** delimited by `+++`.
- Opening `+++` must be on line 1. Closing `+++` required.
- Empty body is legal. Empty frontmatter is not.
- Parser: `gray_matter` + `serde` + `toml` crate.
- Invalid frontmatter → file rejected loudly, reported in `scan`.

---

## 5. Metadata Schema

**Required fields (rejected if missing):**

- `title`
- `slug`
- `language`
- `status`

**Optional fields (explicit "unset" semantics):**

- `publish_at` (absent = never auto-publish)
- `category` (absent = uncategorized)
- `tags` (absent = `[]`)

**Derived fields (never written to file):**

- `id` (not UUID — use slug + language as composite PK)
- `filename`
- `content`
- `content_hash`
- `created_at`
- `updated_at`
- `revision`
- `author`
- `missing_since`
- `source`

**Field naming conventions:**

- Past participles for status: `draft | published | archived`
- Snake_case, conventional names: `updated_at` (not `update_at`)

**Language field:**

- Store as BCP 47 (e.g., `en`, `en-US`, `zh-Hant`), validated length 2–5.
- If constrained to 2-char for v0.1, document as a known future refactoring.

**Tags and category:**

- Stored as **normalized tables** with join tables, not comma-separated strings or JSON arrays.
- Enables "list all articles tagged `rust`" and "rename tag `iso9000` to `iso9001`" cheaply.

**Identity:**

- **Primary key: `(slug, language)` composite.**
- `filename` is a mutable attribute, not identity.
- `content_hash` detects renames vs. delete+create.

---

## 6. Command Surface

**Read-only (safe, never touch DB):**

- `arsip scan` — parse files, validate, report.
- `arsip diff` — compare files vs. DB, show what `commit` would do.
- `arsip status` — short summary.
- `arsip list [--status X]` — list DB rows.
- `arsip show <slug>` — show one row.
- `arsip history <slug>` — (v0.2+) revision list.

**Write — DB from files (the only filesystem→DB path):**

- `arsip commit [--dry-run] [--confirm-missing] [--only <slug>]`

**Write — DB-native content management:**

- `arsip new <slug>` — create row with `source='db'`.
- `arsip edit <slug>` — edit `source='db'` row.
- `arsip archive <slug>` — explicit archive.
- `arsip restore <slug>` — write file to `./restored/`, never to `./content/`.
- `arsip purge <slug> --force` — delete row entirely.

**Cache:**

- `arsip cache invalidate <slug>`
- `arsip cache flush`

**Enrichment (opt-in):**

- `arsip enrich [<slug>] [--review]`

**Operations:**

- `arsip doctor` — health check (DB, Redis, config, editor, paths).
- `arsip export <slug> [--to <path>]` — write a file from a DB row.

---

## 7. Sync Mechanics

- **Scan ≠ commit.** Scan is read-only. Commit is the only writer.
- **Flow:** write files → `scan` (validate) → `diff` (preview) → `commit` (apply).
- **Config-driven policies** with conservative defaults:
  - `missing_policy = "pending"` (mark `missing_since`, don't change status).
  - `--confirm-missing` required to actually archive on missing.
- **Two failure modes, two reports:**
  - Parse failure → error (bad TOML, missing `+++`).
  - Validation failure → warning (missing required field).
- **Exit codes:** 0 = clean, 1 = incomplete files, 2 = parse errors.
- **`--report json`** for machine-readable output.

---

## 8. Missing File Handling

- Missing file ≠ archive immediately.
- First detection → set `missing_since` timestamp, status unchanged.
- Confirm with `arsip commit --confirm-missing` to archive.
- Prevents false positives from renames, moves, config typos, temporary file states.
- Renames detected via `content_hash` → treated as rename, not delete+create.
- **Never auto-delete.** Archive is terminal. Deletion is explicit (`arsip purge`).

---

## 9. Restore Behavior

- `restore` writes to **`./restored/` by default**, never back to `./content/`.
- Writer reviews and moves manually — keeps `./content/` human-authored.
- `--to <path>` for explicit target. `--force` to overwrite. Never overwrite silently.
- Refuses if a file with same slug already exists (unless `--force`).
- **Round-trip CI test:** `restore` → `scan` → `commit` should leave DB unchanged.
- **Formatting caveat:** TOML serialization loses comments and key order. `source_snapshot` column enables lossless restore if needed.

---

## 10. Cache Design

- **Redis only.** No file-based cache option — the filesystem already is the source.
- **Cache the rendered HTML body** (via Maud), not the markdown, not the metadata.
- **Cache key format:**

  ```
  arsip:article:{language}:{slug}:rev{revision}:r{render_version}
  ```

- **`render_version`** is a constant in the binary, bumped when templates change.
- **Invalidation:** explicit on every write; long TTL (e.g., 7 days) as safety net.
- **Never cache metadata in Redis.** Metadata lives in Postgres.
- **Cache key function lives in a shared crate** (`arsip-core`), used by both CLI and portfolio. No string-formatting keys by hand.
- **Renderer behind a trait:** `trait Renderer { fn render(&self, article: &Article) -> Html }` with `MaudRenderer` and `PlaintextRenderer` (for tests).
- **Cache behind a trait:** `trait ContentCache { fn get; fn set; fn invalidate }` with `RedisCache` and `NoopCache` (for tests).

---

## 11. AI Enrichment

- **Separate command: `arsip enrich`.** Never part of `commit`.
- **Opt-in** via config (`enrich.enabled = false` by default).
- **Never mutates source `.md` files.** AI output goes to DB only.
- **Results cached by `content_hash`.** Unchanged content → no re-call.
- **Records model + prompt version** with each enrichment for auditability.
- **Human review required:** `arsip enrich --review` shows suggestions before saving.
- **Language detection: local** (`whatlang` or `lingua`), not an LLM. Deterministic, free, offline.
- **LLM for:** 1000-char summary, suggested tags, suggested category.
- **Enricher behind a trait:** `trait Enricher { async fn summarize; async fn suggest_tags; ... }` with real and `NoopEnricher` impls.
- **Prompts versioned in the repo** (`prompts/summarize/v3.md`), diffable in git.
- **Rate limiting + retries** via `tower` or `backoff`.

---

## 12. Configuration File (`arsip.toml`)

```toml
[scan]
dir = "./content"
file_glob = "**/*.md"
follow_symlinks = false

[validation]
strict = true
required = ["title", "slug", "language", "status"]

[commit]
missing_policy = "pending"    # "pending" | "archive" | "ignore"
default_status = "draft"

[restore]
target_dir = "./restored"
overwrite = false

[cache]
backend = "redis"
key_prefix = "arsip:article"
render_version = 1

[enrich]
enabled = false
provider = "openai"
model = "gpt-4o-mini"
language_detector = "whatlang"
```

**Principle:** every decision has a config key with a safe default. Conservative defaults. Human opts into risk explicitly.

---

## 13. Directory & Repo Structure

**Multirepo:**

- `arsip/` — public, the CMS (workspace inside)
- `portfolio/` — private, the authenticated viewer (consumes arsip crates)
- `iso9001-helper/` — separate repo, later
- `oee-calculator/`, `machine-sim/`, `custom-db/` — future repos

**Inside `arsip/` (workspace):**

```
arsip/
├── crates/
│   ├── domain/          # pure, no I/O, no async
│   ├── application/     # use cases
│   ├── infrastructure/  # sqlx, redis, file readers
│   ├── render/          # Maud templates
│   ├── enrich/          # AI integration (trait + impls)
│   ├── cli/             # clap-based commands
│   ├── tui/             # ratatui — thin shell
│   ├── core/            # shared types, cache_key(), public API
│   └── server/          # bin: `arsip` binary
└── docs/refactorings/
```

**Dependency rule (enforced by crate boundaries):**

- `domain` depends on nothing (except `std`, `thiserror`).
- `application` depends on `domain`.
- `infrastructure` depends on `domain` + `application`.
- `cli` / `tui` / `server` depend on all.
- **`domain` never depends on `infrastructure`.** Compiler enforces this.

---

## 14. Refactoring Trail (The Portfolio Thesis)

Every significant design change → a `docs/refactorings/NNN-*.md` entry with this structure:

1. **Context** — why the original design existed.
2. **Pain** — what change became hard.
3. **The refactor** — link to actual commits/PR.
4. **What made it easy** — which architectural choices paid off.
5. **What made it hard** — honest tradeoffs.
6. **Metrics** — LOC changed, files touched, tests changed vs. unchanged.

**The killer metric:** "0 domain tests changed" when infrastructure was swapped.

**Candidate entries:**

- `000-source-of-truth.md` — files vs. DB decision.
- `001-commit-is-a-command-not-a-scan.md` — splitting scan/diff/commit.
- `002-extracting-tui-as-a-frontend.md` — thin shell discipline.
- `003-silent-defaults-to-explicit-auditable.md` — validation change.
- `004-pending-archive-state.md` — missing-file handling.
- `005-second-consumer-forces-crate-extraction.md` — portfolio integration.
- `006-redis-cache-with-render-version.md` — cache invalidation.
- `007-ai-enrichment-as-opt-in-pipeline.md` — non-determinism containment.

---

## 15. Roadmap

**v0.1 — Core sync**

- Scan dir, parse `+++` TOML, strict validation.
- Postgres schema: `articles`, `tags`, `article_tags`, `categories`.
- Commands: `scan`, `diff`, `commit`, `list`, `show`, `status`, `doctor`.
- No Redis, no restore, no AI, no TUI yet.

**v0.2 — Cache + Render**

- Maud rendering of article body.
- Redis cache with `render_version`.
- Shared `cache_key()` in `arsip-core`.
- `arsip cache invalidate`.

**v0.3 — Lifecycle + TUI**

- `missing_since` + `--confirm-missing` archive flow.
- `restore` command to `./restored/`.
- Round-trip CI test.
- TUI as thin shell over application layer.

**v0.4 — Enrichment (optional)**

- `arsip enrich` — AI summary, tags, category.
- Local language detection.
- Prompt versioning.

**v0.5+ — Refactorings in public**

- Second consumer (portfolio) integration.
- Publish `arsip-core` as a library crate.
- Optional HTTP frontend behind feature flag.

---

## 16. What's Cut (Deliberately)

- **CSS in config** — frontend concern, belongs to portfolio, not CMS.
- **Silent defaults** — replaced by strict validation + reports.
- **File-based cache option** — Redis only.
- **Auto-archive on missing file** — replaced by `missing_since` + confirmation.
- **UUID as primary key** — slug + language instead.
- **`---` YAML frontmatter** — `+++` TOML instead.
- **AI inside `commit`** — separate `enrich` command.
- **WYSIWYG editor** — respects Unix philosophy; use `$EDITOR`.
- **Web admin UI in CMS repo** — belongs in portfolio repo.
- **Custom database engine** — separate future project.

---

## 17. Rust Crates to Use

| Purpose           | Crate                                        |
| ----------------- | -------------------------------------------- |
| CLI parsing       | `clap`                                       |
| TUI               | `ratatui` + `crossterm`                      |
| Async runtime     | `tokio`                                      |
| Postgres          | `sqlx`                                       |
| Redis             | `redis` or `deadpool-redis`                  |
| Frontmatter parse | `gray_matter`                                |
| TOML              | `toml`                                       |
| HTML templating   | `maud`                                       |
| Markdown to HTML  | `pulldown-cmark`                             |
| Errors            | `thiserror` + `anyhow`                       |
| Logging           | `tracing` + `tracing-subscriber`             |
| Config            | `figment` or `config`                        |
| Language detect   | `whatlang` or `lingua`                       |
| HTTP client (AI)  | `reqwest`                                    |
| Retry/backoff     | `backoff` or `tower`                         |
| Testing           | `assert_cmd`, `sqlx::test`, `testcontainers` |
| Benchmarks        | `criterion`                                  |
| Lint/format       | `rustfmt`, `clippy -D warnings`              |
| Audit             | `cargo-audit`                                |

---

## 18. CI/CD

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo audit`
- **Contract test:** round-trip `commit` → `export` → `commit` is idempotent.
- **Cross-crate test:** `domain` crate compiles with no `infrastructure` deps.
- Release via `cargo-dist`.

---

## 19. Naming & Positioning

- **Name:** `arsip` — grab it on crates.io and GitHub early.
- **Tagline:** _"A terminal-first, file-driven CMS in Rust. Files are the truth, the DB is a mirror, Redis is the cache."_
- **One-line README pitch:** _"arsip indexes markdown into a queryable database, renders it for the web, and never writes back to your files — unless you ask it to."_

---

## 20. The Architectural Contract (Write This in the README)

> _"Files are the source of truth. The database is a materialized view updated only by explicit human commands. `scan` and `diff` tell you what would change; `commit` is the only command that changes it. Redis caches rendered HTML and can be flushed at any time. Nothing is automatic. Nothing is silent. Every decision has a config key with a conservative default."_

---

## 21. The One Sentence to Defend the Whole Design

> _"Git is a better version control system than anything I'd build inside Postgres, so I let git be the truth and treat the DB as a rebuildable index."_

---

## 22. What This Portfolio Demonstrates to a Rust Hiring Manager

- **TyDD:** typestate lifecycle, smart constructors, parse-don't-validate.
- **DDD:** ports and adapters, domain isolated from infrastructure.
- **Easy refactoring:** documented in `docs/refactorings/` with metrics.
- **Rust idioms:** traits, generics, zero-cost abstractions, ownership.
- **Engineering rigor:** CI, tests, benchmarks, `cargo audit`, `doctor`.
- **Architectural judgment:** knowing what _not_ to build (CSS, WYSIWYG, auto-archive, custom DB, DB-as-truth-now).
- **Communication:** every design decision has a written rationale.
- **Multi-consumer proof:** CLI + TUI + portfolio all read the same domain crate.

---

That's the complete design. Every decision is defensible, every tradeoff documented, every refactoring is a story. Write it down, build v0.1, and let the trail accumulate.
