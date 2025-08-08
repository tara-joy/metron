Mission

Create a CLI first. Optionally add a standalone GUI/web UI later (Tauri + TypeScript). Store all data locally as JSON. Respect XDG defaults. Work offline. Fail loudly on programmer errors. Fail gracefully on user errors.

References:

    Rust: https://www.rust-lang.org/

    Serde (JSON): https://serde.rs/

    chrono / time: https://docs.rs/chrono/latest/chrono/ or https://time.rs/

    CLI parsing (clap): https://docs.rs/clap/latest/clap/

    XDG spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html

    JSON Schema guide: https://json-schema.org/learn/

    Tauri (optional UI): https://tauri.app/

Acceptance criteria (must satisfy)

    CLI that implements all functional requirements below.

    Robust unit tests for rounding, overlap detection, quota logic.

    Integration tests around storage and analytics.

    Clear JSON schema + sample dataset.

    Atomic writes and file locking for storage.

    Configurable week start (default Monday). Use ISO week semantics unless user config overrides.

    Produce human-readable reports: daily/weekly/monthly/yearly. Also export CSV/JSON.

    Provide README + CLI help text + short examples.

Data model (strict JSON schema summary)

Use RFC3339 timestamps (ISO8601). Store durations in minutes. Keep raw_minutes and recorded_minutes.

Top-level file structure (file versioned):

{
"version": 1,
"meta": { "total_weekly_quota_minutes": 2400, "week_start": "monday" },
"categories": [ ... ],
"tags": [ ... ],
"sessions": [ ... ]
}

Session schema (example):

{
"id": "uuid",
"title": "Build CLI",
"category_id": "uuid",
"tags": ["Build", "Design"],
"start": "2025-08-08T09:00:00+02:00",
"end": "2025-08-08T09:45:00+02:00",
"raw_minutes": 45,
"blocks": 3,
"recorded_minutes": 45, // blocks \* 15
"state": "completed", // running | completed | aborted
"created_at": "...",
"updated_at": "..."
}

Category schema:

{ "id":"uuid","name":"Arbeit 1","weekly_quota_minutes":600 }

Tag schema:

{ "id":"uuid","name":"Build" } // max 7 tags total

Store raw and recorded values. Never throw away raw_minutes.
Core rules (unambiguous)

    Quotas

        total_weekly_quota is the user's overall planned hours for a week.

        Each category has category_weekly_quota.

        Sum(category_weekly_quotas) must not exceed total_weekly_quota.

        Category creation/editing must enforce that invariant. If violation, reject with clear error.

    Tags

        Max 7 tags across the account.

        A session can have multiple tags from that set.

    Time and rounding

        Measure exact raw_minutes = floor((end - start) in seconds / 60).

        Compute blocks = floor(raw_minutes / 15).

        recorded_minutes = blocks * 15.

        Save both raw_minutes and recorded_minutes.

        If blocks == 0, still save the session but mark recorded_minutes = 0 and short_session = true.

        When a running session is aborted by the user, apply same rounding and save (state = aborted).

    Overlap prevention

        New sessions cannot overlap any existing session's raw interval (start..end).

        Overlap check uses raw intervals, not recorded minutes.

        If overlap detected, reject and show overlapping session IDs.

    Overtime calculation

        For each category, work_time = min(sum(recorded_minutes_for_category), category_weekly_quota).

        overtime = sum(recorded_minutes_for_category) - category_weekly_quota (zero if negative).

        total_time = work_time + overtime.

        Category quotas are planned. Sessions beyond quota are allowed and counted as overtime.

    Boundary crossing

        If a session crosses day/week/month/year boundaries, allocate recorded_minutes to periods by chronological order.

        Algorithm: compute minute-by-minute mapping for the recorded range and assign earliest recorded minutes to the earlier period until recorded_minutes are exhausted. This guarantees total recorded minutes preserve rounding and allocation follows wall clock order.

    DST and time zones

        Store timestamps with offsets (RFC3339).

        Convert to local timezone for UX. Use UTC internally for durable computations when comparing ranges.

    Category creation limit

        New category creation is allowed only when sum(existing_category_weekly_quotas) + new_quota <= total_weekly_quota.

        If total_weekly_quota is exhausted (sum equals total), disallow new categories unless the user increases total quota or reduces other quotas.

