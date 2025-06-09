# List available recipes
list:
    @just --list --justfile {{ source_file() }}

fmt_flags := '--all'

# Run formatter
fmt *FLAGS:
    cargo +nightly fmt {{ fmt_flags }} {{ FLAGS }}

check_flags := '--all-features --all-targets'

# Check compilation of Rust code
check *FLAGS:
    cargo check {{ check_flags }} {{ FLAGS }}

alias c := check

# Check compilation of Rust code and catch common mistakes
clippy *FLAGS:
    cargo clippy {{ check_flags }} {{ FLAGS }}

alias cl := clippy

doc_flags := '--all-features'

# Generate documentation
doc *FLAGS:
    cargo doc {{ doc_flags }} {{ FLAGS }}

alias d := doc

# Run tests and documentation tests
test *FLAGS:
    cargo nextest run --no-tests warn --config-file .nextest.toml {{ FLAGS }}
    cargo test --doc

alias t := test
