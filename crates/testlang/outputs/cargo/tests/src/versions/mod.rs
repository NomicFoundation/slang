use semver::Version;
use slang_testlang::utils::LanguageFacts;

#[test]
fn list_supported_versions() {
    let versions = LanguageFacts::SUPPORTED_VERSIONS;

    assert!(!versions.is_empty());
    assert!(!versions.contains(&Version::new(0, 0, 0)));
    assert!(versions.contains(&Version::new(1, 0, 0)));
}