CLI UX & commands (examples)

Interactive and non-interactive modes required.

Basic commands (use clap style):

metron init --total-weekly 40 --week-start monday
metron category add "Arbeit 1" --weekly 10
metron tag add Build
metron start --title "Build CLI" --category "Arbeit 1" --tags Build
metron stop # stops the running session
metron abort # aborts running session (rounds down and saves)
metron session list --period week --format table
metron report --period weekly --format csv --out report.csv
metron export --format json > backup.json
metron import backup.json

Interactive mode:

    metron start prompts only when required flags missing.

    Show remaining weekly minutes for chosen category before start.

Non-interactive mode:

    All flags provided. Exit codes: 0 success, 2 user error, 3 system error.

Analytics (definitions)

Provide daily, weekly, monthly, yearly reports. Each report must include:

    per-category: work_time_minutes, overtime_minutes, total_minutes

    totals across categories

    per-tag aggregates (same three numbers)

    counts: sessions, short_sessions

    distribution (bar chart friendly): hours per day for period

Week definition: configurable. Default = Monday (ISO).

Example weekly output (table / CSV):

category,work_h,overtime_h,total_h
Arbeit 1,8.0,1.5,9.5
Arbeit 2,4.0,0.0,4.0
TOTAL,12.0,1.5,13.5

Storage & reliability

    Use atomic write (write tmp -> fsync -> rename). Use crate suggestion: fs_atomic or write/rename pattern.

    Use file lock on data file during write (advisory), e.g., fs2 crate or flock. Prevent concurrent writes.

    Keep version and migrations in file.

    On corrupted/missing files: attempt safe recovery from backup.json if present; else show clear error and offer --reset option.

    Backup strategy: keep data.json.bak last N (configurable) snapshots.

Errors & messages (style)

    Short. Direct. Actionable.

    Example: ERR: category quota would exceed total weekly quota (sum=40h, total=40h). Increase total or lower other quotas.

    Example: ERR: session overlaps session id=ab12... (2025-08-08T09:00..09:45).

    Exit codes: 0 ok, 2 user error, 3 system failure.

Tests (must include)

    Unit tests:

        rounding behavior (0–14 -> 0, 15–29 -> 15, etc.)

        overlap detection (adjacent sessions allowed; exact touching allowed? define: touching allowed: end == start ok)

        quota enforcement on category creation/edit

        splitting across week/day boundaries

        DST crossing test case

    Integration tests:

        simulate session start/stop across midnight

        run CLI commands in temp XDG dir

        import/export roundtrip

    E2E (optional for web UI):

        start session -> stop -> generate weekly CSV -> assert sums

Include test data fixtures in tests/fixtures.
Deliverables (what AI must produce)

    Working Rust CLI project skeleton (Cargo.toml, src/, tests/).

    Full JSON schema file schema.json.

    Example data.json with 5 sessions demonstrating edge cases: short session, cross-midnight, overtime, aborted.

    README + CLI examples.

    Unit + integration tests runnable with cargo test.

    (Optional) Tauri web skeleton with API endpoints documented (OpenAPI stub).

Implementation hints (opinionated)

    Use crates: serde, serde_json, chrono or time, clap, directories-next, uuid, thiserror/anyhow, tracing or log.

    Use atomic file writes + file locks.

    Keep binary small. Avoid heavy GUI deps in CLI binary.

    Keep pure logic separated from IO for easy testing.

    Use feature flags for optional GUI.

Example scenarios (sanity)

    total_weekly_quota=40h (2400 min). Add category A=10h, B=20h. Sum=30h. OK.

    Start session 09:00 → 09:13. raw=13, blocks=0, recorded=0. Save short session.

    Start session 09:00 → 09:32. raw=32, blocks=2, recorded=30.

    Session 23:50 → 00:20 next day. raw=30, blocks=2, recorded=30. Allocate first recorded minute chunk to previous day until its minutes exhausted (earliest-first allocation).

