pub mod parser;

#[cfg(test)]
pub mod tests;

use std::fmt::Display;

use semver::Version;

use crate::cst::NonterminalKind;
use crate::utils::LanguageFacts;

/// Parse the version pragmas in the given Solidity source code and return a list of language
/// versions that can fulfill those requirements.
pub fn infer_language_versions(src: &str) -> Vec<Version> {
    let parser = crate::parser::Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let output = parser.parse_file_contents(src);

    let mut cursor = output.create_tree_cursor();

    let mut found_ranges = Vec::<Range>::new();
    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionPragma) {
        let pragma_text = cursor.node().unparse();
        let pragma_text = pragma_text.trim();
        let pragma_text = pragma_text
            .strip_prefix("solidity")
            .unwrap_or(pragma_text)
            .trim();

        found_ranges.push(parser::parse(pragma_text));
    }

    let mut matching_versions = vec![];
    for lang_version in LanguageFacts::ALL_VERSIONS {
        if found_ranges.iter().all(|r| r.matches(lang_version)) {
            matching_versions.push(lang_version.clone());
        }
    }

    matching_versions.sort();
    matching_versions
}

/// A version that may not have specified all values. In general unspecified values will
/// convert to `0` when resolved, however it is often important to know which values were _unspecified_
/// (as opposed to being _specified as `0`_) when resolving ranges.
#[derive(Copy, Clone)]
struct PartialVersion {
    major: VersionPart,
    minor: VersionPart,
    patch: VersionPart,
}

impl PartialVersion {
    fn new() -> PartialVersion {
        PartialVersion {
            major: VersionPart::None,
            minor: VersionPart::None,
            patch: VersionPart::None,
        }
    }

    fn wild() -> PartialVersion {
        PartialVersion {
            major: VersionPart::Wildcard,
            minor: VersionPart::Wildcard,
            patch: VersionPart::Wildcard,
        }
    }

    /// Check if `PartialVersion` has no specified values.
    fn is_wild(&self) -> bool {
        self.major.is_wild() && self.minor.is_wild() && self.patch.is_wild()
    }

    fn is_complete(&self) -> bool {
        self.major.is_specified() && self.minor.is_specified() && self.patch.is_specified()
    }
}

impl From<PartialVersion> for Version {
    fn from(partial: PartialVersion) -> Version {
        Version::new(
            partial.major.into(),
            partial.minor.into(),
            partial.patch.into(),
        )
    }
}

impl From<&PartialVersion> for Version {
    fn from(partial: &PartialVersion) -> Version {
        Version::new(
            partial.major.into(),
            partial.minor.into(),
            partial.patch.into(),
        )
    }
}

/// A single part (major, minor, or patch) of a `PartialVersion`.
/// In a typical semver implementation, there's no difference between a wildcard part (using `*` or `x`) and
/// a completely unspecified part. solc, however, distinguishes these two in certain cases, so we need to
/// keep track of which was used.
#[derive(Copy, Clone)]
enum VersionPart {
    /// This part was not specified at all, i.e. the patch part in `1.2`.
    None,
    /// This part was explicitly given a wildcard, i.e. the minor part in `1.x`.
    Wildcard,
    /// This part was given a concrete value.
    Specified(u64),
}

impl VersionPart {
    fn is_wild(self) -> bool {
        matches!(self, Self::Wildcard)
    }

    fn is_none(self) -> bool {
        matches!(self, VersionPart::None)
    }

    fn is_specified(self) -> bool {
        matches!(self, VersionPart::Specified(_))
    }

    fn unwrap(self) -> u64 {
        match self {
            VersionPart::Specified(v) => v,
            VersionPart::Wildcard => panic!("Tried to unwrap a wildcard segment"),
            VersionPart::None => panic!("Tried to unwrap an unspecified segment"),
        }
    }
}

impl From<VersionPart> for u64 {
    fn from(segment: VersionPart) -> u64 {
        match segment {
            VersionPart::Specified(v) => v,
            _ => 0,
        }
    }
}

impl Display for VersionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {}
            Self::Wildcard => write!(f, "*")?,
            Self::Specified(v) => write!(f, "{v}")?,
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Operator::Lt => "<",
            Operator::LtEq => "<=",
            Operator::Gt => ">",
            Operator::GtEq => ">=",
            Operator::Eq => "==",
        };

        write!(f, "{repr}")
    }
}

/// A comparator is the basic unit of comparison in a version range. More complex
/// ranges always boil down to these.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Comparator {
    version: Version,
    op: Operator,
}

impl Comparator {
    fn matches(&self, test_version: &Version) -> bool {
        match self.op {
            Operator::Lt => *test_version < self.version,
            Operator::LtEq => *test_version <= self.version,
            Operator::Gt => *test_version > self.version,
            Operator::GtEq => *test_version >= self.version,
            Operator::Eq => *test_version == self.version,
        }
    }
}

impl Default for Comparator {
    fn default() -> Comparator {
        Comparator {
            version: Version::new(0, 0, 0),
            op: Operator::GtEq,
        }
    }
}

impl Display for Comparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.version)
    }
}

/// A comparator set contains a list of comparators. For a version to match
/// the comparator set, it must match _all_ of the comparators within.
///
/// A basic comparator set is written with one or more comparators separated by whitespace.
/// For example "`>=0.5.0 <1.5.0`" is a comparator set with 2 comparators.
#[derive(Debug, Clone)]
struct ComparatorSet {
    comparators: Vec<Comparator>,
}

impl ComparatorSet {
    fn new() -> ComparatorSet {
        ComparatorSet {
            comparators: vec![],
        }
    }

    fn single(comp: Comparator) -> ComparatorSet {
        ComparatorSet {
            comparators: vec![comp],
        }
    }

