use crate::language_versions::versions::LanguageVersion;

#[test]
fn test_correct_version() {
    let version = LanguageVersion::try_from(semver::Version::new(0, 4, 11));
    assert_eq!(version, Ok(LanguageVersion::Version0_4_11));
}

#[test]
fn test_older_version() {
    let version = LanguageVersion::try_from(semver::Version::parse("1.0.0-alpha").unwrap());
    assert_eq!(
        version,
        Err("Pre-release and build metadata are not supported".to_string())
    );
}

#[test]
fn test_newer_version() {
    let version = LanguageVersion::try_from(semver::Version::new(0, 9, 0));
    assert_eq!(version, Err("Unsupported version".to_string()));
}
