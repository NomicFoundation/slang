// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

/// `LanguageFacts` provides metadata about Slang's `Solidity` language support.
pub struct LanguageFacts;

impl LanguageFacts {
    /// All the versions of `Solidity` supported by Slang.
    pub const ALL_VERSIONS: &'static [Version] = &[
        Version::new(0, 4, 11),
        Version::new(0, 4, 12),
        Version::new(0, 4, 13),
        Version::new(0, 4, 14),
        Version::new(0, 4, 15),
        Version::new(0, 4, 16),
        Version::new(0, 4, 17),
        Version::new(0, 4, 18),
        Version::new(0, 4, 19),
        Version::new(0, 4, 20),
        Version::new(0, 4, 21),
        Version::new(0, 4, 22),
        Version::new(0, 4, 23),
        Version::new(0, 4, 24),
        Version::new(0, 4, 25),
        Version::new(0, 4, 26),
        Version::new(0, 5, 0),
        Version::new(0, 5, 1),
        Version::new(0, 5, 2),
        Version::new(0, 5, 3),
        Version::new(0, 5, 4),
        Version::new(0, 5, 5),
        Version::new(0, 5, 6),
        Version::new(0, 5, 7),
        Version::new(0, 5, 8),
        Version::new(0, 5, 9),
        Version::new(0, 5, 10),
        Version::new(0, 5, 11),
        Version::new(0, 5, 12),
        Version::new(0, 5, 13),
        Version::new(0, 5, 14),
        Version::new(0, 5, 15),
        Version::new(0, 5, 16),
        Version::new(0, 5, 17),
        Version::new(0, 6, 0),
        Version::new(0, 6, 1),
        Version::new(0, 6, 2),
        Version::new(0, 6, 3),
        Version::new(0, 6, 4),
        Version::new(0, 6, 5),
        Version::new(0, 6, 6),
        Version::new(0, 6, 7),
        Version::new(0, 6, 8),
        Version::new(0, 6, 9),
        Version::new(0, 6, 10),
        Version::new(0, 6, 11),
        Version::new(0, 6, 12),
        Version::new(0, 7, 0),
        Version::new(0, 7, 1),
        Version::new(0, 7, 2),
        Version::new(0, 7, 3),
        Version::new(0, 7, 4),
        Version::new(0, 7, 5),
        Version::new(0, 7, 6),
        Version::new(0, 8, 0),
        Version::new(0, 8, 1),
        Version::new(0, 8, 2),
        Version::new(0, 8, 3),
        Version::new(0, 8, 4),
        Version::new(0, 8, 5),
        Version::new(0, 8, 6),
        Version::new(0, 8, 7),
        Version::new(0, 8, 8),
        Version::new(0, 8, 9),
        Version::new(0, 8, 10),
        Version::new(0, 8, 11),
        Version::new(0, 8, 12),
        Version::new(0, 8, 13),
        Version::new(0, 8, 14),
        Version::new(0, 8, 15),
        Version::new(0, 8, 16),
        Version::new(0, 8, 17),
        Version::new(0, 8, 18),
        Version::new(0, 8, 19),
        Version::new(0, 8, 20),
        Version::new(0, 8, 21),
        Version::new(0, 8, 22),
        Version::new(0, 8, 23),
        Version::new(0, 8, 24),
        Version::new(0, 8, 25),
        Version::new(0, 8, 26),
        Version::new(0, 8, 27),
        Version::new(0, 8, 28),
        Version::new(0, 8, 29),
        Version::new(0, 8, 30),
    ];

    /// The earliest version of `Solidity` supported by Slang.
    pub const EARLIEST_VERSION: Version = Version::new(0, 4, 11);

    /// The latest version of `Solidity` supported by Slang.
    pub const LATEST_VERSION: Version = Version::new(0, 8, 30);

    /// Infer the language versions that are compatible with the provided `Solidity`
    /// source code. The returned iterator will produce all compatible versions in order, starting with
    /// the earliest one.
    pub fn infer_language_versions(input: &str) -> impl Iterator<Item = &'static Version> {
        crate::utils::infer_language_versions(input)
    }
}
