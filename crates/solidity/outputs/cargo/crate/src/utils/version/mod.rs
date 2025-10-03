mod parser;

use std::fmt::Display;

pub use parser::parse_range;
use semver::Version;

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

    fn is_complete(&self) -> bool {
        self.major.is_specified() && self.minor.is_specified() && self.patch.is_specified()
    }

    fn lower_bound(&self) -> Version {
        Version::new(self.major.into(), self.minor.into(), self.patch.into())
    }

    fn caret_upper_bound(&self) -> Option<Version> {
        match self.major {
            VersionPart::Specified(0) => match self.minor {
                VersionPart::Specified(0) => match self.patch {
                    VersionPart::Specified(patch) => Some(Version::new(0, 0, patch + 1)),
                    _ => Some(Version::new(0, 1, 0)),
                },
                VersionPart::Specified(minor) => Some(Version::new(0, minor + 1, 0)),
                _ => Some(Version::new(1, 0, 0)),
            },
            VersionPart::Specified(major) => Some(Version::new(major + 1, 0, 0)),
            _ => None,
        }
    }

    fn tilde_upper_bound(&self) -> Option<Version> {
        match self.major {
            VersionPart::Specified(major) => match self.minor {
                VersionPart::Specified(minor) => Some(Version::new(major, minor + 1, 0)),
                _ => Some(Version::new(major + 1, 0, 0)),
            },
            _ => None,
        }
    }
}

impl Display for PartialVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
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

impl TryFrom<&str> for VersionPart {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            Ok(VersionPart::None)
        } else if s == "x" || s == "X" || s == "*" {
            Ok(VersionPart::Wildcard)
        } else if let Ok(val) = s.parse() {
            Ok(VersionPart::Specified(val))
        } else {
            Err(format!("Could not parse {s} into VersionPart"))
        }
    }
}

impl TryFrom<String> for VersionPart {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        VersionPart::try_from(s.as_str())
    }
}

impl VersionPart {
    fn is_wild(self) -> bool {
        matches!(self, Self::Wildcard)
    }

    fn is_specified(self) -> bool {
        matches!(self, VersionPart::Specified(_))
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
    /// A comparator that will match any version.
    fn wild() -> Comparator {
        Comparator::default()
    }

    /// A comparator that will not match any version.
    fn none() -> Comparator {
        Comparator {
            version: Version::new(0, 0, 0),
            op: Operator::Lt,
        }
    }

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

    fn bounds(lower: Comparator, upper: Comparator) -> ComparatorSet {
        ComparatorSet {
            comparators: vec![lower, upper],
        }
    }

    fn wild() -> ComparatorSet {
        ComparatorSet::single(Comparator::wild())
    }

    fn none() -> ComparatorSet {
        ComparatorSet::single(Comparator::none())
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `^[partial]`.
    fn caret(partial: &PartialVersion) -> ComparatorSet {
        match partial.caret_upper_bound() {
            Some(upper_version) => {
                let upper = Comparator {
                    version: upper_version,
                    op: Operator::Lt,
                };

                let lower = Comparator {
                    version: partial.lower_bound(),
                    op: Operator::GtEq,
                };

                ComparatorSet::bounds(lower, upper)
            }
            None => ComparatorSet::wild(),
        }
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `~[partial]`.
    fn tilde(partial: &PartialVersion) -> ComparatorSet {
        match partial.tilde_upper_bound() {
            Some(upper_version) => {
                let upper = Comparator {
                    version: upper_version,
                    op: Operator::Lt,
                };

                let lower = Comparator {
                    version: partial.lower_bound(),
                    op: Operator::GtEq,
                };

                ComparatorSet::bounds(lower, upper)
            }
            None => ComparatorSet::wild(),
        }
    }

    /// Create a `ComparatorSet` which represents the range of versions described by `[lower_version] - [upper_version]`.
    fn hyphen_range(
        lower_version: &PartialVersion,
        upper_version: &PartialVersion,
    ) -> ComparatorSet {
        let lower = Comparator {
            version: lower_version.lower_bound(),
            op: Operator::GtEq,
        };

        let upper = if upper_version.is_complete() {
            Comparator {
                version: upper_version.lower_bound(),
                op: Operator::LtEq,
            }
        } else {
            upper_version
                .tilde_upper_bound()
                .map_or(Comparator::wild(), |version| Comparator {
                    version,
                    op: Operator::Lt,
                })
        };

        ComparatorSet::bounds(lower, upper)
    }

    /// Create a `ComparatorSet` that represents the range described by `[partial]`.
    /// A partially specified version represents a version range, sometimes called an x-range.
    /// `1.x == >=1.0.0 <2.0.0`
    /// `0.4.x == >=0.4.0 <0.5.0`
    /// `2.1 == >=2.1.0 < 2.2.0`
    fn partial_range(partial: &PartialVersion) -> ComparatorSet {
        if partial.major.is_wild() {
            return ComparatorSet::single(Comparator::default());
        }

        if partial.is_complete() {
            return ComparatorSet::single(Comparator {
                version: partial.lower_bound(),
                op: Operator::Eq,
            });
        }

        let lower = Comparator {
            version: partial.lower_bound(),
            op: Operator::GtEq,
        };

        let upper = Comparator {
            version: partial.tilde_upper_bound().unwrap(),
            op: Operator::Lt,
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
    fn eq(&self, other: &ComparatorSet) -> bool {
        if self.comparators.len() != other.comparators.len() {
            return false;
        }

        for comparator in &self.comparators {
            if !other.comparators.contains(comparator) {
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
        if self.comparator_sets.len() != other.comparator_sets.len() {
            return false;
        }

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
