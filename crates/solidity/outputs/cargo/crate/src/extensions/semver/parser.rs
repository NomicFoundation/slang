use std::fmt::Display;

use super::{Comparator, ComparatorSet, Operator, PartialVersion, Range, VersionPart};

pub fn parse(text: &str) -> Range {
    // Special case, "" == "*"
    if text.is_empty() {
        return Range::wild();
    }

    let mut range = Range::new();
    for set_text in text.split("||") {
        let mut scanner = Scanner::new(set_text);
        scanner.skip_whitespace();

        let mut subset = ComparatorSet::new();
        while !scanner.eof() {
            if let Ok(partial_subset) = ComparatorSet::parse(&mut scanner) {
                subset.merge(&partial_subset);
            } else {
                scanner.skip_non_whitespace();
            }

            scanner.skip_whitespace();
        }

        range.comparator_sets.push(subset);
    }

    range
}

trait Parse
where
    Self: std::marker::Sized,
{
    fn parse(text: &mut Scanner<'_>) -> Result<Self, ParseError>;
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

impl Parse for PartialVersion {
    /// Parse a single `PartialVersion`. It's important to remember that this function will give us whatever
    /// version numbers were specified, but it doesn't tell you what those numbers _mean_. That has to be
    /// determined by the context that this partial version is seen in.
    fn parse(scanner: &mut Scanner<'_>) -> Result<PartialVersion, ParseError> {
        if scanner.eof() || scanner.accept("*") || scanner.accept("x") || scanner.accept("X") {
            scanner.skip_whitespace();
            if scanner.accept(".") {
                return Err(
                    scanner.error("Cannot specify minor version after wildcard major version")
                );
            }
            return Ok(PartialVersion::wild());
        }

        let mut partial = PartialVersion::default();

        scanner.skip_whitespace();
        partial.major = VersionPart::Specified(scanner.expect_int()?);
        scanner.skip_whitespace();

        // The remaining parts are optional
        if scanner.accept(".") {
            scanner.skip_whitespace();
            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    scanner.next();
                    partial.minor = VersionPart::Wildcard;
                }
                Some(_) => partial.minor = VersionPart::Specified(scanner.expect_int()?),
                None => return Err(scanner.error("Expected digit or wildcard in minor position")),
            }
        }

        scanner.skip_whitespace();

        if scanner.accept(".") {
            scanner.skip_whitespace();

            match scanner.peek() {
                Some('X' | 'x' | '*') => {
                    scanner.next();
                    partial.patch = VersionPart::Wildcard;
                }
                Some(_) => {
                    if partial.minor.is_wild() {
                        return Err(scanner.error(
                            "Cannot specify concrete patch version after a wildcard minor version.",
                        ));
                    }
                    partial.patch = VersionPart::Specified(scanner.expect_int()?);
                }
                None => return Err(scanner.error("Expected a digit or wildcard in patch position")),
            }
        }

        Ok(partial)
    }
}

impl Parse for Operator {
    /// Parse the leading operator for a version. Will also eat any leading 'v' that the user
    /// might have added.
    /// This will return an error if no operator is found, but that is usually an acceptable case.
    fn parse(scanner: &mut Scanner<'_>) -> Result<Operator, ParseError> {
        match scanner.peek() {
            Some('=') => {
                scanner.expect("=")?;
                scanner.accept("v");
                Ok(Operator::Eq)
            }
            Some('v') => {
                scanner.expect("v")?;
                Ok(Operator::Eq)
            }
            Some('>') => {
                scanner.expect(">")?;
                let op = if scanner.accept("=") {
                    Operator::GtEq
                } else {
                    Operator::Gt
                };
                scanner.accept("v");

                Ok(op)
            }
            Some('<') => {
                scanner.expect("<")?;

                let op = if scanner.accept("=") {
                    Operator::LtEq
                } else {
                    Operator::Lt
                };
                scanner.accept("v");

                Ok(op)
            }
            x => Err(scanner.error(format!("Expected operator but found {x:?} instead"))),
        }
    }
}

