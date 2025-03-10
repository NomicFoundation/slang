// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

pub struct LanguageFacts;

impl LanguageFacts {
    pub const ALL_VERSIONS: &'static [Version] = &[
        Version::new(1, 0, 0),
        Version::new(1, 0, 1),
        Version::new(1, 1, 0),
        Version::new(1, 1, 1),
    ];

    pub const EARLIEST_VERSION: Version = Version::new(1, 0, 0);

    pub const LATEST_VERSION: Version = Version::new(1, 1, 1);

    pub fn infer_language_versions(input: &str) -> Vec<Version> {
        crate::extensions::semver::infer_language_versions(input)
    }
}
