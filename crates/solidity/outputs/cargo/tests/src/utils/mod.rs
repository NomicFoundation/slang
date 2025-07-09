use std::collections::HashSet;

use slang_solidity::utils::LanguageFacts;

#[test]
fn list_all_versions() {
    assert_eq!(LanguageFacts::ALL_VERSIONS.len(), 85);
    assert!(LanguageFacts::ALL_VERSIONS.is_sorted());
    assert_eq!(
        LanguageFacts::ALL_VERSIONS.iter().min(),
        Some(&LanguageFacts::EARLIEST_VERSION)
    );
    assert_eq!(
        LanguageFacts::ALL_VERSIONS.iter().max(),
        Some(&LanguageFacts::LATEST_VERSION)
    );
    let as_set: HashSet<_> = LanguageFacts::ALL_VERSIONS.iter().collect();
    assert_eq!(
        as_set.len(),
        LanguageFacts::ALL_VERSIONS.len(),
        "There are duplicated versions"
    );
}
