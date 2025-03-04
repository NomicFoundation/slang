use core::panic;
use std::cmp::{Ord, Ordering};
use std::fmt::Display;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version({}, {}, {})", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => self.patch.cmp(&other.patch),
            },
        }
    }
}

impl From<&semver::Version> for Version {
    fn from(v: &semver::Version) -> Version {
        Version {
            major: u32::try_from(v.major).unwrap(),
            minor: u32::try_from(v.minor).unwrap(),
            patch: u32::try_from(v.patch).unwrap(),
        }
    }
}

impl From<semver::Version> for Version {
    fn from(v: semver::Version) -> Version {
        Version {
            major: u32::try_from(v.major).unwrap(),
            minor: u32::try_from(v.minor).unwrap(),
            patch: u32::try_from(v.patch).unwrap(),
        }
    }
}

#[derive(Copy, Clone)]
enum VersionSegment {
    None,
    Wildcard,
    Specified(u32),
}

impl VersionSegment {
    fn is_wild(self) -> bool {
        matches!(self, Self::Wildcard)
    }

    fn is_none(self) -> bool {
        matches!(self, VersionSegment::None)
    }

    fn is_specified(self) -> bool {
        matches!(self, VersionSegment::Specified(_))
    }

    fn unwrap(self) -> u32 {
        match self {
            VersionSegment::Specified(v) => v,
            VersionSegment::Wildcard => panic!("Tried to unwrap a wildcard segment"),
            VersionSegment::None => panic!("Tried to unwrap an unspecified segment"),
        }
    }
    
}

impl Default for VersionSegment {
    fn default() -> Self {
        Self::None
    }
}

impl From<VersionSegment> for u32 {
    fn from(segment: VersionSegment) -> u32 {
        match segment {
            VersionSegment::Specified(v) => v,
            _ => 0,
        }
    }
}

impl Display for VersionSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => {},
            Self::Wildcard => write!(f, "*")?,
            Self::Specified(v) => write!(f, "{v}")?,
        }

        Ok(())
    }
}

/// A version that may not have specified all values. In general unspecified values will
/// convert to `0` when resolved, however it is often important to know which values were _unspecified_
/// (as opposed to being _specified as `0`_) when resolving ranges.
#[derive(Default, Copy, Clone)]
struct PartialVersion {
    major: VersionSegment,
    minor: VersionSegment,
    patch: VersionSegment,
}

impl PartialVersion {
    fn wild() -> PartialVersion {
        PartialVersion {
            major: VersionSegment::Wildcard,
            minor: VersionSegment::Wildcard,
            patch: VersionSegment::Wildcard,
        }
    }

    fn parse(scanner: &mut Scanner<'_>) -> Result<PartialVersion, ParseError> {
        if scanner.eof() || scanner.accept("*") || scanner.accept("x") || scanner.accept("X") {
            return Ok(PartialVersion::wild());
        }

        let mut partial = PartialVersion::default();
        // We've established that the string isn't empty or a wildcard, so at least the major version
        // must be specified.
        scanner.skip_whitespace();

        partial.major = VersionSegment::Specified(scanner.expect_int()?);

        scanner.skip_whitespace();

        // The remaining versions are optional
        let mut has_seen_wildcard = false;
        if scanner.accept(".") {
            scanner.skip_whitespace();
            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    partial.minor = VersionSegment::Wildcard;
                    has_seen_wildcard = true;
                    scanner.next();
                }
                Some(_) => partial.minor = VersionSegment::Specified(scanner.expect_int()?),
                None => return Err(scanner.error("Expected digit or wildcard in minor position")),
            }
        }

        scanner.skip_whitespace();

        if scanner.accept(".") {
            scanner.skip_whitespace();

            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    partial.patch = VersionSegment::Wildcard;
                    scanner.next();
                }
                Some(_) => {
                    if has_seen_wildcard {
                        return Err(scanner.error("Cannot specify concrete patch version after a wildcard minor version."));
                    }
                    partial.patch = VersionSegment::Specified(scanner.expect_int()?);
                },
                None => return Err(scanner.error("Expected a digit or wildcard in patch position")),
            }
        }
            

        Ok(partial)
    }

    /// Check if `PartialVersion` has no specified values.
    fn is_wild(&self) -> bool {
        self.major.is_wild() && self.minor.is_wild() && self.patch.is_wild()
    }

    /// Check if `PartialVersion` has any unspecified values.
    // fn is_incomplete(&self) -> bool {
    //     !(self.major.is_specified() && self.minor.is_specified() && self.patch.is_specified())
    // }

    fn is_complete(&self) -> bool {
        self.major.is_specified() && self.minor.is_specified() && self.patch.is_specified()
    }

}