Minimal report for reviewers

    Provide cargo test pass log.

    Provide sample metron report --period weekly output.

Rationale — what I changed and why (short)

    Clarified ambiguous rounding.

    Kept raw data for audit.

    Fixed category creation ambiguity.

    Defined overlap detection precisely.

    Added DST / boundary rules.

    Added storage safety and atomic writes.

    Added explicit CLI commands and exit codes.

    Added a solid test plan.

    Added links to key crates and specs.

{
"$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://example.com/metron.schema.json",
"title": "Metron Data File",
"description": "Data structure for the Metron CLI time-tracking app",
"type": "object",
"required": ["version", "meta", "categories", "tags", "sessions"],
"properties": {
"version": {
"type": "integer",
"minimum": 1
},
"meta": {
"type": "object",
"required": ["total_weekly_quota_minutes", "week_start"],
"properties": {
"total_weekly_quota_minutes": {
"type": "integer",
"minimum": 15,
"description": "Overall weekly quota in minutes"
},
"week_start": {
"type": "string",
"enum": ["monday", "sunday", "saturday"],
"description": "Start day of the week"
}
},
"additionalProperties": false
},
"categories": {
"type": "array",
"items": {
"type": "object",
"required": ["id", "name", "weekly_quota_minutes"],
"properties": {
"id": { "type": "string", "format": "uuid" },
"name": { "type": "string", "minLength": 1 },
"weekly_quota_minutes": {
"type": "integer",
"minimum": 15
}
},
"additionalProperties": false
},
"uniqueItems": true
},
"tags": {
"type": "array",
"maxItems": 7,
"items": {
"type": "object",
"required": ["id", "name"],
"properties": {
"id": { "type": "string", "format": "uuid" },
"name": { "type": "string", "minLength": 1 }
},
"additionalProperties": false
},
"uniqueItems": true
},
"sessions": {
"type": "array",
"items": {
"type": "object",
"required": [
"id",
"title",
"category_id",
"tags",
"start",
"end",
"raw_minutes",
"blocks",
"recorded_minutes",
"state",
"created_at",
"updated_at"
],
"properties": {
"id": { "type": "string", "format": "uuid" },
"title": { "type": "string", "minLength": 1 },
"category_id": { "type": "string", "format": "uuid" },
"tags": {
"type": "array",
"items": { "type": "string", "minLength": 1 },
"uniqueItems": true
},
"start": { "type": "string", "format": "date-time" },
"end": { "type": "string", "format": "date-time" },
"raw_minutes": { "type": "integer", "minimum": 0 },
"blocks": { "type": "integer", "minimum": 0 },
"recorded_minutes": { "type": "integer", "minimum": 0 },
"state": {
"type": "string",
"enum": ["running", "completed", "aborted"]
},
"created_at": { "type": "string", "format": "date-time" },
"updated_at": { "type": "string", "format": "date-time" },
"short_session": { "type": "boolean" }
},
"additionalProperties": false
},
"uniqueItems": true
}
},
"additionalProperties": false
}

## CLI Interface Design

metron init Initialize app config and data file metron init --total-weekly 40 --week-start monday
metron category add Add category with quota metron category add "Arbeit 1" --weekly 10
metron category list Show all categories metron category list
metron category rm Remove category metron category rm "Arbeit 1"
metron tag add Add tag metron tag add Build
metron tag list List tags metron tag list
metron tag rm Remove tag metron tag rm Build
metron start Start session metron start --title "Build CLI" --category "Arbeit 1" --tags Build,Docs
metron stop Stop running session metron stop
metron abort Abort running session metron abort
metron session list List sessions in period metron session list --period week --format table
metron report Generate analytics report metron report --period month --format csv
metron export Export all data metron export --format json > backup.json
metron import Import from file metron import backup.json
metron config View or change config metron config set total-weekly 45
