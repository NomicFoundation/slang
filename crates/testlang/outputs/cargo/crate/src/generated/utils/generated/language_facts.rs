// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

/// `LanguageFacts` provides metadata about Slang's `Testlang` language support.
pub struct LanguageFacts;

impl LanguageFacts {
    /// All the versions of `Testlang` supported by Slang.
    pub const ALL_VERSIONS: &'static [Version] = &[
        Version::new(1, 0, 0),
        Version::new(1, 0, 1),
        Version::new(1, 1, 0),
        Version::new(1, 1, 1),
    ];

    /// The earliest version of `Testlang` supported by Slang.
    pub const EARLIEST_VERSION: Version = Version::new(1, 0, 0);

    /// The latest version of `Testlang` supported by Slang.
    pub const LATEST_VERSION: Version = Version::new(1, 1, 1);

    /// Infer the language versions that are compatible with the provided `Testlang`
    /// source code. The returned iterator will produce all compatible versions in order, starting with
    /// the earliest one.
    pub fn infer_language_versions(input: &str) -> impl Iterator<Item = &'static Version> {
        crate::extensions::utils::infer_language_versions(input)
    }
}
