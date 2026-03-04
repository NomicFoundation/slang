// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::convert::TryFrom;

use semver::Version;
use thiserror::Error;

/// All supported versions of `Solidity`.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LanguageVersion {
    V0_8_0,
    V0_8_1,
    V0_8_2,
    V0_8_3,
    V0_8_4,
    V0_8_5,
    V0_8_6,
    V0_8_7,
    V0_8_8,
    V0_8_9,
    V0_8_10,
    V0_8_11,
    V0_8_12,
    V0_8_13,
    V0_8_14,
    V0_8_15,
    V0_8_16,
    V0_8_17,
    V0_8_18,
    V0_8_19,
    V0_8_20,
    V0_8_21,
    V0_8_22,
    V0_8_23,
    V0_8_24,
    V0_8_25,
    V0_8_26,
    V0_8_27,
    V0_8_28,
    V0_8_29,
    V0_8_30,
    V0_8_31,
    V0_8_32,
    V0_8_33,
    V0_8_34,
}

#[derive(Debug, Error, PartialEq)]
pub enum FromSemverError {
    #[error("provided version is not supported")]
    UnsupportedVersion,
    #[error("versions with pre-release or build metadata are not supported")]
    UnexpectedMetadata,
}

impl TryFrom<Version> for LanguageVersion {
    type Error = FromSemverError;

    #[allow(clippy::too_many_lines)]
    fn try_from(version: Version) -> Result<Self, Self::Error> {
        let Version {
            major,
            minor,
            patch,
            pre,
            build,
        } = &version;

        if !pre.is_empty() || !build.is_empty() {
            return Err(FromSemverError::UnexpectedMetadata);
        }

        Ok(match (major, minor, patch) {
            (0, 8, 0) => LanguageVersion::V0_8_0,
            (0, 8, 1) => LanguageVersion::V0_8_1,
            (0, 8, 2) => LanguageVersion::V0_8_2,
            (0, 8, 3) => LanguageVersion::V0_8_3,
            (0, 8, 4) => LanguageVersion::V0_8_4,
            (0, 8, 5) => LanguageVersion::V0_8_5,
            (0, 8, 6) => LanguageVersion::V0_8_6,
            (0, 8, 7) => LanguageVersion::V0_8_7,
            (0, 8, 8) => LanguageVersion::V0_8_8,
            (0, 8, 9) => LanguageVersion::V0_8_9,
            (0, 8, 10) => LanguageVersion::V0_8_10,
            (0, 8, 11) => LanguageVersion::V0_8_11,
            (0, 8, 12) => LanguageVersion::V0_8_12,
            (0, 8, 13) => LanguageVersion::V0_8_13,
            (0, 8, 14) => LanguageVersion::V0_8_14,
            (0, 8, 15) => LanguageVersion::V0_8_15,
            (0, 8, 16) => LanguageVersion::V0_8_16,
            (0, 8, 17) => LanguageVersion::V0_8_17,
            (0, 8, 18) => LanguageVersion::V0_8_18,
            (0, 8, 19) => LanguageVersion::V0_8_19,
            (0, 8, 20) => LanguageVersion::V0_8_20,
            (0, 8, 21) => LanguageVersion::V0_8_21,
            (0, 8, 22) => LanguageVersion::V0_8_22,
            (0, 8, 23) => LanguageVersion::V0_8_23,
            (0, 8, 24) => LanguageVersion::V0_8_24,
            (0, 8, 25) => LanguageVersion::V0_8_25,
            (0, 8, 26) => LanguageVersion::V0_8_26,
            (0, 8, 27) => LanguageVersion::V0_8_27,
            (0, 8, 28) => LanguageVersion::V0_8_28,
            (0, 8, 29) => LanguageVersion::V0_8_29,
            (0, 8, 30) => LanguageVersion::V0_8_30,
            (0, 8, 31) => LanguageVersion::V0_8_31,
            (0, 8, 32) => LanguageVersion::V0_8_32,
            (0, 8, 33) => LanguageVersion::V0_8_33,
            (0, 8, 34) => LanguageVersion::V0_8_34,
            _ => return Err(FromSemverError::UnsupportedVersion),
        })
    }
}
