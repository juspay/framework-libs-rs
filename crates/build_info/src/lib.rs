//! This crate provides a way to obtain information about the build environment.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "cargo_workspace")]
mod cargo_workspace;

#[cfg(feature = "cargo_workspace")]
pub use cargo_workspace::set_cargo_workspace_members_env;

/// Obtain the crates in the framework libs repository's workspace as a `HashSet`.
///
/// This may be useful for enabling logs from crates in the framework libs repository's workspace,
/// for example.
#[cfg(all(feature = "cargo_workspace", feature = "framework_libs_members_env"))]
#[inline]
pub fn framework_libs_workspace_members() -> std::collections::HashSet<&'static str> {
    crate::cargo_workspace_members!()
}
