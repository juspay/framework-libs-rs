/// Sets the `CARGO_WORKSPACE_MEMBERS` environment variable to include a comma-separated list of
/// names of all crates in the current cargo workspace.
///
/// This function should be typically called within build scripts, so that the environment variable
/// is available to the corresponding crate at compile time.
///
/// # Panics
///
/// Panics if running the `cargo metadata` command fails.
///
/// # Example
///
/// ```
/// // In your crate's build script (build.rs):
/// build_info::set_cargo_workspace_members_env();
/// ```
#[allow(clippy::expect_used)]
pub fn set_cargo_workspace_members_env() {
    use std::io::Write;

    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("Failed to obtain cargo metadata");

    let workspace_members = metadata
        .workspace_packages()
        .iter()
        .map(|package| package.name.as_str())
        .collect::<Vec<_>>()
        .join(",");

    writeln!(
        &mut std::io::stdout(),
        "cargo:rustc-env=CARGO_WORKSPACE_MEMBERS={workspace_members}"
    )
    .expect("Failed to set `CARGO_WORKSPACE_MEMBERS` environment variable");
}

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
/// // In your crate's build script (build.rs):
/// build_info::set_cargo_workspace_members_env();
///
/// # #[cfg(feature = "framework-libs-members-env")]
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

#[cfg(test)]
mod tests {
    #[test]
    fn verify_cargo_metadata_output_contains_current_crate() {
        let metadata = cargo_metadata::MetadataCommand::new()
            .exec()
            .expect("Failed to obtain cargo metadata");

        assert!(
            metadata
                .workspace_packages()
                .iter()
                .any(|package| package.name == env!("CARGO_PKG_NAME")),
            "Current crate is not present in `cargo metadata` output"
        );
    }

    #[cfg(feature = "framework-libs-members-env")]
    #[test]
    fn test_cargo_workspace_members_contains_current_crate() {
        let env_value = env!("CARGO_WORKSPACE_MEMBERS");
        assert!(
            env_value
                .split(',')
                .any(|name| name == env!("CARGO_PKG_NAME")),
            "Current crate is not present in the `CARGO_WORKSPACE_MEMBERS` environment variable"
        );

        let members = crate::cargo_workspace_members!();
        assert!(
            members.contains(env!("CARGO_PKG_NAME")),
            "Current crate is not present in the output of `cargo_workspace_members!()` macro"
        );
    }
}
