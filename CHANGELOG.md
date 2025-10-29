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