    fn wild() -> ComparatorSet {
        ComparatorSet::single(Comparator::default())
    }

    fn bounds(lower: Comparator, upper: Comparator) -> ComparatorSet {
        ComparatorSet {
            comparators: vec![lower, upper],
        }
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `^[partial]`.
    fn caret(partial: &PartialVersion) -> ComparatorSet {
        let lower = Comparator {
            version: partial.into(),
            op: Operator::GtEq,
        };

        let upper_version = match partial.major {
            VersionPart::Specified(0) => match partial.minor {
                VersionPart::Specified(0) => match partial.patch {
                    VersionPart::Specified(patch) => Version::new(0, 0, patch + 1),
                    _ => Version::new(0, 1, 0),
                },
                VersionPart::Specified(minor) => Version::new(0, minor + 1, 0),
                _ => Version::new(1, 0, 0),
            },
            VersionPart::Specified(major) => Version::new(major + 1, 0, 0),
            _ => panic!("Major version must be specified when resolving a caret upper bound"),
        };

        let upper = Comparator {
            version: upper_version,
            op: Operator::Lt,
        };

        ComparatorSet::bounds(lower, upper)
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `~[partial]`.
    fn tilde(partial: &PartialVersion) -> ComparatorSet {
        let major = partial.major.unwrap();

        let lower = Comparator {
            version: partial.into(),
            op: Operator::GtEq,
        };

        let upper_version = match partial.minor {
            VersionPart::Specified(minor) => Version::new(major, minor + 1, 0),
            _ => Version::new(major + 1, 0, 0),
        };

        let upper = Comparator {
            version: upper_version,
            op: Operator::Lt,
        };

        ComparatorSet::bounds(lower, upper)
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `[lower_version] - [upper_version]`.
    fn hyphen_range(
        lower_version: &PartialVersion,
        upper_version: &PartialVersion,
    ) -> ComparatorSet {
        let lower = Comparator {
            version: lower_version.into(),
            op: Operator::GtEq,
        };

        let upper = if upper_version.is_complete() {
            Comparator {
                version: upper_version.into(),
                op: Operator::LtEq,
            }
        } else {
            let mut upper_resolved: Version = upper_version.into();
            if upper_version.minor.is_none() {
                upper_resolved.major += 1;
            } else if upper_version.patch.is_none() {
                upper_resolved.minor += 1;
            }

            Comparator {
                version: upper_resolved,
                op: Operator::Lt,
            }
        };

        ComparatorSet::bounds(lower, upper)
    }

    /// Create a `ComparatorSet` that represents the range described by `[partial]`.
    /// A partially specified version represents a version range, sometimes called an x-range.
    /// `1.x == >=1.0.0 <2.0.0`
    /// `0.4.x == >=0.4.0 <0.5.0`
    /// `2.1 == >=2.1.0 < 2.2.0`
    fn partial_range(partial: &PartialVersion) -> ComparatorSet {
        if partial.is_wild() {
            return ComparatorSet::single(Comparator::default());
        }

        if partial.is_complete() {
            return ComparatorSet::single(Comparator {
                version: partial.into(),
                op: Operator::Eq,
            });
        }

        let lower = Comparator {
            version: partial.into(),
            op: Operator::GtEq,
        };

        let upper = match partial.major {
            VersionPart::Specified(major) => match partial.minor {
                VersionPart::Specified(minor) => match partial.patch {
                    // Handled above by checking !partial.is_incomplete()
                    VersionPart::Specified(_) => unreachable!(),
                    _ => Comparator {
                        version: Version::new(major, minor + 1, 0),
                        op: Operator::Lt,
                    },
                },
                _ => Comparator {
                    version: Version::new(major + 1, 0, 0),
                    op: Operator::Lt,
                },
            },
            // Handled above by checking partial.is_wild()
            _ => unreachable!(),
        };

        ComparatorSet::bounds(lower, upper)
    }

    fn merge(&mut self, other: &ComparatorSet) {
        for comp in &other.comparators {
            self.comparators.push(comp.clone());
        }
    }

    fn matches(&self, test_version: &Version) -> bool {
        if self.comparators.is_empty() {
            false
        } else {
            self.comparators.iter().all(|cmp| cmp.matches(test_version))
        }
    }
}

impl Display for ComparatorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, c) in self.comparators.iter().enumerate() {
            write!(f, "{c}")?;
            if i < self.comparators.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl PartialEq for ComparatorSet {
    fn eq(&self, other: &Self) -> bool {
        for comp in &self.comparators {
            if !other.comparators.contains(comp) {
                return false;
            }
        }
        true
    }
}

/// A `Range` represents a complete version matcher. It is composed of multiple `ComparatorSet`s, separated by '||'.
/// In order for a version to match a `Range`, it only has to match at least one of the comparator sets.
#[derive(Debug)]
pub struct Range {
    comparator_sets: Vec<ComparatorSet>,
}

impl Range {
    pub fn new() -> Range {
        Range {
            comparator_sets: vec![],
        }
    }

    // Create a version range representing a wildcard expression.
    pub fn wild() -> Range {
        Range {
            comparator_sets: vec![ComparatorSet::wild()],
        }
    }

    pub fn matches(&self, test_version: &Version) -> bool {
        if self.comparator_sets.is_empty() {
            false
        } else {
            self.comparator_sets
                .iter()
                .any(|cmp_set| cmp_set.matches(test_version))
        }
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Range) -> bool {
        for set in &self.comparator_sets {
            if !other.comparator_sets.contains(set) {
                return false;
            }
        }
        true
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for set in &self.comparator_sets {
            writeln!(f, "{set}")?;
        }

        Ok(())
    }
}