impl From<PartialVersion> for Version {
    fn from(partial: PartialVersion) -> Version {
        Version {
            major: u32::from(partial.major),
            minor: u32::from(partial.minor),
            patch: u32::from(partial.patch),
        }
    }
}

impl From<&PartialVersion> for Version {
    fn from(partial: &PartialVersion) -> Version {
        Version {
            major: u32::from(partial.major),
            minor: u32::from(partial.minor),
            patch: u32::from(partial.patch),
        }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Comparator {
    version: Version,
    op: Operator,
}

impl Default for Comparator {
    fn default() -> Comparator {
        Comparator {
            version: Version::default(),
            op: Operator::GtEq,
        }
    }
}

impl Display for Comparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.op, self.version)
    }
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

#[derive(Clone)]
struct ComparatorSet {
    comparators: Vec<Comparator>,
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
                return false
            }
        }

        true
    }
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

    /// Create a `ComparatorSet` which represents the range of versions described by `^[partial]`.
    fn caret(partial: &PartialVersion) -> ComparatorSet {
        let lower = Comparator {
            version: partial.into(),
            op: Operator::GtEq,
        };

        let upper_version = match partial.major {
            VersionSegment::Specified(0) => match partial.minor {
                VersionSegment::Specified(0) => match partial.patch {
                    VersionSegment::Specified(patch) => Version::new(0, 0, patch + 1),
                    _ => Version::new(0, 1, 0),
                },
                VersionSegment::Specified(minor) => Version::new(0, minor + 1, 0),
                _ => Version::new(1, 0, 0),
            },
            VersionSegment::Specified(major) => Version::new(major + 1, 0, 0),
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
            VersionSegment::Specified(minor) => Version::new(major, minor + 1, 0),
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

    /// Create a `ComparatorSet` which represents the range of versions described by `[partial]`. If `partial` is completely specified
    /// then this set will just contain a single comparator, however if `partial` is incomplete then it will describe a range of versions.
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
            VersionSegment::Specified(major) => match partial.minor {
                VersionSegment::Specified(minor) => match partial.patch {
                    // Handled above by checking !partial.is_incomplete()
                    VersionSegment::Specified(_) => unreachable!(),
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
        for &comp in &other.comparators {
            self.comparators.push(comp);
        }
    }

    fn parse(scanner: &mut Scanner<'_>) -> Result<ComparatorSet, ParseError> {
        if scanner.eof() {
            return Err(scanner.error("Tried to parse empty string"));
        }

        // Parse the leading operator, if provided
        let op = match scanner.peek() {
            Some('=') => {
                scanner.expect("=")?;
                scanner.accept("v");
                Some(Operator::Eq)
            },
            Some('v') => {
                scanner.expect("v")?;
                None
            },
            Some('>') => {
                scanner.expect(">")?;
                let op = if scanner.accept("=") {
                    Some(Operator::GtEq)
                } else {
                    Some(Operator::Gt)
                };
                scanner.accept("v");

                op
            },
            Some('<') => {
                scanner.expect("<")?;
                
                let op = if scanner.accept("=") {
                    Some(Operator::LtEq)
                } else {
                    Some(Operator::Lt)
                };
                scanner.accept("v");

                op
            },
            _ => None
        };

        // Parse the partial version string. This may be a concrete version, a wildcard version, or
        // a partial version range.
        let partial = PartialVersion::parse(scanner)?;

        let set = if partial.is_wild() {
            // * ~= >=0.0.0
            ComparatorSet::single(Comparator::default())
        } else if partial.is_complete() {
            // Partial version is complete i.e. it represents a concrete version, not a range
            ComparatorSet::single(Comparator{
                version: partial.into(),
                op: op.unwrap_or(Operator::Eq),
            })
        } else {
            // Not all of the values for this version were provided, so this is a partial version range.
            // How we handle this depends on whether or not the user put an operator in front of the range.
            match op {
                Some(Operator::Lt) => {
                    // <0.7 ~= <0.7.x ~= <0.7.0
                    ComparatorSet::single(Comparator{
                        version: partial.into(),
                        op: Operator::Lt,    
                    })
                },
                Some(Operator::LtEq) => {
                    let comparator = if partial.minor.is_wild() || partial.patch.is_wild() {
                        // <=0.7.x ~= <0.8.0
                        Comparator{
                            version: ComparatorSet::tilde(&partial).comparators[1].version,
                            op: Operator::Lt,
                        }
                    } else {
                        // <=0.7 ~= <=0.7.0
                        Comparator{
                            version: partial.into(),
                            op: Operator::LtEq,
                        }
                    };

                    ComparatorSet::single(comparator)
                },
                Some(Operator::Gt) => {
                    let comparator = if partial.minor.is_wild() || partial.patch.is_wild() {
                        // >0.7.x ~= >0.7.0
                        Comparator{
                            version: partial.into(),
                            op: Operator::Gt,
                        }
                    } else {
                        // >0.7 ~= >=0.8.0
                        Comparator{
                            version: ComparatorSet::tilde(&partial).comparators[1].version,
                            op: Operator::GtEq,
                        }
                    };

                    ComparatorSet::single(comparator)
                },
                Some(Operator::GtEq) => {
                    // >=0.7 ~= >=0.7.x >=0.7.0
                    ComparatorSet::single(Comparator{
                        version: partial.into(),
                        op: Operator::GtEq,    
                    })
                },
                Some(Operator::Eq) | None => {
                    // Treat '=[range]' as the same as '[range]'
                    ComparatorSet::partial_range(&partial)
                }
            }
        };

        Ok(set)
    }

    fn matches(&self, test_version: &Version) -> bool {
        self.comparators.iter().all(|cmp| cmp.matches(test_version))
    }
}

pub struct Range {
    comparator_sets: Vec<ComparatorSet>,
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

impl Range {
    pub fn new() -> Range {
        Range {
            comparator_sets: vec![],
        }
    }

    pub fn matches(&self, test_version: &Version) -> bool {
        self.comparator_sets
            .iter()
            .any(|cmp_set| cmp_set.matches(test_version))
    }
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
    position: usize,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError(pos = {}) : {}", self.position, self.message)
    }
}

struct Scanner<'a> {
    data: &'a str,
    index: usize,
}

impl<'a> Scanner<'a> {
    fn new(data: &'a str) -> Scanner<'a> {
        Scanner { data, index: 0 }
    }

    fn accept(&mut self, value: &str) -> bool {
        if self.data[self.index..].starts_with(value) {
            self.index += value.len();
            true
        } else {
            false
        }
    }

    fn accept_any(&mut self, values: &[&str]) -> bool {
        for v in values {
            if self.accept(v) {
                return true;
            }
        }

        false
    }

    fn expect(&mut self, value: &str) -> Result<(), ParseError> {
        if self.accept(value) {
            Ok(())
        } else {
            let err = if self.eof() {
                self.error(format!("Expected {value}, found eof"))
            } else {
                self.error(format!(
                    "Expected {value}, found {}",
                    self.data.chars().nth(self.index).unwrap()
                ))
            };

            Err(err)
        }
    }

    fn accept_int(&mut self) -> Option<u32> {
        let start = self.index;

        let num_digits = self
            .data
            .chars()
            .skip(start)
            .take_while(|c| c.is_ascii_digit())
            .count();

        if num_digits == 0 {
            return None;
        }

        let mut result = 0;

        let digits = self.data.chars().skip(start).map_while(|c| c.to_digit(10));
        for (i, d) in digits.enumerate() {
            result +=
                d * 10_u32.pow(u32::try_from(num_digits).unwrap() - u32::try_from(i).unwrap() - 1);
        }

        self.index += num_digits;

        Some(result)
    }

    fn expect_int(&mut self) -> Result<u32, ParseError> {
        if let Some(i) = self.accept_int() {
            Ok(i)
        } else {
            let err = if self.eof() {
                self.error("Expected int, found eof")
            } else {
                self.error(format!(
                    "Expected int, found {}",
                    self.data.chars().nth(self.index).unwrap()
                ))
            };

            Err(err)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '"' || c == '\'' {
                self.index += 1;
            } else {
                break;
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.index += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.data.chars().nth(self.index)
    }

    fn eof(&self) -> bool {
        self.index >= self.data.len()
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            message: message.into(),
            position: self.index,
        }
    }
}

pub fn parse(text: &str) -> Result<Range, ParseError> {
    let mut range = Range::new();

    for set_text in text.split("||") {
        let mut scanner = Scanner::new(set_text);
        scanner.skip_whitespace();

        let mut subset = ComparatorSet::new();
        while !scanner.eof() {
            let partial_subset = if set_text.contains('-') {
                // solc allows users to specify range operators ('^', '~') in hyphen ranges, but ignores them.
                // We'll do the same here.
            
                // Ignore all leading '^' and '~'
                while scanner.accept_any(&["^", "~"]) {}

                let lower_version = PartialVersion::parse(&mut scanner)?;

                scanner.skip_whitespace();
                scanner.expect("-")?;
                scanner.skip_whitespace();

                // Ignore all leading '^' and '~'
                while scanner.accept_any(&["^", "~"]) {}

                let upper_version = PartialVersion::parse(&mut scanner)?;

                ComparatorSet::hyphen_range(&lower_version, &upper_version)
            } else if scanner.accept("^") {
                let partial = PartialVersion::parse(&mut scanner)?;
                ComparatorSet::caret(&partial)
            } else if scanner.accept("~") {
                let partial = PartialVersion::parse(&mut scanner)?;
                ComparatorSet::tilde(&partial)
            } else {
                let mut set = ComparatorSet::new();

                while let Ok(comparator) = ComparatorSet::parse(&mut scanner) {
                    set.merge(&comparator);
                    scanner.skip_whitespace();
                }

                set
            };

            subset.merge(&partial_subset);
            scanner.skip_whitespace();
        }

        range.comparator_sets.push(subset);
    }

    Ok(range)
}

#[test]
fn single_version_range() {
    let range = parse("1.2.3").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 3)]);
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(1, 0, 0)]);
}

