# OpenAPI Field Explorer - Project Specifications

## Description
TUI (Terminal User Interface) in Rust with ratatui that parses an OpenAPI specification and allows searching/analyzing database field usage across all endpoints.

## Tech Stack
- **ratatui**: TUI interface
- **crossterm**: Terminal management
- **serde + serde_json**: JSON parsing
- **tokio**: Async and file watching
- **fuzzy-matcher**: Fuzzy search
- **anyhow**: Error handling

## TUI Interface

### Main Layout
```
┌─ OpenAPI Field Explorer ─────────────────────────────────────────┐
│ Search: USER_ID                                                  │
├─────────────┬─────────────────────┬───────────────────────────────┤
│ Fields      │ Field: USER_ID      │ Endpoints (3)                 │
│ ► USER_ID   │ Type: integer       │ GET /apiHc0/user              │
│   USER_NOM  │ Desc: User ID       │ POST /apiHc0/user             │
│   PAT_ID    │                     │ PUT /apiHc0/user/{id}         │
│   ART_CODE  │ Used in 2 schemas:  │                               │
│   ...       │ • User              │ Impact: 3 endpoints           │
│             │ • Patient (FK)      │ Critical: 1 POST, 1 PUT      │
└─────────────┴─────────────────────┴───────────────────────────────┘
│ F1:Help  F2:Reload  F3:Graph  q:Quit                            │
```

### Panels
- **Search bar**: Top, real-time search
- **Left panel**: List of fields/schemas (navigable)
- **Center panel**: Selected field details
- **Right panel**: List of endpoints using this field
- **Status bar**: Stats and shortcuts at bottom

### Interactive Features
- Keyboard navigation (↑↓ to navigate, Tab to switch panel)
- Real-time search (dynamic filtering)
- ASCII graph of schema relationships
- "Impact analysis" mode with visualization
- Pop-ups for complete endpoint details

### Multiple Views
1. **Fields View**: Navigation by database fields
2. **Schemas View**: Navigation by OpenAPI schemas
3. **Endpoints View**: Navigation by endpoints
4. **Graph View**: Relationship visualization
5. **Stats View**: Dashboard with metrics

## Keyboard Interactions
- `q/Ctrl+C`: Quit
- `Tab`: Switch panel
- `/`: Search mode
- `Enter`: View details/navigate
- `Esc`: Back
- `1-5`: Switch view (Fields/Schemas/Endpoints/Graph/Stats)
- `r`: Reload OpenAPI file
- `h`: Help

## Project Structure
```
field-explorer/
├── src/
│   ├── main.rs           # Entry point + event loop
│   ├── app.rs            # Application state
│   ├── ui/
│   │   ├── mod.rs        # UI modules
│   │   ├── layout.rs     # Main layout
│   │   ├── fields.rs     # Fields view
│   │   ├── schemas.rs    # Schemas view
│   │   ├── endpoints.rs  # Endpoints view
│   │   └── graph.rs      # Graph view
│   ├── parser.rs         # OpenAPI parsing
│   ├── indexer.rs        # Field indexing
│   └── events.rs         # Event handling
├── examples/
└── Cargo.toml
```

## Useful Commands
```bash
# Run the project
cargo run

# Run with specific OpenAPI file
cargo run -- examples/petstore.json

# Linter
cargo clippy

# Formatter
cargo fmt

# Tests
cargo test

# Build release
cargo build --release
```

## Goal
Interactive and intuitive TUI to visually explore relationships between database fields and OpenAPI API endpoints.