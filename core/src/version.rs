/// UTC timestamp of when the crate was built.
pub const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");
/// Output of `git describe` for the build's source revision.
pub const GIT_DESCRIBE: &str = env!("VERGEN_GIT_DESCRIBE");
/// Git commit SHA of the build's source revision.
pub const GIT_SHA: &str = env!("VERGEN_GIT_SHA");
/// Release channel of the `rustc` used to build the crate (e.g. `stable`).
pub const RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
/// Commit date of the `rustc` used to build the crate.
pub const RUSTC_COMMIT_DATE: &str = env!("VERGEN_RUSTC_COMMIT_DATE");
/// Commit hash of the `rustc` used to build the crate.
pub const RUSTC_COMMIT_HASH: &str = env!("VERGEN_RUSTC_COMMIT_HASH");
/// Host target triple of the `rustc` used to build the crate.
pub const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
/// LLVM version backing the `rustc` used to build the crate.
pub const RUSTC_LLVM_VERSION: &str = env!("VERGEN_RUSTC_LLVM_VERSION");
/// Semantic version of the `rustc` used to build the crate.
pub const RUSTC_SEMVER: &str = env!("VERGEN_RUSTC_SEMVER");

/// Returns the crate version combined with the git describe string
/// (e.g. `1.2.3-v1.2.3-4-gabcdef`).
pub fn version() -> String {
    format!("{}-{GIT_DESCRIBE}", env!("CARGO_PKG_VERSION"))
}

/// Returns the crate's semantic version as declared in `Cargo.toml`.
pub fn semver() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
