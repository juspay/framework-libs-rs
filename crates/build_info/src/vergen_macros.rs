//! Macros for accessing vergen-generated environment variables.

/// Returns the build date.
///
/// Reads the `VERGEN_BUILD_DATE` environment variable.
#[macro_export]
macro_rules! build_date {
    () => {
        env!("VERGEN_BUILD_DATE")
    };
}

/// Returns the build timestamp.
///
/// Reads the `VERGEN_BUILD_TIMESTAMP` environment variable.
#[macro_export]
macro_rules! build_timestamp {
    () => {
        env!("VERGEN_BUILD_TIMESTAMP")
    };
}

/// Returns the cargo target triple.
///
/// Reads the `VERGEN_CARGO_TARGET_TRIPLE` environment variable.
#[macro_export]
macro_rules! cargo_target_triple {
    () => {
        env!("VERGEN_CARGO_TARGET_TRIPLE")
    };
}

/// Returns the rustc semantic version.
///
/// Reads the `VERGEN_RUSTC_SEMVER` environment variable.
#[macro_export]
macro_rules! rustc_semver {
    () => {
        env!("VERGEN_RUSTC_SEMVER")
    };
}

/// Returns the rustc commit hash.
///
/// Reads the `VERGEN_RUSTC_COMMIT_HASH` environment variable.
#[macro_export]
macro_rules! rustc_commit_hash {
    () => {
        env!("VERGEN_RUSTC_COMMIT_HASH")
    };
}

/// Returns the rustc commit date.
///
/// Reads the `VERGEN_RUSTC_COMMIT_DATE` environment variable.
#[macro_export]
macro_rules! rustc_commit_date {
    () => {
        env!("VERGEN_RUSTC_COMMIT_DATE")
    };
}

/// Returns the git commit timestamp.
///
/// Reads the `VERGEN_GIT_COMMIT_TIMESTAMP` environment variable.
#[macro_export]
macro_rules! git_commit_timestamp {
    () => {
        env!("VERGEN_GIT_COMMIT_TIMESTAMP")
    };
}

/// Returns the git describe output.
///
/// Reads the `VERGEN_GIT_DESCRIBE` environment variable.
#[macro_export]
macro_rules! git_describe {
    () => {
        env!("VERGEN_GIT_DESCRIBE")
    };
}

/// Returns the git commit SHA.
///
/// Reads the `VERGEN_GIT_SHA` environment variable.
#[macro_export]
macro_rules! git_sha {
    () => {
        env!("VERGEN_GIT_SHA")
    };
}
