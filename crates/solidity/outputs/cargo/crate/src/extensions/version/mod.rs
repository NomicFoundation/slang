use core::panic;
use std::{cmp::{Ord, Ordering}, fmt::Display};

struct Scanner {
    data: String,
    index: usize,
}

impl Scanner {
    fn new(data: impl Into<String>) -> Scanner {
        Scanner {
            data: data.into(),
            index: 0,
        }
    }

    fn accept(&mut self, value: &str) -> bool {
        if self.data[self.index..].starts_with(value) {
            self.index += value.len();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, value: &str) -> Result<(), ParseError> {
        if self.accept(value) {
            Ok(())
        } else {
            let err = if self.eof() {
                self.error(format!("Expected {value}, found eof"))
            } else {
                self.error(format!("Expected {value}, found {}", self.data.chars().nth(self.index).unwrap()))
            };

            Err(err)
        }
    }

    fn accept_int(&mut self) -> Option<u32> {
        let start = self.index;

        let digits: Vec<u32> = self.data
            .chars()
            .skip(start)
            .map_while(|c| c.to_digit(10))
            .collect();

        if digits.is_empty() {
            return None;
        }

        let num_digits = digits.len() as u32;
        let mut result = 0;

        for (i, d) in digits.iter().enumerate() {
            result += d * 10_u32.pow(num_digits - (i as u32) - 1);
        }

        self.index += digits.len();

        Some(result)
    }

    fn expect_int(&mut self) -> Result<u32, ParseError> {
        if let Some(i) = self.accept_int() {
            Ok(i)
        } else {
            let err = if self.eof() {
                self.error("Expected int, found eof")
            } else {
                self.error(format!("Expected int, found {}", self.data.chars().nth(self.index).unwrap()))
            };

            Err(err)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
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

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version{
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

/// A version that may not have specified all values. In general unspecified values will
/// convert to `0` when resolved, however it is often important to know which values were _unspecified_
/// (as opposed to being _specified as `0`_) when resolving ranges.
#[derive(Default, Copy, Clone)]
struct PartialVersion {
    major: Option<u32>,
    minor: Option<u32>,
    patch: Option<u32>,
}

impl PartialVersion {
    fn parse_str(text: &str) -> Result<PartialVersion, ParseError> {
        let text = text.trim();
        let mut scanner = Scanner::new(text);

        PartialVersion::parse(&mut scanner)
    }

    fn parse(scanner: &mut Scanner) -> Result<PartialVersion, ParseError> {
        let mut partial = PartialVersion::default();

        if scanner.eof() || scanner.accept("*") {
            return Ok(partial);
        }

        // We've established that the string isn't empty or a wildcard, so at least the major version
        // must be specified.
        partial.major = Some(scanner.expect_int()?);

        // The remaining versions are optional
        let mut has_seen_wildcard = false;
        if scanner.accept(".") {
            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    has_seen_wildcard = true;
                },
                Some(_) => partial.minor = Some(scanner.expect_int()?),
                None => return Err(scanner.error("Expected digit or wildcard in minor position")),
            }
        }

        if scanner.accept(".") {
            if has_seen_wildcard {
                return Err(scanner.error("Cannot specify patch version because a wildcard was set for the minor version."))
            }
            match scanner.peek() {
                Some('X' | 'x' | '*') => {},
                Some(_) => partial.patch = Some(scanner.expect_int()?),
                None => return Err(scanner.error("Expected a digit or wildcard in patch position")),
            }
        }

        Ok(partial)
    }

    /// Resolve this partial version as the upper bound of a caret range. 
    fn resolve_caret(&self) -> Comparator {
        let version = match self.major {
            Some(0) => match self.minor {
                Some(0) => match self.patch {
                    Some(patch) => Version::new(0, 0, patch + 1),
                    None => Version::new(0, 1, 0),
                },
                Some(minor) => Version::new(0, minor + 1, 0),
                None => Version::new(1, 0, 0),
            },
            Some(major) => Version::new(major + 1, 0, 0),
            None => panic!("Major version must be specified when resolving a caret upper bound"),
        };

        Comparator{
            version,
            op: Operator::Lt,
        }
    }

    fn resolve_tilde(&self) -> Comparator {
        let major = self.major.unwrap();

        let version = match self.minor {
            Some(minor) => Version::new(major, minor + 1, 0),
            None => Version::new(major + 1, 0, 0),
        };

        Comparator{
            version,
            op: Operator::Lt,
        }
    }

    fn resolve_partial_range(&self) -> ComparatorSet {
        if self.is_wild() {
            return ComparatorSet{
                comparators: vec![
                    Comparator{
                        version: Version::default(),
                        op: Operator::GtEq,
                    }
                ],
            };
        }

        if !self.is_incomplete() {
            return ComparatorSet{
                comparators: vec![
                    Comparator{
                        version: self.into(),
                        op: Operator::Eq,
                    }
                ]
            }
        }

        let lower = Comparator{
            version: self.into(),
            op: Operator::GtEq,
        };

        let upper = match self.major {
            Some(major) => match self.minor {
                Some(minor) => match self.patch {
                    // Handled above by checking !self.is_incomplete()
                    Some(_) => unreachable!(),
                    None => Comparator{
                        version: Version::new(major, minor + 1, 0),
                        op: Operator::Lt,
                    }
                },
                None => Comparator{
                    version: Version::new(major + 1, 0, 0),
                    op: Operator::Lt,
                }
            },
            // Handled above by checking self.is_wild()
            None => unreachable!(),
        };

        ComparatorSet::bounds(lower, upper)
    }

    fn is_wild(&self) -> bool {
        self.major.is_none() && self.minor.is_none() && self.patch.is_none()   
    }

    fn is_incomplete(&self) -> bool {
        self.major.is_none() || self.minor.is_none() || self.patch.is_none()
    }
}

impl From<PartialVersion> for Version {
    fn from(partial: PartialVersion) -> Version {
        Version {
            major: partial.major.unwrap_or(0),
            minor: partial.minor.unwrap_or(0),
            patch: partial.patch.unwrap_or(0),
        }
    }
}

impl From<&PartialVersion> for Version {
    fn from(partial: &PartialVersion) -> Version {
        Version {
            major: partial.major.unwrap_or(0),
            minor: partial.minor.unwrap_or(0),
            patch: partial.patch.unwrap_or(0),
        }
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
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
}

#[derive(Debug, Copy, Clone)]
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

    fn merge(&mut self, other: &ComparatorSet) {
        for &comp in &other.comparators {
            self.comparators.push(comp);
        }
    }

    fn parse_str(text: &str) -> Result<ComparatorSet, ParseError> {
        let text = text.trim();        
        let mut scanner = Scanner::new(text);
        ComparatorSet::parse(&mut scanner)
    }

    fn parse(scanner: &mut Scanner) -> Result<ComparatorSet, ParseError> {
        if scanner.eof() {
            return Err(scanner.error("Tried to parse empty string"));
        }

        let set = match scanner.peek() {
            Some('=') => {
                scanner.next();
                scanner.accept("v");
                let partial = PartialVersion::parse(scanner)?;
                partial.resolve_partial_range()
            },
            Some('v') => {
                scanner.next();
                let partial = PartialVersion::parse(scanner)?;
                partial.resolve_partial_range()
            },
            Some('>') => {
                scanner.next();
                let op = if scanner.accept("=") {
                    Operator::GtEq
                } else {
                    Operator::Gt
                };

                scanner.accept("v");

                let partial = PartialVersion::parse(scanner)?;

                ComparatorSet::single(Comparator{ op, version: partial.into() })
            },
            Some('<') => {
                scanner.next();
                let op = if scanner.accept("=") {
                    Operator::LtEq
                } else {
                    Operator::Lt
                };

                scanner.accept("v");

                let partial = PartialVersion::parse(scanner)?;
                ComparatorSet::single(Comparator{ op, version: partial.into() })
            },
            Some(_) => {
                scanner.accept("v");

                let partial = PartialVersion::parse(scanner)?;
                partial.resolve_partial_range()
            },
            None => unreachable!(),
        };

        Ok(set)
    }

    fn matches(&self, test_version: &Version) -> bool {
        self.comparators.iter().all(|cmp| cmp.matches(test_version))
    }
}

#[derive(Clone)]
struct Range {
    comparator_sets: Vec<ComparatorSet>,
}

impl Range {
    fn new() -> Range {
        Range {
            comparator_sets: vec![],
        }
    }

    fn matches(&self, test_version: &Version) -> bool {
        self.comparator_sets.iter().any(|cmp_set| cmp_set.matches(test_version))
    }

    fn append_range(&mut self, other: &Range) {
        for set in &other.comparator_sets {
            self.comparator_sets.push(set.clone());
        }
    }
}

#[derive(Debug)]
struct ParseError {
    message: String,
    position: usize,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError(pos = {}) : {}", self.position, self.message)
    }
}

pub fn parse(text: &str) -> Result<Range, ParseError> {
    let mut range = Range::new();
    let sets: Vec<&str> = text.split("||").map(|set| set.trim()).collect();

    for set in &sets {
        let subrange = if set.starts_with('^') {
            parse_caret_range(set)?
        } else if set.starts_with('~') {
            parse_tilde_range(set)?
        } else if set.contains('-') {
            parse_hyphen_range(set)?
        } else {
            parse_primitive_range(set)?
        };

        range.comparator_sets.push(subrange);
    }

    Ok(range)
}

fn parse_primitive_range(text: &str) -> Result<ComparatorSet, ParseError> {
    let mut set = ComparatorSet::new();

    for comp in text.split_whitespace() {
        let subset = ComparatorSet::parse_str(comp)?;
        set.merge(&subset);
    }

    Ok(set)
}

fn parse_hyphen_range(text: &str) -> Result<ComparatorSet, ParseError> {
    let parts: Vec<&str> = text.split('-').collect();

    if parts.len() != 2 {
        return Err(ParseError{
            position: 0,
            message: format!("Hyphen range requires exactly 2 parts, found {}", parts.len()),
        });
    }

    let lower = Comparator{
        version: PartialVersion::parse_str(parts[0])?.into(),
        op: Operator::GtEq,
    };

    let upper_version = PartialVersion::parse_str(parts[1])?;
    let upper = if upper_version.is_incomplete() {
        let mut upper_resolved: Version = upper_version.into();
        if upper_version.minor.is_none() {
            upper_resolved.major += 1;
        } else if upper_version.patch.is_none() {
            upper_resolved.minor += 1;
        }

        Comparator{
            version: upper_resolved,
            op: Operator::Lt,
        }
    } else {
        Comparator{
            version: upper_version.into(),
            op: Operator::LtEq,
        }
    };
    
    Ok(ComparatorSet::bounds(lower, upper))
}

fn parse_caret_range(text: &str) -> Result<ComparatorSet, ParseError> {
    let mut scanner = Scanner::new(text);

    scanner.expect("^")?;

    let partial = PartialVersion::parse(&mut scanner)?;

    let lower = Comparator{
        version: partial.into(),
        op: Operator::GtEq,
    };

    let upper = partial.resolve_caret();

    let set = ComparatorSet{
        comparators: vec![lower, upper],
    };
    
    Ok(set)
}

fn parse_tilde_range(text: &str) -> Result<ComparatorSet, ParseError> {
    let mut scanner = Scanner::new(text);

    scanner.expect("~")?;

    let partial = PartialVersion::parse(&mut scanner)?;

    let lower = Comparator{
        version: partial.into(),
        op: Operator::GtEq,
    };

    let upper = partial.resolve_tilde();

    let set = ComparatorSet{
        comparators: vec![lower, upper],
    };
    
    Ok(set)
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

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 100), Version::new(0, 8, 0)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(1, 3, 5), Version::new(2, 0, 0)]);
}

