// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

pub struct LanguageFacts;

impl LanguageFacts {
    pub const ALL_VERSIONS: &'static [Version] = &[Version::new(0, 0, 0)];

    pub const EARLIEST_VERSION: Version = Version::new(0, 0, 0);

    pub const LATEST_VERSION: Version = Version::new(0, 0, 0);

    pub fn infer_language_versions(input: &str) -> Vec<Version> {
        crate::extensions::utils::infer_language_versions(input)
    }
}
