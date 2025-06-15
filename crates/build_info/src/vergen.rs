/// Generate `cargo` build instructions with information about the build environment using the
/// `vergen` family of crates.
///
/// This function should be typically called within build scripts, so that the environment
/// variables are available to the corresponding crate at compile time.
///
/// The generated instructions would provide the following information:
/// - Build date and timestamp
/// - Cargo's target triple
/// - Rust compiler version, commit date and commit hash
/// - Git commit timestamp, tag (output of `git describe` command) and short commit hash
///
/// Refer to the documentation of the [`vergen_gix`] crate for more information on the
/// environment variables that would be set.
///
/// # Panics
///
/// Panics if any of the `vergen` emitters fail to generate the instructions.
#[allow(clippy::expect_used, reason = "panics are acceptable in build scripts")]
pub fn generate_vergen_cargo_instructions() {
    use vergen_gix::{BuildBuilder, CargoBuilder, Emitter, GixBuilder, RustcBuilder};

    Emitter::default()
        .add_instructions(
            &BuildBuilder::default()
                .build_date(true)
                .build_timestamp(true)
                .build()
                .expect("Failed to generate build related build instructions"),
        )
        .expect("Failed to generate `cargo` related build instructions")
        .add_instructions(
            &CargoBuilder::default()
                .target_triple(true)
                .build()
                .expect("Failed to generate `cargo` related build instructions"),
        )
        .expect("Failed to generate `cargo` related build instructions")
        .add_instructions(
            &RustcBuilder::default()
                .semver(true)
                .commit_hash(true)
                .commit_date(true)
                .build()
                .expect("Failed to generate `rustc` related build instructions"),
        )
        .expect("Failed to generate `rustc` related build instructions")
        .add_instructions(
            &GixBuilder::default()
                .commit_timestamp(true)
                .describe(true, true, None)
                .sha(true)
                .build()
                .expect("Failed to generate `git` related build instructions"),
        )
        .expect("Failed to generate `git` related build instructions")
        .emit()
        .expect("Failed to generate `vergen`-based `cargo` build instructions");
}
