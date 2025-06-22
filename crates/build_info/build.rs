#![allow(missing_docs)]

#[cfg(feature = "framework-libs-members-env")]
mod cargo_workspace {
    include!("src/cargo_workspace.rs");
}

fn main() {
    #[cfg(feature = "framework-libs-members-env")]
    cargo_workspace::set_cargo_workspace_members_env();
}