#[test]
fn less_than_version_v() {
    let range = parse("<v1.3.0").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 100), Version::new(0, 8, 0)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(1, 3, 5), Version::new(2, 0, 0)]);
}

#[test]
fn less_than_equal_version() {
    let range = parse("<=1.3.0").unwrap();

    test_range_match(&range, &vec![Version::new(1, 3, 0), Version::new(1, 2, 0), Version::new(1, 2, 100), Version::new(0, 8, 0)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 5), Version::new(2, 0, 0)]);
}

#[test]
fn partial_version_with_operator() {
    let range = parse(">1.3").unwrap();

    test_range_match(&range, &vec![Version::new(1, 3, 5), Version::new(1, 4, 0), Version::new(2, 0, 0)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(1, 2, 0), Version::new(0, 8, 0)]);
}

#[test]
fn partial_version() {
    let range = parse("1.2").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 3)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 2), Version::new(2, 0, 0), Version::new(1, 0, 0)]);
}

#[test]
fn x_partial_version() {
    let range = parse("1.2.x").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 4), Version::new(1, 2, 9)]);
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(2, 0, 0)]);
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

    test_range_match(&range, &vec![Version::new(1, 10, 1), Version::new(1, 10, 6)]);
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(1, 9, 0), Version::new(1, 11, 0)]);
}

#[test]
fn hyphen_range() {
    let range = parse("1.2 - 1.5.1").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 5, 1), Version::new(1, 3, 17)]);
    test_range_match_fail(&range, &vec![Version::new(1, 6, 0), Version::new(1, 0, 0), Version::new(2, 0, 0), Version::new(1, 5, 5)]);
}

fn test_range_match(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(range.matches(t));
    }
}

fn test_range_match_fail(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(!range.matches(t));
    }
}
