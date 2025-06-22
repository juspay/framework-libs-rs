//! This crate provides a way to obtain information about the build environment.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(test(attr(deny(warnings))))]

#[cfg(feature = "cargo-workspace-build")]
mod cargo_workspace;
#[cfg(feature = "vergen-gix-build")]
mod vergen;

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
/// # #[cfg(all(feature = "cargo-workspace-build", feature = "framework-libs-members-env"))]
/// # {
/// // In your crate:
/// let members = build_info::cargo_workspace_members!();
/// assert!(members.contains(env!("CARGO_PKG_NAME")));
/// # }
/// ```
#[macro_export]
macro_rules! cargo_workspace_members {
    () => {
        std::env!("CARGO_WORKSPACE_MEMBERS")
            .split(',')
            .collect::<std::collections::HashSet<&'static str>>()
    };
}

/// Obtain the crates in the `framework-libs-rs` repository's cargo workspace as a `HashSet`.
///
/// This may be useful for enabling logs from crates in the `framework-libs-rs` repository's
/// workspace, for example.
#[cfg(all(
    feature = "cargo-workspace-build",
    feature = "framework-libs-members-env"
))]
#[inline]
pub fn framework_libs_workspace_members() -> std::collections::HashSet<&'static str> {
    crate::cargo_workspace_members!()
}
