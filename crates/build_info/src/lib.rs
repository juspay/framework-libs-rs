//! Utilities to obtain information about the build environment.
//!
//! This crate provides utilities for extracting build environment information and cargo workspace
//! metadata, primarily intended for use in build scripts.
//! This information about the build environment would be available as environment variables at
//! compile time, for crates that need to access it.
//!
//! # Features
//!
//! ## Build-time Features
//!
//! These feature are intended for use in build scripts and may pull in additional dependencies.
//!
//! ### `cargo-workspace-build`
//!
//! Enables the [`cargo_metadata`] dependency for build scripts that need to extract workspace
//! information.
//! Enabling this feature provides the [`set_cargo_workspace_members_env()`] function.
//!
//! #### Usage in Build Scripts
//!
//! ```toml
//! [build-dependencies]
//! build_info = { version = "0.1.0", features = ["cargo-workspace-build"] }
//! ```
//!
//! ```
//! // In your crate's build script (build.rs):
//! # #[cfg(feature = "cargo-workspace-build")]
//! # {
//! build_info::set_cargo_workspace_members_env();
//! # }
//! ```
//!
//! ### `vergen-gix-build`
//!
//! Enables the [`vergen_gix`] dependency for build scripts that need to generate build environment
//! information.
//! Enabling this feature provides the [`generate_vergen_cargo_instructions()`] function.
//! The available build-time information includes:
//!
//! - Build date and timestamp
//! - Cargo's target triple
//! - Rust compiler version, commit date and commit hash
//! - Git commit timestamp, tag (output of `git describe` command) and short commit hash
//!
//! #### Usage in Build Scripts
//!
//! ```toml
//! [build-dependencies]
//! build_info = { version = "0.1.0", features = ["vergen-gix-build"] }
//! ```
//!
//! ```
//! // In your crate's build script (build.rs):
//! # #[cfg(feature = "vergen-gix-build")]
//! # {
//! build_info::generate_vergen_cargo_instructions();
//! # }
//! ```
//!
//! This will set various environment variables that can be accessed at compile time using the
//! `env!()` macro.
//! Refer to the documentation of the [`vergen_gix`] crate for more information on the
//! environment variables that would be set.
//!
//! ## Runtime Features
//!
//! These features provide functionality that can be used at runtime (when this crate is used as a
//! regular dependency), requiring minimal or no additional dependencies.
//!
//! ### `cargo-workspace`
//!
//! Enables the [`cargo_workspace_members!()`][cargo_workspace_members] macro for accessing
//! workspace member information at runtime.
//!
//! #### Example
//!
//! ```toml
//! [dependencies]
//! build_info = { version = "0.1.0", features = ["cargo-workspace"] }
//! ```
//!
//! ```
//! # #[cfg(all(
//! #     feature = "cargo-workspace",
//! #     feature = "framework-libs-members-env"
//! # ))]
//! # {
//! // Assuming that the `set_cargo_workspace_members_env()` function was called in build script
//! let members = build_info::cargo_workspace_members!();
//! assert!(members.contains(env!("CARGO_PKG_NAME")));
//! # }
//! ```
//!
//! ### `vergen-gix`
//!
//! Provides macros for accessing vergen-generated environment variables at runtime.
//!
//! #### Example
//!
//! Refer to the [`vergen_integration` example][vergen-integration-example] for a complete example
//! of using the `vergen-gix` feature.
//!
//! [vergen-integration-example]: https://github.com/juspay/framework-libs-rs/tree/main/examples/vergen_integration
//!
//! ### `framework-libs-members-env`
//!
//! Allows access to the [`framework-libs-rs`][framework-libs-rs-github] repository's cargo
//! workspace members at runtime.
//! This can be useful for enabling logs from all crates in the
//! [`framework-libs-rs`][framework-libs-rs-github] repository when an application depends on
//! multiple crates from the repository, for example.
//! Enabling this feature provides the [`framework_libs_workspace_members()`] function.
//!
//! [framework-libs-rs-github]: https://github.com/juspay/framework-libs-rs
//!
//! #### Example
//!
//! ```toml
//! [dependencies]
//! build_info = { version = "0.1.0", features = ["framework-libs-members-env"] }
//! ```
//!
//! ```
//! # #[cfg(feature = "framework-libs-members-env")]
//! # {
//! let members = build_info::framework_libs_workspace_members();
//! assert!(members.contains("build_info"));
//! # }
//! ```

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "cargo-workspace-build")]
mod cargo_workspace;
#[cfg(feature = "vergen-gix-build")]
mod vergen;
#[cfg(feature = "vergen-gix")]
mod vergen_macros;

#[cfg(feature = "cargo-workspace-build")]
pub use cargo_workspace::set_cargo_workspace_members_env;
#[cfg(feature = "vergen-gix-build")]
pub use vergen::generate_vergen_cargo_instructions;

/// Obtain the crates in the current cargo workspace as a `HashSet`.
///
/// This macro requires that [`set_cargo_workspace_members_env()`] function be called in the
/// build script of the crate where this macro is being called.
///
/// # Errors
///
/// Causes a compilation error if the `CARGO_WORKSPACE_MEMBERS` environment variable is unset.
///
/// # Example
///
/// ```
/// # #[cfg(feature = "cargo-workspace-build")]
/// # {
/// // In your crate's build script (build.rs):
/// build_info::set_cargo_workspace_members_env();
/// # }
///
/// # #[cfg(all(
/// #     feature = "cargo-workspace",
/// #     feature = "framework-libs-members-env"
/// # ))]
/// # {
/// // In your crate:
/// let members = build_info::cargo_workspace_members!();
/// assert!(members.contains(env!("CARGO_PKG_NAME")));
/// # }
/// ```
#[cfg(feature = "cargo-workspace")]
#[macro_export]
macro_rules! cargo_workspace_members {
    () => {
        std::env!("CARGO_WORKSPACE_MEMBERS")
            .split(',')
            .collect::<std::collections::HashSet<&'static str>>()
    };
}

/// Obtain the crates in the [`framework-libs-rs`][framework-libs-rs-github] repository's
/// cargo workspace as a `HashSet`.
///
/// This may be useful for enabling logs from crates in the `framework-libs-rs` repository's
/// workspace, for example.
///
/// [framework-libs-rs-github]: https://github.com/juspay/framework-libs-rs
#[cfg(feature = "framework-libs-members-env")]
#[inline]
pub fn framework_libs_workspace_members() -> std::collections::HashSet<&'static str> {
    crate::cargo_workspace_members!()
}
