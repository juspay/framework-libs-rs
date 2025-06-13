#![allow(missing_docs)]

#[cfg(all(feature = "cargo_workspace", feature = "framework_libs_members_env"))]
mod cargo_workspace {
    include!("src/cargo_workspace.rs");
}

fn main() {
    #[cfg(all(feature = "cargo_workspace", feature = "framework_libs_members_env"))]
    cargo_workspace::set_cargo_workspace_members_env();
}
