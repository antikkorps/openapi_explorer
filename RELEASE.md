# Release Guide

This document provides instructions for creating and publishing releases of OpenAPI Field Explorer.

## Pre-Release Checklist

Before creating a new release, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] CHANGELOG.md is updated with all changes
- [ ] Version number is bumped in `Cargo.toml`
- [ ] README.md reflects current feature status
- [ ] All documentation is up to date

## Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR** version (X.0.0): Breaking changes, major feature overhauls
- **MINOR** version (0.X.0): New features, improvements, non-breaking changes
- **PATCH** version (0.0.X): Bug fixes, documentation updates

## Release Process

### 1. Update Version Number

Edit `Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this line
```

### 2. Update CHANGELOG.md

Move items from `[Unreleased]` to a new version section:

```markdown
## [0.2.0] - 2025-11-06

### Added
- List of new features

### Fixed
- List of bug fixes
```

Update the comparison links at the bottom:

```markdown
[Unreleased]: https://github.com/antikkorps/openapi_explorer/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/antikkorps/openapi_explorer/compare/v0.1.0...v0.2.0
```

### 3. Build and Test

```bash
# Clean build
cargo clean

# Build release binary
cargo build --release

# Run all tests
cargo test --all-features

# Test the binary
./target/release/openapi-explorer examples/petstore.json
```

### 4. Test with Real-World Specs

```bash
# Test with GitHub API spec
curl -o github-api.json https://raw.githubusercontent.com/github/rest-api-description/main/descriptions/api.github.com/api.github.com.json
./target/release/openapi-explorer github-api.json

# Test with Stripe API spec
curl -o stripe-api.json https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json
./target/release/openapi-explorer stripe-api.json
```

### 5. Commit Version Bump

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"
git push origin main
```

### 6. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release v0.2.0: Quick Wins & Bug Fixes"

# Push tag to trigger CI/CD
git push origin v0.2.0
```

### 7. GitHub Actions Automation

Once you push the tag, GitHub Actions will automatically:

1. **Build binaries** for all platforms:
   - Linux (x86_64 glibc)
   - Linux (x86_64 musl)
   - macOS (x86_64 Intel)
   - macOS (aarch64 Apple Silicon)
   - Windows (x86_64 MSVC)

2. **Create GitHub Release** with:
   - Release notes from CHANGELOG.md
   - Downloadable binary archives (.tar.gz for Linux/macOS, .zip for Windows)
   - Checksums for verification

3. **Run full test suite** on all platforms

### 8. Verify Release

After GitHub Actions completes:

1. Go to https://github.com/antikkorps/openapi_explorer/releases
2. Verify the new release is published
3. Download and test binaries for each platform
4. Verify release notes are correct

### 9. Announce Release (Optional)

- Post to project discussion board
- Tweet/social media announcement
- Update project website if applicable
- Notify stakeholders

## Manual Release (If CI/CD Fails)

If GitHub Actions fails or you need to create a manual release:

### Build All Platforms

```bash
# Linux x86_64 (glibc)
cargo build --release --target x86_64-unknown-linux-gnu
strip target/x86_64-unknown-linux-gnu/release/openapi-explorer
tar czf openapi-explorer-v0.2.0-linux-x86_64.tar.gz \
  -C target/x86_64-unknown-linux-gnu/release openapi-explorer

# Linux x86_64 (musl)
cargo build --release --target x86_64-unknown-linux-musl
strip target/x86_64-unknown-linux-musl/release/openapi-explorer
tar czf openapi-explorer-v0.2.0-linux-x86_64-musl.tar.gz \
  -C target/x86_64-unknown-linux-musl/release openapi-explorer

# macOS x86_64
cargo build --release --target x86_64-apple-darwin
strip target/x86_64-apple-darwin/release/openapi-explorer
tar czf openapi-explorer-v0.2.0-macos-x86_64.tar.gz \
  -C target/x86_64-apple-darwin/release openapi-explorer

# macOS aarch64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin
strip target/aarch64-apple-darwin/release/openapi-explorer
tar czf openapi-explorer-v0.2.0-macos-aarch64.tar.gz \
  -C target/aarch64-apple-darwin/release openapi-explorer

# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc
zip openapi-explorer-v0.2.0-windows-x86_64.zip \
  target/x86_64-pc-windows-msvc/release/openapi-explorer.exe
```

### Create GitHub Release Manually

1. Go to https://github.com/antikkorps/openapi_explorer/releases/new
2. Tag: `v0.2.0`
3. Title: `OpenAPI Field Explorer v0.2.0`
4. Description: Copy from CHANGELOG.md
5. Upload all binary archives
6. Mark as latest release
7. Publish

## Publishing to crates.io (Optional)

If you want to publish to the Rust package registry:

```bash
# Login to crates.io (one-time)
cargo login

# Dry run to check everything
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

**Note**: You cannot unpublish versions from crates.io, only yank them. Be sure before publishing.

## Post-Release

### 1. Update Development Branch

```bash
# Merge release commit back to develop if using git-flow
git checkout develop
git merge main
```

### 2. Create Next Version in CHANGELOG.md

Add a new `[Unreleased]` section:

```markdown
## [Unreleased]

### Added
-

### Changed
-

### Fixed
-
```

### 3. Monitor for Issues

- Watch GitHub Issues for bug reports
- Monitor installation script usage
- Check CI/CD for any failures
- Respond to user feedback

## Hotfix Process

For critical bugs in production:

1. Create hotfix branch from tag: `git checkout -b hotfix/v0.2.1 v0.2.0`
2. Fix the bug and commit
3. Update version to 0.2.1 in Cargo.toml
4. Update CHANGELOG.md with fix
5. Merge to main and create tag v0.2.1
6. CI/CD will build and release automatically
7. Merge hotfix back to develop

## Rollback

If a release has critical issues:

1. **Yank from crates.io** (if published): `cargo yank --vers 0.2.0`
2. **Edit GitHub Release**: Mark as "pre-release" or delete
3. **Communicate**: Post issue explaining the problem
4. **Fix and re-release**: Create new patch version

## Release Schedule

Suggested schedule:

- **Patch releases**: As needed for critical bugs
- **Minor releases**: Monthly or when significant features are ready
- **Major releases**: When breaking changes are necessary

## Release Checklist Summary

```markdown
- [ ] All tests pass
- [ ] Code formatted and linted
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Documentation updated
- [ ] Release binary tested
- [ ] Real-world spec testing complete
- [ ] Version commit pushed
- [ ] Git tag created and pushed
- [ ] GitHub Actions completed successfully
- [ ] Release verified on GitHub
- [ ] Binaries tested on target platforms
- [ ] Announcement made (optional)
- [ ] Development branch updated
```

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Cargo Release Guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [GitHub Releases](https://docs.github.com/en/repositories/releasing-projects-on-github)

---

For questions or issues with the release process, open an issue or contact the maintainers.
