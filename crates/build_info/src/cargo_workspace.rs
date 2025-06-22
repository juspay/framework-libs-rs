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
        let current_crate = env!("CARGO_PKG_NAME");

        assert!(
            env_value.split(',').any(|name| name == current_crate),
            "Current crate is not present in the `CARGO_WORKSPACE_MEMBERS` environment variable"
        );

        let members = crate::cargo_workspace_members!();
        assert!(
            members.contains(current_crate),
            "Current crate is not present in the output of `cargo_workspace_members!()` macro"
        );
    }
}
