use crate::versions::specifier::LanguageVersionSpecifier;
use crate::versions::version::{LanguageVersion, LanguageVersionConversionError};

#[test]
fn test_correct_version() {
    let version = LanguageVersion::try_from(semver::Version::new(0, 8, 0));
    assert_eq!(version, Ok(LanguageVersion::V0_8_0));
}

#[test]
fn test_older_version() {
    let unsupported_version = semver::Version::new(0, 7, 6);
    let version = LanguageVersion::try_from(unsupported_version.clone());
    assert_eq!(
        version,
        Err(LanguageVersionConversionError::UnsupportedVersion)
    );
}

#[test]
fn test_newer_version() {
    let unsupported_version = semver::Version::new(0, 9, 0);
    let version = LanguageVersion::try_from(unsupported_version.clone());
    assert_eq!(
        version,
        Err(LanguageVersionConversionError::UnsupportedVersion)
    );
}

#[test]
fn test_pre_release_metadata() {
    let version = LanguageVersion::try_from(semver::Version::parse("0.8.0-alpha").unwrap());
    assert_eq!(
        version,
        Err(LanguageVersionConversionError::UnexpectedMetadata)
    );
}

#[test]
fn test_build_metadata() {
    let version = LanguageVersion::try_from(semver::Version::parse("0.8.0+alpha").unwrap());
    assert_eq!(
        version,
        Err(LanguageVersionConversionError::UnexpectedMetadata)
    );
}

#[test]
fn test_intersection_of_intersecting_specifiers() {
    let from = LanguageVersionSpecifier::from(LanguageVersion::V0_8_10);
    let till = LanguageVersionSpecifier::till(LanguageVersion::V0_8_20);

    assert_eq!(
        from.intersect(&till),
        Some(LanguageVersionSpecifier::range(
            LanguageVersion::V0_8_10,
            LanguageVersion::V0_8_20
        ))
    );
    // The intersection doesn't depend on the order of the operands.
    assert_eq!(from.intersect(&till), till.intersect(&from));
}

#[test]
fn test_intersection_of_disjoint_specifiers() {
    let till = LanguageVersionSpecifier::till(LanguageVersion::V0_8_10);
    let from = LanguageVersionSpecifier::from(LanguageVersion::V0_8_10);

    // 'Till' excludes its own bound, so these two share no version.
    assert_eq!(till.intersect(&from), None);
    assert_eq!(
        LanguageVersionSpecifier::range(LanguageVersion::V0_8_1, LanguageVersion::V0_8_4)
            .intersect(&LanguageVersionSpecifier::range(
                LanguageVersion::V0_8_4,
                LanguageVersion::V0_8_8
            )),
        None
    );
}

#[test]
fn test_intersection_is_canonical() {
    // Two shapes of the same set intersection into the same canonical specifier, so
    // comparing intersections is a subset test.
    let till = LanguageVersionSpecifier::till(LanguageVersion::V0_8_20);
    let range =
        LanguageVersionSpecifier::range(LanguageVersion::EARLIEST, LanguageVersion::V0_8_20);

    assert_eq!(till.intersect(&till), range.intersect(&range));
    assert_eq!(till.intersect(&till), Some(range));

    let unbounded = LanguageVersionSpecifier::from(LanguageVersion::V0_8_5);
    assert_eq!(unbounded.intersect(&unbounded), Some(unbounded.clone()));

    // A subset intersects with itself; a wider specifier doesn't.
    let inner = LanguageVersionSpecifier::range(LanguageVersion::V0_8_6, LanguageVersion::V0_8_9);
    assert_eq!(inner.intersect(&unbounded), inner.intersect(&inner));
    assert_ne!(unbounded.intersect(&inner), unbounded.intersect(&unbounded));
}
