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
deny_doc_warnings := 'false'
rustdocflags := (if deny_doc_warnings == "true" { "-D warnings " } else { "" }) + '--generate-link-to-definition --cfg docsrs -Z unstable-options'

# Generate documentation
doc *FLAGS:
    RUSTDOCFLAGS='{{ rustdocflags }}' cargo +nightly doc {{ doc_flags }} {{ FLAGS }}

alias d := doc

# Run tests and documentation tests
test *FLAGS:
    cargo nextest run --no-tests warn --config-file .nextest.toml --all-features {{ FLAGS }}
    cargo test --doc --all-features

alias t := test

hack *FLAGS:
    cargo hack check --each-feature --all-targets {{ FLAGS }}
