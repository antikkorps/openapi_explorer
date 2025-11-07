# Changelog

All notable changes to OpenAPI Field Explorer will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Installation scripts for Linux/macOS (`install.sh`) and Windows (`install.ps1`)
- Comprehensive CHANGELOG documentation
- GitHub Actions CI/CD workflow (pending)

## [0.2.0] - 2025-11-06

### Added - Quick Wins & Polish
- **Enhanced Statistics Dashboard**: Comprehensive stats view with field type distribution, HTTP method breakdown, top fields usage, and validation warnings
- **Enriched Help System**: Detailed help popup with organized sections, tips, and usage examples
- **Loading Indicators**: Real-time feedback with contextual messages during file reload operations
- **Endpoint Details Popup**: Complete endpoint documentation viewer with parameters, request body, responses, and color-coded status codes
- **OpenAPI Validation**: Automatic spec validation with 7 types of checks (missing components, unknown types, unused schemas, missing descriptions, etc.)
- **Validation Warnings Display**: Visual warnings in Stats View with numbering and color-coding

### Fixed - Critical Bugs
- **Bounds Checking**: Fixed potential panics when lists become empty during filtering
- **Array Access Safety**: Replaced direct indexing with `.get()` for safe bounds-checked access
- **Navigation Consistency**: Fixed asymmetry between `navigate_up` and `navigate_down` in Right panel
- **Empty List Handling**: Explicit reset of cursor state to 0 when filtered lists are empty

### Changed
- Magic number (25% match rate) extracted to documented constant `FUZZY_SEARCH_MATCH_RATE`
- README feature status updated to reflect actual implementation
- Documentation dates corrected to ISO 8601 format (YYYY-MM-DD)
- Help popup enlarged to fit comprehensive content
- Stats View now shows validation results prominently

## [0.1.0] - 2025-11-06

### Added - Core Features

#### Infrastructure (Phases 1-6)
- Complete Rust project setup with Cargo
- Core data structures for OpenAPI parsing
- Field indexing and relationship tracking
- Basic TUI framework with ratatui
- Main UI components (search bar, panels, status bar)

#### Views & Navigation (Phases 7-8)
- **5 View Modes**:
  - Fields View: Navigate by database field names
  - Schemas View: Browse by OpenAPI schemas
  - Endpoints View: Explore API endpoints
  - Graph View: Visualize relationships
  - Stats View: Metrics dashboard
- **Fuzzy Search**: Real-time fuzzy matching with SkimMatcherV2 and relevance scoring
- **Visual Navigation**: Cursor indicator (►) showing current position
- **Color Coding**: Selected items (yellow/bold) vs cursor position (cyan)
- **Keyboard Shortcuts**: Complete keyboard-driven interface
  - `↑/↓`: Navigate lists
  - `Tab`: Switch panels
  - `Enter`: Select items
  - `1-5`: Switch views
  - `r`: Reload file
  - `h`: Help
  - `q`: Quit

#### Advanced Features (Phases 9-10)
- **File Reload**: Manual reload with 'r' key and visual feedback (⟳ symbol)
- **Error Handling**: Comprehensive error capture and display in status bar
- **Logging System**: Debug and trace level logs for indexer and parser
- **Performance Optimizations**:
  - Release build with LTO and optimization level 3
  - Pre-allocation with `Vec::with_capacity()`
  - Unstable sorting for 20-30% performance gain
  - Fast path for empty search queries
- **Testing Suite**:
  - Unit tests for core components (parser, indexer)
  - Integration tests (tests/integration_test.rs)
  - Library module (src/lib.rs) for external testing

#### Documentation
- Comprehensive README.md with installation, usage, and keyboard shortcuts
- PERFORMANCE.md with optimization techniques and profiling guide
- TODO.md tracking development progress
- AGENTS.md with project specifications

### Changed
- Project structure reorganized for better maintainability
- Parameter parsing fixed with `#[serde(rename = "in")]` for `in_` field
- Search algorithm upgraded from simple substring to fuzzy matching
- Filtering now sorts results by relevance score

### Performance
- 10x-100x faster in release mode vs debug builds
- ~20-30% faster sorting operations with unstable sort
- Reduced memory allocations during search
- Efficient iterator chains and minimal cloning

## [0.0.1] - 2025-11-05

### Added
- Initial project setup
- Basic OpenAPI parsing
- Simple field indexing
- Proof of concept TUI

---

## Release Types

- **Major** (x.0.0): Breaking changes, major feature overhauls
- **Minor** (0.x.0): New features, improvements, non-breaking changes
- **Patch** (0.0.x): Bug fixes, documentation updates

## Categories

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security updates

---

[Unreleased]: https://github.com/antikkorps/openapi_explorer/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/antikkorps/openapi_explorer/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/antikkorps/openapi_explorer/compare/v0.0.1...v0.1.0
[0.0.1]: https://github.com/antikkorps/openapi_explorer/releases/tag/v0.0.1