#[test]
fn less_than_version() {
    let range = parse("<1.3.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 2, 100),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(2, 0, 0),
        ],
    );
}

#[test]
fn less_than_version_v() {
    let range = parse("<v1.3.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 2, 100),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(2, 0, 0),
        ],
    );
}

#[test]
fn less_than_equal_version() {
    let range = parse("<=1.3.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 2, 0),
            Version::new(1, 2, 100),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 3, 5), Version::new(2, 0, 0)]);
}

#[test]
fn empty_version() {
    let range = parse("").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 0, 0),
            Version::new(1, 212, 1),
            Version::new(20, 1, 10),
        ],
    );
}

#[test]
fn wildcard_version() {
    let range = parse("*").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 0, 0),
            Version::new(1, 212, 1),
            Version::new(20, 1, 10),
        ],
    );
}

#[test]
fn gteq_partial_version() {
    let range = parse(">=1.3").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(1, 4, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(0, 8, 0),
        ],
    );
}

#[test]
fn gteq_wildcard_version() {
    let range = parse(">=1.3.x").unwrap();

    test_range_match(&range, &vec![Version::new(1, 3, 0), Version::new(1, 3, 4), Version::new(1, 4, 0), Version::new(2, 3, 3)]);
    test_range_match_fail(&range, &vec![Version::new(1, 2, 1), Version::new(0, 8, 0)]);
}

