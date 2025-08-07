// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

/// `LanguageFacts` provides metadata about Slang's `CodegenRuntime` language support.
pub struct LanguageFacts;

impl LanguageFacts {
    /// All the versions of `CodegenRuntime` supported by Slang.
    pub const ALL_VERSIONS: &'static [Version] = &[Version::new(0, 0, 0)];

    /// The earliest version of `CodegenRuntime` supported by Slang.
    pub const EARLIEST_VERSION: Version = Version::new(0, 0, 0);

    /// The latest version of `CodegenRuntime` supported by Slang.
    pub const LATEST_VERSION: Version = Version::new(0, 0, 0);

    /// Infer the language versions that are compatible with the provided `CodegenRuntime`
    /// source code. The returned iterator will produce all compatible versions in order, starting with
    /// the earliest one.
    pub fn infer_language_versions(input: &str) -> impl Iterator<Item = &'static Version> {
        crate::extensions::utils::infer_language_versions(input)
    }
}
