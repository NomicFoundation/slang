use semver::Version;

/// Returns true if a given `solc` version is known to segfault on a multitude
/// of parse errors and should be skipped from automated testing.
pub fn is_solc_segfault(version: &Version) -> bool {
    matches!(
        version,
        Version {
            major: 0,
            minor: 4,
            patch: 11,
            pre: _,
            build: _,
        }
    )
}