#[test]
fn gt_partial_version() {
    let range = parse(">1.3").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 4, 0),
            Version::new(2, 5, 3),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(1, 2, 0),
            Version::new(0, 8, 0),
        ],
    );
}

#[test]
fn gt_wildcard_version() {
    let range = parse(">1.3.x").unwrap();

    test_range_match(&range, &vec![Version::new(1, 4, 0), Version::new(2, 1, 0), Version::new(1, 3, 5)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(1, 2, 0), Version::new(0, 8, 0)]);
}

#[test]
fn lteq_partial_version() {
    let range = parse("<=0.8").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5), Version::new(0, 8, 0)]);
    test_range_match_fail(&range, &vec![Version::new(0, 8, 5), Version::new(1, 2, 0)]);
}

#[test]
fn lteq_wildcard_version() {
    let range = parse("<=0.8.x").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5), Version::new(0, 8, 0), Version::new(0, 8, 5)]);
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(0, 9, 1)]);
}

#[test]
fn lt_partial_version() {
    let range = parse("<0.8").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5)]);
    test_range_match_fail(&range, &vec![Version::new(0, 8, 0), Version::new(0, 8, 5), Version::new(1, 2, 0)]);
}

#[test]
fn lt_wildcard_version() {
    let range = parse("<0.8.x").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5)]);
    test_range_match_fail(&range, &vec![Version::new(0, 8, 0), Version::new(0, 8, 5), Version::new(1, 2, 0)]);
}

