## 0.2.1 (2025-11-01)

Added:

- File argument support for Unix-style usage (`tabx file.csv` instead of `cat file.csv | tabx`)
- Named constants for exit codes and detection limits to improve code maintainability

Fixed:

- GitHub release workflow now correctly renames binaries before upload
- Standardized error message format across all modules (removed redundant prefixes)

## 0.2.0 (2025-10-31)

**BREAKING CHANGES:**

- Made `Table` struct fields (`headers` and `rows`) private to prevent invariant violations
- API consumers must now use accessor methods: `headers()`, `rows()`, `into_parts()`

Added:

- Accessor methods for `Table`: `headers()`, `rows()`, and `into_parts()`
- Early delimiter conflict detection - fails fast before processing large files
- Input and output delimiter validation with helpful error messages
- Windows support to CI and release workflows

Changed:

- Migrated from `once_cell` to `std::sync::OnceLock` (no external dependency needed)

Performance:

- Optimized string allocations in all parsers by pre-allocating Vec capacity

## 0.1.1 (2025-10-28)

Security:

- **CRITICAL**: Fixed ReDoS vulnerability in format detection regex patterns that could cause CPU exhaustion with malicious input

Fixed:

- CLI version now correctly reads from `Cargo.toml` instead of being hardcoded

## 0.1.0 (2025-10-28)

Added:

- Initial release of `tabx` CLI tool
- Support for CSV, TSV, Markdown, MySQL, and PostgreSQL table formats
- Auto-detection of input format
- Custom delimiter support
- Shell completion generation (bash, zsh, fish, powershell, elvish)
- Input size limits (100MB) and column limits (10,000) for DoS protection
- Full Unicode support
