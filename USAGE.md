# Metron CLI

A modular, cross-platform time-tracking CLI application written in Rust.

## Features

- ✅ Category management with weekly quotas
- ✅ Tag management (up to 7 tags)
- ✅ Session tracking in 15-minute intervals
- ✅ Time rounding for interrupted sessions
- ✅ Analytics with worktime vs overtime tracking
- ✅ JSON data storage
- ✅ Quota enforcement

## Installation

### Build from Source

```bash
# Clone the repository
cd /path/to/metron

# Build in release mode
cargo build --release

# The binary will be available at ./target/release/metron
```

### Add to PATH (optional)

```bash
# Copy to a directory in your PATH
sudo cp target/release/metron /usr/local/bin/

# Or create a symlink
ln -s $(pwd)/target/release/metron ~/.local/bin/metron
```

## Usage

### Basic Commands

```bash
# Show help
metron --help

# Set total weekly quota (e.g., 40 hours)
metron set-quota 40

# Create categories with quotas
metron category create "Development" --quota 20
metron category create "Meetings" --quota 10
metron category list

# Create tags
metron tag create "Coding"
metron tag create "Planning"
metron tag list

# Start work sessions (duration must be multiple of 15)
metron session start "Building CLI" "Development" --tags Coding --duration 60
metron session start "Daily standup" "Meetings" --tags Planning --duration 30
metron session list

# Generate analytics
metron analysis --period week
metron analysis --period month --category "Development"
```

### Advanced Usage

```bash
# Update category quota
metron category update "Development" --quota 25

# Delete sessions (by ID or partial ID)
metron session delete 51312924

# Delete categories or tags (with confirmation)
metron category delete "Old Category"
metron tag delete "Unused Tag"

# Different analysis periods
metron analysis --period day     # Today's work
metron analysis --period week    # This week
metron analysis --period month   # This month
metron analysis --period year    # This year
```

## Data Storage

All data is stored in `metron_data.json` in the current directory. The file contains:

- Categories with weekly quotas
- Tags (up to 7)
- Work sessions with timestamps
- Total weekly quota setting

## Time Tracking Rules

1. **Time Blocks**: All sessions are tracked in 15-minute increments (15, 30, 45, 60, etc.)
2. **Rounding**: Interrupted sessions are rounded down to the nearest 15 minutes
3. **Quotas**: Category quotas cannot exceed the total weekly quota
4. **Work time vs Overtime**: Time within category quotas counts as work time, excess as overtime
5. **Tags**: Optional, up to 7 total tags, multiple tags per session allowed

## Examples

```bash
# Complete workflow example
metron set-quota 40
metron category create "Project A" --quota 20
metron category create "Project B" --quota 15
metron tag create "Development"
metron tag create "Testing"

# Work session
metron session start "Feature implementation" "Project A" --tags Development --duration 120

# Check progress
metron analysis --period week
```

## Data Format

The JSON schema follows this structure:

```json
{
  "categories": [
    {
      "name": "Project A",
      "category_weekly_quota": 20
    }
  ],
  "tags": [
    {
      "name": "Development"
    }
  ],
  "sessions": [
    {
      "id": "uuid",
      "title": "Feature implementation",
      "category": "Project A",
      "tags": ["Development"],
      "start": "2025-08-08T10:20:55.934Z",
      "end": "2025-08-08T12:20:55.934Z",
      "duration": 120
    }
  ],
  "total_weekly_quota": 40
}
```

## Error Handling

The CLI provides clear error messages for common issues:

- Invalid duration (not multiple of 15)
- Category/tag not found
- Quota exceeded
- Duplicate names
- Storage errors

## Development

Built with:

- Rust 2021 edition
- Clap 4.x for CLI parsing
- Serde for JSON serialization
- Chrono for date/time handling
- UUID for session IDs
