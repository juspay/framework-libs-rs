//! This crate provides a way to obtain information about the build environment.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "cargo-workspace")]
mod cargo_workspace;
#[cfg(feature = "vergen-gix")]
mod vergen;

#[cfg(feature = "cargo-workspace")]
pub use cargo_workspace::set_cargo_workspace_members_env;
#[cfg(feature = "vergen-gix")]
pub use vergen::generate_vergen_cargo_instructions;

/// Obtain the crates in the framework libs repository's workspace as a `HashSet`.
///
/// This may be useful for enabling logs from crates in the framework libs repository's workspace,
/// for example.
#[cfg(all(feature = "cargo-workspace", feature = "framework-libs-members-env"))]
#[inline]
pub fn framework_libs_workspace_members() -> std::collections::HashSet<&'static str> {
    crate::cargo_workspace_members!()
}
