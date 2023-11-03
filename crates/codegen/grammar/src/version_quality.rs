use semver::Version;

#[derive(Clone, Debug, Copy, PartialEq, Eq, strum_macros::Display)]
pub enum VersionQuality {
    Introduced,
    Removed,
}

#[derive(Clone, Debug)]
pub struct VersionQualityRange {
    pub from: Version,
    pub quality: VersionQuality,
}
