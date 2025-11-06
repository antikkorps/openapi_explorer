# OpenAPI Field Explorer 🔍

A powerful Terminal User Interface (TUI) tool built in Rust that parses OpenAPI specifications and allows interactive exploration of database field usage across all API endpoints.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Status](https://img.shields.io/badge/status-in%20development-yellow.svg)

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Interface](#interface)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Project Structure](#project-structure)
- [Development Status](#development-status)
- [Tech Stack](#tech-stack)
- [Contributing](#contributing)
- [License](#license)

## 🎯 Overview

OpenAPI Field Explorer is an interactive TUI application that helps developers and API designers understand how database fields are used across their OpenAPI-defined endpoints. It provides a visual and intuitive way to:

- **Search** for specific fields across your entire API specification
- **Analyze** field usage and relationships between schemas
- **Visualize** the impact of field changes on endpoints
- **Navigate** through complex API structures with ease
- **Understand** schema dependencies and relationships

## ✨ Features

### Current Features

- ✅ **OpenAPI Parsing**: Full support for OpenAPI 3.x specifications
- ✅ **Field Indexing**: Reverse index mapping fields to schemas and endpoints
- ✅ **Multiple Views**:
  - Fields View (navigation by database fields)
  - Schemas View (schema-centric navigation)
  - Endpoints View (endpoint-centric navigation)
  - Graph View (relationship visualization)
  - Stats View (metrics dashboard)
- ✅ **Interactive Navigation**: Keyboard-driven navigation with Tab and arrow keys
- ✅ **Real-time Search**: Dynamic filtering as you type
- ✅ **Fuzzy Search**: Enhanced search with fuzzy matching using SkimMatcherV2
- ✅ **Field Selection**: Enhanced selection and navigation logic with visual cursor
- ✅ **File Reload**: Manual reload with 'r' key and visual feedback
- ✅ **Help System**: Built-in help popup
- ✅ **Relationship Tracking**: Understand field usage across schemas and endpoints

### In Progress

- 🚧 **Impact Analysis**: Visual representation of field change impact
- 🚧 **File Watching**: Automatic file watching (manual reload currently available)

### Planned Features

- 📋 **Extended Documentation**: User guides and API documentation
- 📋 **Export Capabilities**: Export analysis results
- 📋 **Automatic File Watching**: Real-time reload on file changes

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/antikkorps/openapi_explorer.git
cd openapi_explorer

# Build the project
cargo build --release

# The binary will be available at ./target/release/openapi-explorer
```

### Development Build

```bash
# Build and run in development mode
cargo run

# Run with a specific OpenAPI file
cargo run -- examples/petstore.json
```

## 📖 Usage

### Basic Usage

```bash
# Run with default example
./openapi-explorer

# Run with a specific OpenAPI file
./openapi-explorer path/to/your/openapi.json

# Using cargo run
cargo run -- examples/petstore.json
```

### Quick Start

1. Launch the application with an OpenAPI specification file
2. Use `/` to enter search mode and type a field name
3. Navigate through results with arrow keys (↑/↓)
4. Switch between panels with `Tab`
5. Switch views with number keys (1-5)
6. Press `h` for help
7. Press `q` or `Ctrl+C` to quit

## 🖥️ Interface

The interface is divided into several panels:

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

### Panel Description

- **Top**: Search bar for real-time field filtering
- **Left**: List of fields or schemas (depending on active view)
- **Center**: Detailed information about selected item
- **Right**: Related endpoints using the selected field
- **Bottom**: Status bar with shortcuts and information

## ⌨️ Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `q` or `Ctrl+C` | Quit application |
| `Tab` | Switch between panels |
| `/` | Enter search mode |
| `Enter` | View details / Navigate into item |
| `Esc` | Go back / Exit search mode |
| `↑` / `↓` | Navigate up/down in lists |
| `1` | Switch to Fields View |
| `2` | Switch to Schemas View |
| `3` | Switch to Endpoints View |
| `4` | Switch to Graph View |
| `5` | Switch to Stats View |
| `r` | Reload OpenAPI file |
| `h` | Show help popup |

## 📁 Project Structure

```
openapi_explorer/
├── src/
│   ├── main.rs           # Entry point and event loop
│   ├── app.rs            # Application state management
│   ├── events.rs         # Event handling system
│   ├── parser.rs         # OpenAPI specification parser
│   ├── indexer.rs        # Field indexing and relationship tracking
│   └── ui/               # UI modules
│       ├── mod.rs        # UI module exports
│       ├── layout.rs     # Main layout rendering
│       ├── fields.rs     # Fields view implementation
│       ├── schemas.rs    # Schemas view implementation
│       ├── endpoints.rs  # Endpoints view implementation
│       └── graph.rs      # Graph visualization
├── examples/
│   └── petstore.json     # Sample OpenAPI specification
├── Cargo.toml            # Project dependencies
├── README.md             # This file
├── TODO.md               # Development roadmap
└── AGENTS.md             # Project specifications
```

## 🏗️ Development Status

### Completed Phases

- ✅ **Phase 1**: Project Setup & Dependencies
- ✅ **Phase 2**: Core Data Structures
- ✅ **Phase 3**: OpenAPI Parser
- ✅ **Phase 4**: Field Indexer
- ✅ **Phase 5**: Basic TUI Framework
- ✅ **Phase 6**: UI Components

### Current Phase

- 🚧 **Phase 7**: Views Implementation (90% complete)
- 🚧 **Phase 8**: Search & Navigation (70% complete)
- 🚧 **Phase 9**: Advanced Features (60% complete)

### Upcoming Phases

- 📋 **Phase 10**: Polish & Testing
- 📋 **Phase 11**: Build & Release

For detailed progress, see [TODO.md](TODO.md).

## 🛠️ Tech Stack

- **[ratatui](https://github.com/ratatui-org/ratatui)** - Terminal UI framework
- **[crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal manipulation
- **[serde](https://serde.rs/)** + **[serde_json](https://github.com/serde-rs/json)** - JSON parsing and serialization
- **[tokio](https://tokio.rs/)** - Async runtime for file watching
- **[fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher)** - Fuzzy search implementation
- **[anyhow](https://github.com/dtolnay/anyhow)** - Error handling
- **[notify](https://github.com/notify-rs/notify)** - File system notifications
- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parsing

## 🧪 Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

### Building Release Version

```bash
cargo build --release
```

## 🤝 Contributing

Contributions are welcome! This project is currently in active development.

### Current Priority Tasks

1. Add field selection and navigation logic
2. Implement fuzzy search functionality
3. Complete Fields view interaction
4. Add comprehensive testing

Please check [TODO.md](TODO.md) for the complete list of planned features and improvements.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 👤 Author

Franck - [GitHub](https://github.com/antikkorps)

## 🙏 Acknowledgments

- Inspired by the need for better OpenAPI specification analysis tools
- Built with the amazing Rust ecosystem
- Special thanks to the ratatui and crossterm communities

## 📞 Support

If you encounter any issues or have questions:

- Check existing [GitHub Issues](https://github.com/antikkorps/openapi_explorer/issues)
- Create a new issue with detailed information
- Refer to [TODO.md](TODO.md) for known limitations

---

**Note**: This project is in active development. Features and interfaces may change.