#[test]
fn partial_version() {
    let range = parse("1.2").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 3)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 2),
            Version::new(2, 0, 0),
            Version::new(1, 0, 0),
        ],
    );
}

#[test]
fn combo_partial_version() {
    let range = parse(">=1 <1.8").unwrap();

    test_range_match(&range, &vec![Version::new(1, 0, 0), Version::new(1, 5, 0), Version::new(1, 7, 9)]);
    test_range_match_fail(&range, &vec![Version::new(0, 9, 9), Version::new(3, 0, 0), Version::new(2, 8, 3)]);
}

#[test]
fn x_patch_partial_version() {
    let range = parse("1.2.x").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 2, 4),
            Version::new(1, 2, 9),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(2, 0, 0)]);
}

#[test]
fn x_minor_partial_version() {
    let range = parse("1.X").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 0, 0),
            Version::new(1, 2, 0),
            Version::new(1, 4, 2),
            Version::new(1, 8, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(0, 7, 0)]);
}

#[test]
fn caret_range() {
    let range = parse("^1.2").unwrap();

    test_range_match(&range, &vec![Version::new(1, 9, 9), Version::new(1, 2, 0)]);
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(1, 0, 0)]);
}

#[test]
fn tilde_range() {
    let range = parse("~1.10.1").unwrap();

    test_range_match(
        &range,
        &vec![Version::new(1, 10, 1), Version::new(1, 10, 6)],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(2, 0, 0),
            Version::new(1, 9, 0),
            Version::new(1, 11, 0),
        ],
    );
}

#[test]
fn hyphen_range() {
    let range = parse("1.2 - 1.5.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 5, 1),
            Version::new(1, 3, 17),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 6, 0),
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
            Version::new(1, 5, 5),
        ],
    );
}

#[test]
fn ignore_operators_in_hyphen_ranges() {
    let range = parse("^1.2 - ~1.5.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 5, 1),
            Version::new(1, 3, 17),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 6, 0),
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
            Version::new(1, 5, 5),
        ],
    );
}

#[test]
fn concat_comparators() {
    let range = parse(">1.0.0 <=2.5.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 0, 1),
            Version::new(2, 0, 0),
            Version::new(2, 5, 1),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 0, 0),
            Version::new(2, 5, 2),
            Version::new(0, 5, 0),
            Version::new(3, 0, 0),
        ],
    );
}

#[test]
fn comparator_union() {
    let range = parse("<1.5 || ^2.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 1, 0),
            Version::new(2, 2, 2),
            Version::new(0, 8, 1),
            Version::new(2, 9, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 5, 0),
            Version::new(1, 7, 0),
            Version::new(3, 0, 0),
        ],
    );
}

#[test]
fn allow_inner_quotes() {
    let target_range = parse("0.8").unwrap();
    let target_version = Version::new(0, 8, 0);

    // "0.8" but with different combinations of quotes embeded within
    // solc currently ignores these by stringifying the whole version pragma tokens together before parsing.
    // In practice we should also be doing this, but that happens higher up in the code.
    // Down here in the parser, we're just going to ignore them when we see them.
    let examples = [
        "\"0.8\"",
        "\"0.\" 8",
        "\"0\" .8",
        "0  . \"8\"",
        "0 '.' 8",
        "'0.8'",
        "\"0\".\"8\"",
    ];

    let example_ranges: Vec<Range> = examples.iter().map(|e| parse(e).unwrap()).collect();
    for r in &example_ranges {
        assert!(r == &target_range);
        assert!(r.matches(&target_version));
    }
}
        
