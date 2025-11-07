# Testing Guide

This document provides comprehensive testing instructions for OpenAPI Field Explorer.

## Quick Test

```bash
# Build and run with example
cargo run --release -- examples/petstore.json

# Or use the installed binary
openapi-explorer examples/petstore.json
```

## Testing Checklist

### Core Functionality

- [ ] **OpenAPI Parsing**
  - [ ] Valid OpenAPI 3.x spec loads without errors
  - [ ] Invalid spec shows appropriate error messages
  - [ ] Large specs (>100 endpoints) load within reasonable time

- [ ] **Navigation**
  - [ ] Arrow keys navigate lists smoothly
  - [ ] Tab switches between panels correctly
  - [ ] Enter selects items properly
  - [ ] Esc returns to previous state

- [ ] **Views** (Press 1-5)
  - [ ] Fields View (1): Shows all fields, allows navigation
  - [ ] Schemas View (2): Displays schemas correctly
  - [ ] Endpoints View (3): Lists all endpoints
  - [ ] Graph View (4): Renders relationships
  - [ ] Stats View (5): Shows accurate statistics

- [ ] **Search**
  - [ ] Type to search works in Fields View
  - [ ] Fuzzy matching finds partial matches
  - [ ] Backspace removes characters
  - [ ] Search updates results in real-time

- [ ] **Features**
  - [ ] Help popup (h) displays correctly
  - [ ] Reload (r) reloads the file
  - [ ] Endpoint details (Enter in Right panel) shows full info
  - [ ] Validation warnings appear in Stats View

## Testing with Real-World Specs

### 1. Swagger Petstore (Included)

```bash
openapi-explorer examples/petstore.json
```

**What to test**:
- Basic navigation
- Field search (try "id", "name", "status")
- Endpoint details
- Stats view validation

**Expected**:
- 3-5 schemas
- 8-15 endpoints
- Clean validation (no critical errors)

### 2. GitHub API (Large)

Download the GitHub OpenAPI spec:
```bash
curl -o github-api.json https://raw.githubusercontent.com/github/rest-api-description/main/descriptions/api.github.com/api.github.com.json

openapi-explorer github-api.json
```

**What to test**:
- Performance with large spec (>600 endpoints)
- Search performance
- Memory usage
- Validation warnings (expect some due to size)

**Expected**:
- 200+ schemas
- 600+ endpoints
- Loading time <3 seconds on modern hardware

### 3. Stripe API (Complex)

Download Stripe's OpenAPI spec:
```bash
curl -o stripe-api.json https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json

openapi-explorer stripe-api.json
```

**What to test**:
- Complex schema relationships
- Deep nesting
- Field type diversity
- Validation accuracy

**Expected**:
- 500+ schemas
- 300+ endpoints
- Rich field metadata

### 4. Custom/Company API

Test with your own OpenAPI specification:
```bash
openapi-explorer path/to/your/openapi.json
```

**What to verify**:
- All your schemas appear
- Field search finds your database fields
- Endpoint details are accurate
- Validation catches real issues

## Performance Benchmarks

### Load Times (Release Build)

| Spec Size | Schemas | Endpoints | Load Time |
|-----------|---------|-----------|-----------|
| Small     | <10     | <20       | <100ms    |
| Medium    | 10-100  | 20-200    | 100-500ms |
| Large     | 100-500 | 200-600   | 500-2000ms|
| X-Large   | >500    | >600      | 2-5s      |

### Memory Usage

| Spec Size | Memory  |
|-----------|---------|
| Small     | 5-10 MB |
| Medium    | 10-50 MB|
| Large     | 50-150 MB|
| X-Large   | 150-500 MB|

## Regression Testing

Run after each change:

```bash
# 1. Unit tests
cargo test

# 2. Integration tests
cargo test --test integration_test

# 3. Build check
cargo build --release

# 4. Clippy (no warnings)
cargo clippy -- -D warnings

# 5. Format check
cargo fmt -- --check

# 6. Quick manual test
cargo run --release -- examples/petstore.json
```

## Known Limitations

1. **YAML Specs**: Currently only JSON format is fully supported
2. **OpenAPI 2.0**: Not supported (OpenAPI 3.x only)
3. **Very Large Specs** (>1000 endpoints): May be slow on initial load
4. **Circular References**: May cause deep recursion (limited by Rust stack)

## Reporting Issues

When reporting bugs, include:

1. OpenAPI spec (if possible) or characteristics (size, complexity)
2. Rust version: `rustc --version`
3. OS and version
4. Steps to reproduce
5. Expected vs actual behavior
6. Screenshots if UI-related

## Test Scenarios

### Scenario 1: Field Impact Analysis

1. Open any spec: `openapi-explorer spec.json`
2. Press `/` and search for a common field (e.g., "id")
3. Press Enter to select
4. Navigate to Right panel (Tab)
5. Observe all endpoints using this field
6. Press Enter on an endpoint to see full details

**Expected**: Complete traceability from field to all usage points.

### Scenario 2: API Quality Check

1. Open your spec: `openapi-explorer your-api.json`
2. Press `5` for Stats View
3. Scroll down to validation warnings
4. Note any quality issues

**Expected**: Actionable validation feedback.

### Scenario 3: Schema Exploration

1. Press `2` for Schemas View
2. Navigate through schemas with arrows
3. Press Enter on a schema
4. See all fields in that schema

**Expected**: Clear schema structure visualization.

### Scenario 4: Performance Test

1. Download GitHub API spec (see above)
2. Run: `time openapi-explorer github-api.json`
3. Immediately press `5` for Stats
4. Note load time and responsiveness

**Expected**: <5s load, smooth UI even with 600+ endpoints.

## Continuous Testing

The CI pipeline automatically tests:
- ✅ Linux (Ubuntu latest)
- ✅ macOS (Intel and Apple Silicon)
- ✅ Windows (MSVC)
- ✅ Rust stable and MSRV (1.70.0)

See `.github/workflows/ci.yml` for details.

## Manual Cross-Platform Testing

### Linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu
./target/x86_64-unknown-linux-gnu/release/openapi-explorer examples/petstore.json
```

### macOS
```bash
cargo build --release --target x86_64-apple-darwin
./target/x86_64-apple-darwin/release/openapi-explorer examples/petstore.json
```

### Windows
```powershell
cargo build --release --target x86_64-pc-windows-msvc
.\target\x86_64-pc-windows-msvc\release\openapi-explorer.exe examples/petstore.json
```

## Profiling

### CPU Profiling
```bash
# Install flamegraph
cargo install flamegraph

# Profile with large spec
cargo flamegraph --bin openapi-explorer -- path/to/large-spec.json

# View flamegraph.svg in browser
```

### Memory Profiling
```bash
# Install valgrind (Linux)
sudo apt-get install valgrind

# Profile memory usage
valgrind --tool=massif --massif-out-file=massif.out \
  ./target/release/openapi-explorer examples/petstore.json

# Analyze results
ms_print massif.out
```

## Test Automation

Future test automation could include:
- [ ] Screenshot comparison tests
- [ ] Automated UI testing with expect scripts
- [ ] Benchmark suite with criterion
- [ ] Fuzz testing for parser
- [ ] Property-based testing

---

Last updated: 2025-11-06
