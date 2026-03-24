#![expect(missing_docs)]

#[cfg(feature = "framework-libs-members-env")]
mod cargo_workspace {
    include!("src/cargo_workspace.rs");
}

#[cfg(feature = "framework-libs-members-env")]
#[expect(clippy::expect_used)]
fn set_framework_libs_workspace_members_env() {
    use std::io::Write;

    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("Failed to obtain cargo metadata");

    let workspace_members = metadata
        .workspace_packages()
        .iter()
        .filter(|package| {
            package
                .publish
                .as_ref()
                .map(|v| !v.is_empty())
                .unwrap_or(true)
        })
        .map(|package| package.name.as_str())
        .collect::<Vec<_>>()
        .join(",");

    writeln!(
        &mut std::io::stdout(),
        "cargo:rustc-env=FRAMEWORK_LIBS_WORKSPACE_MEMBERS={workspace_members}"
    )
    .expect("Failed to set FRAMEWORK_LIBS_WORKSPACE_MEMBERS env var");
}

fn main() {
    #[cfg(feature = "framework-libs-members-env")]
    {
        cargo_workspace::set_cargo_workspace_members_env();
        set_framework_libs_workspace_members_env();
    }
}
