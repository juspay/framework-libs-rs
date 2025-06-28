use build_info::{
    build_date, build_timestamp, cargo_target_triple, git_commit_timestamp, git_describe, git_sha,
    rustc_commit_date, rustc_commit_hash, rustc_semver,
};

fn main() {
    let build_date = build_date!();
    let build_timestamp = build_timestamp!();
    println!("Build date: {build_date}");
    println!("Build timestamp: {build_timestamp}");

    let target_triple = cargo_target_triple!();
    println!("Target triple: {target_triple}");

    let rustc_version = rustc_semver!();
    let rustc_commit_hash = rustc_commit_hash!();
    let rustc_commit_date = rustc_commit_date!();
    println!("Rustc version: {rustc_version}");
    println!("Rustc commit hash: {rustc_commit_hash}");
    println!("Rustc commit date: {rustc_commit_date}");

    let git_timestamp = git_commit_timestamp!();
    let git_describe = git_describe!();
    let git_sha = git_sha!();
    println!("Git commit timestamp: {git_timestamp}");
    println!("Git describe: {git_describe}");
    println!("Git SHA: {git_sha}");
}