#[test]
fn solc_positive_tests() {
    solc_test_case("*", Version::new(1,2,3), true);
	solc_test_case("1.0.0 - 2.0.0", Version::new(1,2,3), true);
	solc_test_case("1.0.0", Version::new(1,0,0), true);
	solc_test_case("1.0", Version::new(1,0,0), true);
	solc_test_case("1", Version::new(1,0,0), true);
	solc_test_case(">=*", Version::new(0,2,4), true);
	solc_test_case("*", Version::new(1,2,3), true);
	solc_test_case(">=1.0.0", Version::new(1,0,0), true);
	solc_test_case(">=1.0.0", Version::new(1,0,1), true);
	solc_test_case(">=1.0.0", Version::new(1,1,0), true);
	solc_test_case(">1.0.0", Version::new(1,0,1), true);
	solc_test_case(">1.0.0", Version::new(1,1,0), true);
	solc_test_case("<=2.0.0", Version::new(2,0,0), true);
	solc_test_case("<=2.0.0", Version::new(1,9999,9999), true);
	solc_test_case("<=2.0.0", Version::new(0,2,9), true);
	solc_test_case("<2.0.0", Version::new(1,9999,9999), true);
	solc_test_case("<2.0.0", Version::new(0,2,9), true);
	solc_test_case(">= 1.0.0", Version::new(1,0,0), true);
	solc_test_case(">=  1.0.0", Version::new(1,0,1), true);
	solc_test_case(">=   1.0.0", Version::new(1,1,0), true);
	solc_test_case("> 1.0.0", Version::new(1,0,1), true);
	solc_test_case(">  1.0.0", Version::new(1,1,0), true);
	solc_test_case("<=   2.0.0", Version::new(2,0,0), true);
	solc_test_case("<= 2.0.0", Version::new(1,9999,9999), true);
	solc_test_case("<=  2.0.0", Version::new(0,2,9), true);
	solc_test_case("<    2.0.0", Version::new(1,9999,9999), true);
	solc_test_case("<\t2.0.0", Version::new(0,2,9), true);
	solc_test_case(">=0.1.97", Version::new(0,1,97), true);
	solc_test_case("0.1.20 || 1.2.4", Version::new(1,2,4), true);
	solc_test_case(">=0.2.3 || <0.0.1", Version::new(0,0,0), true);
	solc_test_case(">=0.2.3 || <0.0.1", Version::new(0,2,3), true);
	solc_test_case(">=0.2.3 || <0.0.1", Version::new(0,2,4), true);
	solc_test_case("\"2.x.x\"", Version::new(2,1,3), true);
	solc_test_case("1.2.x", Version::new(1,2,3), true);
	solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(2,1,3), true);
	solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(1,2,3), true);
	solc_test_case("x", Version::new(1,2,3), true);
	solc_test_case("2.*.*", Version::new(2,1,3), true);
	solc_test_case("1.2.*", Version::new(1,2,3), true);
	solc_test_case("1.2.* || 2.*", Version::new(2,1,3), true);
	solc_test_case("1.2.* || 2.*", Version::new(1,2,3), true);
	solc_test_case("*", Version::new(1,2,3), true);
	solc_test_case("2", Version::new(2,1,2), true);
	solc_test_case("2.3", Version::new(2,3,1), true);
	solc_test_case("~2.4", Version::new(2,4,0), true);
	solc_test_case("~2.4", Version::new(2,4,5), true);
	solc_test_case("~1", Version::new(1,2,3), true);
	solc_test_case("~1.0", Version::new(1,0,2), true);
	solc_test_case("~ 1.0", Version::new(1,0,2), true);
	solc_test_case("~ 1.0.3", Version::new(1,0,12), true);
	solc_test_case(">=1", Version::new(1,0,0), true);
	solc_test_case(">= 1", Version::new(1,0,0), true);
	solc_test_case("<1.2", Version::new(1,1,1), true);
	solc_test_case("< 1.2", Version::new(1,1,1), true);
	solc_test_case("=0.7.x", Version::new(0,7,2), true);
	solc_test_case("<=0.7.x", Version::new(0,7,2), true);
	solc_test_case(">=0.7.x", Version::new(0,7,2), true);
	solc_test_case("<=0.7.x", Version::new(0,6,2), true);
	solc_test_case("~1.2.1 >=1.2.3", Version::new(1,2,3), true);
	solc_test_case("~1.2.1 =1.2.3", Version::new(1,2,3), true);
	solc_test_case("~1.2.1 1.2.3", Version::new(1,2,3), true);
	solc_test_case("~1.2.1 >=1.2.3 1.2.3", Version::new(1,2,3), true);
	solc_test_case("~1.2.1 1.2.3 >=1.2.3", Version::new(1,2,3), true);
	solc_test_case(">=\"1.2.1\" 1.2.3", Version::new(1,2,3), true);
	solc_test_case("1.2.3 >=1.2.1", Version::new(1,2,3), true);
	solc_test_case(">=1.2.3 >=1.2.1", Version::new(1,2,3), true);
	solc_test_case(">=1.2.1 >=1.2.3", Version::new(1,2,3), true);
	solc_test_case(">=1.2", Version::new(1,2,8), true);
	solc_test_case("^1.2.3", Version::new(1,8,1), true);
	solc_test_case("^0.1.2", Version::new(0,1,2), true);
	solc_test_case("^0.1", Version::new(0,1,2), true);
	solc_test_case("^1.2", Version::new(1,4,2), true);
	solc_test_case("^1.2", Version::new(1,2,0), true);
	solc_test_case("^1", Version::new(1,2,0), true);
	solc_test_case("<=1.2.3", Version::new(1,2,3), true);
	solc_test_case(">1.2", Version::new(1,3,0), true);
	solc_test_case("^1.2 ^1", Version::new(1,4,2), true);
	solc_test_case("^0", Version::new(0,5,1), true);
	solc_test_case("^0", Version::new(0,1,1), true);
}