impl Parse for ComparatorSet {
    /// Parse a `ComparatorSet`.
    ///
    /// If you're looking for `Comparator::parse`, you should look at this function instead. Because of
    /// partial version ranges it makes more sense to assume that you're going to get a `ComparatorSet` when
    /// parsing a version string.
    /// If the version string is a concrete/fully-specified version, then this will return a `ComparatorSet`
    /// containing a single `Comparator`, which is completely valid.
    fn parse(scanner: &mut Scanner<'_>) -> Result<ComparatorSet, ParseError> {
        // Hyphen Range
        if scanner.data.contains('-') {
            // solc allows users to specify range operators ('^', '~') in hyphen ranges, but ignores them.
            // We'll do the same here.

            // Ignore all leading '^' and '~'
            while scanner.accept_any(&["^", "~"]) {}

            let lower_version = PartialVersion::parse(scanner)?;

            scanner.skip_whitespace();
            scanner.expect("-")?;
            scanner.skip_whitespace();

            // Ignore all leading '^' and '~'
            while scanner.accept_any(&["^", "~"]) {}

            let upper_version = PartialVersion::parse(scanner)?;

            return Ok(ComparatorSet::hyphen_range(&lower_version, &upper_version));
        }

        // Caret Range
        if scanner.accept("^") {
            let partial = PartialVersion::parse(scanner)?;
            return Ok(ComparatorSet::caret(&partial));
        }

        // Tilde Range
        if scanner.accept("~") {
            let partial = PartialVersion::parse(scanner)?;
            return Ok(ComparatorSet::tilde(&partial));
        }

        // Partial Version Range (X-Range)
        let op = Operator::parse(scanner).ok();
        let partial = PartialVersion::parse(scanner)?;

        if partial.is_wild() {
            return Ok(ComparatorSet::wild());
        }

        if partial.is_complete() {
            return Ok(ComparatorSet::single(Comparator {
                version: partial.into(),
                op: op.unwrap_or(Operator::Eq),
            }));
        }

        if op.is_none() || op == Some(Operator::Eq) {
            // Treat '=[range]' as the same as '[range]'
            return Ok(ComparatorSet::partial_range(&partial));
        }

        // This is a partial version range, but the user has provided an operator other than '='.
        // We need to handle these differently depending on which operator user provided and whether
        // the partial version was not fully specified or if the user used wildcards.
        let comparator = match op {
            Some(Operator::Lt) => {
                // <0.7 == <0.7.x == <0.7.0
                Comparator {
                    version: partial.into(),
                    op: Operator::Lt,
                }
            }
            Some(Operator::LtEq) => {
                if partial.minor.is_wild() || partial.patch.is_wild() {
                    // <=0.7.x == <0.8.0
                    Comparator {
                        version: ComparatorSet::tilde(&partial).comparators[1].version,
                        op: Operator::Lt,
                    }
                } else {
                    // <=0.7 == <=0.7.0
                    Comparator {
                        version: partial.into(),
                        op: Operator::LtEq,
                    }
                }
            }
            Some(Operator::Gt) => {
                if partial.minor.is_wild() || partial.patch.is_wild() {
                    // >0.7.x == >0.7.0
                    Comparator {
                        version: partial.into(),
                        op: Operator::Gt,
                    }
                } else {
                    // >0.7 == >=0.8.0
                    Comparator {
                        version: ComparatorSet::tilde(&partial).comparators[1].version,
                        op: Operator::GtEq,
                    }
                }
            }
            Some(Operator::GtEq) => {
                // >=0.7 == >=0.7.x >=0.7.0
                Comparator {
                    version: partial.into(),
                    op: Operator::GtEq,
                }
            }
            Some(Operator::Eq) | None => unreachable!(), // Handled above
        };

        Ok(ComparatorSet::single(comparator))
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

    fn expect_int(&mut self) -> Result<u32, ParseError> {
        let digits: String = self
            .data
            .chars()
            .skip(self.index)
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if digits.is_empty() {
            let err = if self.eof() {
                self.error("Expected int, found eof")
            } else {
                self.error(format!(
                    "Expected int, found {}",
                    self.data.chars().nth(self.index).unwrap()
                ))
            };

            return Err(err);
        }

        self.index += digits.len();
        Ok(digits.parse().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.index += 1;
        }
    }

    fn skip_non_whitespace(&mut self) {
        while !self.eof() && !self.is_whitespace() {
            self.index += 1;
        }
    }

    /// Return true if the current character is whitespace. Returns false at eof.
    /// For our purposes '"' and '\'' are treated as whitespace, since they can be
    /// embeded within semvers and must be ignored.
    fn is_whitespace(&self) -> bool {
        if let Some(c) = self.peek() {
            c.is_whitespace() || c == '"' || c == '\''
        } else {
            false
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
