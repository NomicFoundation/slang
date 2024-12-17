// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

pub struct LanguageFacts;

impl LanguageFacts {
    pub const NAME: &'static str = "Testlang";

    pub const SUPPORTED_VERSIONS: &'static [Version] = &[
        Version::new(1, 0, 0),
        Version::new(1, 0, 1),
        Version::new(1, 1, 0),
        Version::new(1, 1, 1),
    ];
}
