## <img src="metron.svg" alt="Metron Logo" width="120"/> Metron - a simple time tracker

A modular, cross-platform time-tracking app for managing multiple work categories, sessions, and tags, with robust analysis and error handling.

## Core Concept

Track work sessions across user-defined categories, with support for tags, time rounding, and detailed analytics. Enforce weekly work limits per category and overall.

## Functional Requirements

User defines a _total weekly quota_ in hours (e.g., 40h, 50h).

Users can create multiple main _categories_ (e.g., "Arbeit 1", "Arbeit 2").
Each category has a user-defined _category weekly quota_.
The sum of all _category weekly quotas_ must not exceed the _total weekly quota_.
No new categories can be created if the _total weekly quota_ is exhausted.

Up to 7 **tags** can be created.
One tap per session is required, but a session can have multiple tags.

_Sessions_ are tracked in 15-minute _timeblocks_ (15min, 30min, 45min...etc.).
Each session requires:

- Title (string)
- Category (string)
- Time unit (integer, multiple of 15)
- Tag (string)

If a session is interrupted early, round down to the nearest _timeblock_:
eg. 15min → 0min, 13min → 0min, 25min → 15min, 32min → 30min etc.
_Timeblocks_ up until the _category*weekly_quota*_ are counted as _work time_. _Timeblocks_ that exceed the _category weekly quota_ are counted as _overtime_.

Provide _analytics_ for daily, weekly, monthly, and yearly periods.
Session titles are not included in analytics.
Analytics look at the sum of work time, overtime, total_time (work time plus overtime), for each category and for all categories together.

## Technical Requirements

Target Platforms:

- CLI (primary)
- Optional: Standalone GUI web app

Languages & Frameworks:

- CLI/Desktop: Rust (preferred) or Nim
- Web: TypeScript with Tauri (and a framework like React)

Data Storage:

- Local JSON files (for CLI/desktop)

```json
Session: { "id": 1, "title": "Building the cli for the app", "category": "Arbeit 1", "tag": "Build", "start": "2025-08-08T09:00", "end": "2025-08-08T09:45", "duration": 45 }
Category: { "name": "Arbeit 1", "category_weekly_quota": 10 }
Tag: { "name": "Build" }
```

## Modules

CategoryManager: CRUD for categories, enforce limits
TagManager: CRUD for tags, enforce tag limit
SessionManager: Create, round, and validate sessions
Analysis: Generate reports and overtime tracking
Storage: Read/write JSON data
CLI/UI: User interaction layer

## Input/Output:

- CLI: Text prompts and outputs
- Web: JSON API, interactive UI

## Error Handling & Edge Cases:

- Reject negative or zero durations
- Prevent overlapping sessions
- Validate required fields (category: name & category_weekly_quota; session: title, category, duration, tag)
- Handle corrupted/missing data files gracefully

## Testing:

Unit tests for all logic (e.g., time rounding, session creation)
Integration tests for data flow and storage
CLI command tests
(Optional) End-to-end tests for web UI

# metron
