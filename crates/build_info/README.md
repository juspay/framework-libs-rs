# build_info

Utilities to obtain information about the build environment.

This crate provides utilities for extracting build environment information and Cargo workspace metadata, primarily intended for use in build scripts.
This information about the build environment would be available as environment variables at compile time, for crates that need to access it.

## Feature Flags

This crate provides both build-time features
(for use in build scripts, i.e., when this crate is used as a build dependency), and runtime features (when this crate is used as a regular dependency).
The build-time features typically have a `-build` suffix, and may also enable additional build-time dependencies.

### Build-time Features

- `cargo-workspace-build`: To extract information about the Cargo workspace.
- `vergen-gix-build`: To generate build environment information using [`vergen-gix`][vergen-gix].

### Runtime Features

- `cargo-workspace`: To access workspace member information at runtime.
- `framework-libs-members-env`: Allows access to this repository's Cargo workspace members.
  This may be useful for applications that wish to enable logs from all crates in this repository, for example.

## Usage and Examples

Refer to the crate documentation in the [`src/lib.rs`][lib-rs] file for examples and usage information.

## License

Licensed under [Apache-2.0][license].

[vergen-gix]: https://crates.io/crates/vergen-gix
[lib-rs]: src/lib.rs
[license]: ../../LICENSE
