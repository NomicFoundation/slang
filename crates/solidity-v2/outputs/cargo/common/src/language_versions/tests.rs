use crate::language_versions::versions::{FromVersionErrors, LanguageVersion};

#[test]
fn test_correct_version() {
    let version = LanguageVersion::try_from(semver::Version::new(0, 4, 11));
    assert_eq!(version, Ok(LanguageVersion::V0_4_11));
}

#[test]
fn test_older_version() {
    let version = LanguageVersion::try_from(semver::Version::parse("1.0.0-alpha").unwrap());
    assert_eq!(version, Err(FromVersionErrors::PreReleaseOrBuildMetadata));
}

#[test]
fn test_newer_version() {
    let unsupported_version = semver::Version::new(0, 9, 0);
    let version = LanguageVersion::try_from(unsupported_version.clone());
    assert_eq!(
        version,
        Err(FromVersionErrors::UnsupportedVersion(unsupported_version))
    );
}
