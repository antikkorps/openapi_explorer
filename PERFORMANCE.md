# Performance Guide

## Optimization Techniques Implemented

### 1. Build Optimizations (Cargo.toml)

```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization, slower compile
strip = true           # Strip symbols for smaller binary
panic = "abort"        # Smaller binary size
```

### 2. Memory Optimizations

#### Pre-allocated Vectors
- Used `Vec::with_capacity()` to pre-allocate memory for search results
- Estimated capacity based on data size (25% match rate assumption)
- Reduces reallocation during filtering operations

#### Unstable Sorting
- Used `sort_unstable()` instead of `sort()` where order of equal elements doesn't matter
- ~20-30% faster than stable sort for large datasets

### 3. Search Optimizations

#### Fast Path for Empty Queries
- Separate code path for when search query is empty
- Avoids fuzzy matching overhead when not needed
- Direct key collection instead of scoring

#### Fuzzy Matching Efficiency
- Single matcher instance per filter update
- Batch processing with `extend()` instead of individual pushes
- Early filtering with `filter_map()` to avoid unnecessary allocations

### 4. Index Building Optimizations

#### Logging with Levels
- Debug logs for major operations
- Trace logs for detailed iteration
- Allows profiling without performance impact in production

### 5. UI Rendering Optimizations

#### Minimal Redraws
- 250ms tick rate balances responsiveness and CPU usage
- Event-driven rendering instead of constant polling
- Conditional rendering based on app state

## Performance Benchmarks

### Expected Performance

| Operation | Time (ms) | Notes |
|-----------|-----------|-------|
| Small OpenAPI (< 50 schemas) | < 100 | Initial load |
| Medium OpenAPI (50-200 schemas) | 100-500 | Initial load |
| Large OpenAPI (> 200 schemas) | 500-2000 | Initial load |
| Fuzzy Search | < 50 | Per keystroke |
| View Switch | < 10 | Between views |
| Reload | Same as initial load | - |

### Memory Usage

| OpenAPI Size | Estimated Memory |
|--------------|------------------|
| Small | 5-20 MB |
| Medium | 20-100 MB |
| Large | 100-500 MB |

## Performance Tips for Users

### 1. Use Release Builds

```bash
cargo build --release
./target/release/openapi-explorer examples/petstore.json
```

Release builds are **significantly faster** than debug builds (10x-100x).

### 2. Large OpenAPI Files

For very large OpenAPI specifications:
- Initial parsing may take a few seconds
- Use the reload feature (`r` key) sparingly
- Consider splitting very large specs into multiple files

### 3. Search Performance

- Fuzzy search is optimized for interactive use
- Results are sorted by relevance (best matches first)
- Search is case-insensitive for better UX

### 4. Memory Management

The application uses Rust's ownership system for memory safety:
- No garbage collection overhead
- Predictable memory usage
- No memory leaks (enforced by compiler)

## Profiling

### CPU Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile the application
cargo flamegraph --bin openapi-explorer -- examples/petstore.json
```

### Memory Profiling

```bash
# Install valgrind
sudo apt-get install valgrind

# Profile memory usage
valgrind --tool=massif ./target/release/openapi-explorer examples/petstore.json
```

### Performance Logging

Enable debug logging to see performance metrics:

```bash
RUST_LOG=debug ./target/release/openapi-explorer examples/petstore.json
```

## Future Optimizations

### Planned Improvements

1. **Lazy Loading**
   - Load schemas on-demand instead of all at once
   - Reduce initial load time for large specs

2. **Caching**
   - Cache fuzzy search results
   - Memoize frequently accessed data

3. **Parallel Processing**
   - Use rayon for parallel schema indexing
   - Parallel fuzzy matching for large datasets

4. **Incremental Updates**
   - Only re-index changed parts on reload
   - Faster reload times

5. **Custom Allocator**
   - Consider jemalloc for better allocation patterns
   - Potential 10-15% performance improvement

## Benchmarking

Run benchmarks with:

```bash
cargo bench
```

(Note: Benchmark suite to be added in future versions)

## Known Bottlenecks

1. **Initial OpenAPI Parsing**
   - JSON parsing is I/O bound
   - Can be slow for very large files (> 10 MB)

2. **Schema Reference Resolution**
   - Recursive reference resolution can be slow
   - Deep nesting impacts performance

3. **Real-time Fuzzy Search**
   - Becomes slower with > 1000 fields
   - May need debouncing for very large specs

## Contributing Performance Improvements

When contributing performance improvements:

1. Profile first - measure, don't guess
2. Provide benchmarks showing improvement
3. Don't sacrifice readability for marginal gains
4. Document trade-offs clearly

---

Last updated: 2024-11-06
