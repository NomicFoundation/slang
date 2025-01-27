use semver::Version;
use slang_testlang::utils::LanguageFacts;

#[test]
fn list_all_versions() {
    assert_eq!(
        LanguageFacts::ALL_VERSIONS,
        vec![
            Version::new(1, 0, 0),
            Version::new(1, 0, 1),
            Version::new(1, 1, 0),
            Version::new(1, 1, 1),
        ],
    );
}

#[test]
fn list_earliest_version() {
    assert_eq!(LanguageFacts::EARLIEST_VERSION, Version::new(1, 0, 0));
}

#[test]
fn list_latest_version() {
    assert_eq!(LanguageFacts::LATEST_VERSION, Version::new(1, 1, 1));
}