#[test]
fn solc_negative_tests() {
    solc_test_case("^0^1", Version::new(0,0,0), false);
	solc_test_case("^0^1", Version::new(1,0,0), false);
	solc_test_case("1.0.0 - 2.0.0", Version::new(2,2,3), false);
	solc_test_case("1.0.0", Version::new(1,0,1), false);
	solc_test_case(">=1.0.0", Version::new(0,0,0), false);
	solc_test_case(">=1.0.0", Version::new(0,0,1), false);
	solc_test_case(">=1.0.0", Version::new(0,1,0), false);
	solc_test_case(">1.0.0", Version::new(0,0,1), false);
	solc_test_case(">1.0.0", Version::new(0,1,0), false);
	solc_test_case("<=2.0.0", Version::new(3,0,0), false);
	solc_test_case("<=2.0.0", Version::new(2,9999,9999), false);
	solc_test_case("<=2.0.0", Version::new(2,2,9), false);
	solc_test_case("<2.0.0", Version::new(2,9999,9999), false);
	solc_test_case("<2.0.0", Version::new(2,2,9), false);
	solc_test_case(">=0.1.97", Version::new(0,1,93), false);
	solc_test_case("0.1.20 || 1.2.4", Version::new(1,2,3), false);
	solc_test_case(">=0.2.3 || <0.0.1", Version::new(0,0,3), false);
	solc_test_case(">=0.2.3 || <0.0.1", Version::new(0,2,2), false);
	solc_test_case("\"2.x.x\"", Version::new(1,1,3), false);
	solc_test_case("\"2.x.x\"", Version::new(3,1,3), false);
	solc_test_case("1.2.x", Version::new(1,3,3), false);
	solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(3,1,3), false);
	solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(1,1,3), false);
	solc_test_case("2.*.*", Version::new(1,1,3), false);
	solc_test_case("2.*.*", Version::new(3,1,3), false);
	solc_test_case("1.2.*", Version::new(1,3,3), false);
	solc_test_case("1.2.* || 2.*", Version::new(3,1,3), false);
	solc_test_case("1.2.* || 2.*", Version::new(1,1,3), false);
	solc_test_case("2", Version::new(1,1,2), false);
	solc_test_case("2.3", Version::new(2,4,1), false);
	solc_test_case("~2.4", Version::new(2,5,0), false);
	solc_test_case("~2.4", Version::new(2,3,9), false);
	solc_test_case("~1", Version::new(0,2,3), false);
	solc_test_case("~1.0", Version::new(1,1,0), false);
	solc_test_case("<1", Version::new(1,0,0), false);
	solc_test_case(">=1.2", Version::new(1,1,1), false);
	solc_test_case("=0.7.x", Version::new(0,8,2), false);
	solc_test_case(">=0.7.x", Version::new(0,6,2), false);
	solc_test_case("<0.7.x", Version::new(0,7,2), false);
	solc_test_case(">1.2", Version::new(1,2,8), false);
	solc_test_case("^1.2.3", Version::new(2,0,0), false);
	solc_test_case("^1.2.3", Version::new(1,2,2), false);
	solc_test_case("^1.2", Version::new(1,1,9), false);
	solc_test_case("^0", Version::new(1,0,0), false);
}

#[allow(dead_code)]
fn test_range_match(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(range.matches(t));
    }
}

#[allow(dead_code)]
fn test_range_match_fail(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(!range.matches(t));
    }
}

#[allow(dead_code)]
fn solc_test_case(range_str: &str, version: Version, positive: bool) {
    let range = parse(range_str).unwrap();

    if positive {
        assert!(range.matches(&version));
    } else {
        assert!(!range.matches(&version));
    }
}
