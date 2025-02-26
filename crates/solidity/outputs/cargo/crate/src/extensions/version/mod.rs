use core::panic;
use std::cmp::{Ord, Ordering};

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

    // fn consume_until(&mut self, value: &str) -> Option<&str> {
    //     if self.eof() {
    //         return None;
    //     }

    //     let start = self.index;
    //     while self.next().is_some() {
    //         let end = self.index;
    //         if self.accept(value) {
    //             return Some(&self.data[start..end])
    //         }
    //     }

    //     Some(&self.data[start..self.index])
    // }

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
            Err(ParseError::UnexpectedChar)
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
            result += d * 10 * (num_digits - (i as u32) - 1);
        }

        self.index += digits.len();

        Some(result)
    }

    fn expect_int(&mut self) -> Result<u32, ParseError> {
        if let Some(i) = self.accept_int() {
            Ok(i)
        } else {
            Err(ParseError::UnexpectedChar)
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
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
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

/// A version that may not have specified all values. In most cases unspecified values will
/// convert to `0` when resolved, however it can be important to know which values were _unspecified_
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
        if let Some(major) = scanner.accept_int() {
            partial.major = Some(major);
        } else {
            return Err(ParseError::UnexpectedChar);
        }

        // The remaining versions are optional
        let mut has_seen_wildcard = false;
        if scanner.accept(".") {
            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    has_seen_wildcard = true;
                },
                Some(_) => partial.minor = Some(scanner.expect_int()?),
                None => return Err(ParseError::UnexpectedChar),
            }
        }

        if scanner.accept(".") {
            if has_seen_wildcard {
                return Err(ParseError::UnexpectedChar);
            }
            match scanner.peek() {
                Some('X' | 'x' | '*') => {},
                Some(_) => partial.patch = Some(scanner.expect_int()?),
                None => return Err(ParseError::UnexpectedChar),
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

#[derive(Copy, Clone)]
enum Operator {
    Lt,
    LtEq,
    Gt,
    GtEq,
    Eq,
}

#[derive(Copy, Clone)]
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
    fn parse_str(text: &str) -> Result<Comparator, ParseError> {
        let text = text.trim();        
        let mut scanner = Scanner::new(text);
        Comparator::parse(&mut scanner)
    }

    fn parse(scanner: &mut Scanner) -> Result<Comparator, ParseError> {
        match scanner.next() {
            Some('=') => {
                scanner.accept("v");
                let partial = PartialVersion::parse(scanner)?;
                Ok(Comparator{
                    op: Operator::Eq,
                    version: partial.into(),
                })
            },
            Some('v') => {
                scanner.accept("=");
                let partial = PartialVersion::parse(scanner)?;
                Ok(Comparator{
                    op: Operator::Eq,
                    version: partial.into(),
                })
            },
            Some('>') => {
                let op = if scanner.accept("=") {
                    Operator::GtEq
                } else {
                    Operator::Gt
                };

                let partial = PartialVersion::parse(scanner)?;
                Ok(Comparator{ op, version: partial.into() })
            },
            Some('<') => {
                let op = if scanner.accept("=") {
                    Operator::LtEq
                } else {
                    Operator::Lt
                };

                let partial = PartialVersion::parse(scanner)?;
                Ok(Comparator{ op, version: partial.into() })
            },
            Some(_) => {
                let partial = PartialVersion::parse(scanner)?;
                Ok(Comparator{
                    op: Operator::GtEq,
                    version: partial.into(),
                })
            },
            None => Err(ParseError::UnexpectedChar),
        }
    }

    fn matches(&self, test_version: &Version) -> bool {
        match self.op {
            Operator::Lt => self.version < *test_version,
            Operator::LtEq => self.version <= *test_version,
            Operator::Gt => self.version > *test_version,
            Operator::GtEq => self.version >= *test_version,
            Operator::Eq => self.version == *test_version,
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

    fn push(&mut self, comp: Comparator) {
        self.comparators.push(comp);
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
        for set in other.comparator_sets.iter() {
            self.comparator_sets.push(set.clone());
        }
    }
}

enum ParseError {
    UnexpectedChar,
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

        range.append_range(&subrange);
    }

    Ok(range)
}

fn parse_primitive_range(text: &str) -> Result<Range, ParseError> {
    let mut set = ComparatorSet::new();

    for comp in text.split_whitespace() {
        let comparator = Comparator::parse_str(comp)?;
        set.comparators.push(comparator);
   }

    Ok(Range{ comparator_sets: vec![set] })
}

fn parse_hyphen_range(text: &str) -> Result<Range, ParseError> {
    let parts: Vec<&str> = text.split('-').collect();

    if parts.len() != 2 {
        return Err(ParseError::UnexpectedChar);
    }

    let mut set = ComparatorSet::new();
    
    let lower = PartialVersion::parse_str(parts[0])?;
    let upper = PartialVersion::parse_str(parts[1])?;

    set.push(Comparator { version: lower.into(), op: Operator::GtEq });

    if upper.is_incomplete() {
        let mut upper_resolved: Version = upper.into();
        if upper.minor.is_none() {
            upper_resolved.major += 1;
        } else if upper.patch.is_none() {
            upper_resolved.minor += 1;
        }

        set.push(Comparator{
            version: upper_resolved,
            op: Operator::Lt,
        });
    } else {
        set.push(Comparator{
            version: upper.into(),
            op: Operator::LtEq,
        });
    }
    
    Ok(Range{ comparator_sets: vec![set] })
}

fn parse_caret_range(text: &str) -> Result<Range, ParseError> {
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
    
    Ok(Range{
        comparator_sets: vec![set],
    })
}

fn parse_tilde_range(text: &str) -> Result<Range, ParseError> {
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
    
    Ok(Range{
        comparator_sets: vec![set],
    })
}

