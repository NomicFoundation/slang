use std::{
    cmp::{max, min},
    fmt::Display,
    ops::Range,
};

use semver::Version;

pub type VersionRange = Range<Version>;

#[derive(Clone, Debug)]
pub struct VersionSet {
    ranges: Vec<VersionRange>,
}

impl VersionSet {
    pub fn from(ranges: Vec<VersionRange>) -> Self {
        for range in &ranges {
            // End is always exclusive:
            assert!(range.start < range.end, "Invalid range: {range:?}");
        }

        return Self { ranges };
    }

    pub fn empty() -> Self {
        return Self::from(vec![]);
    }

    pub fn range(range: VersionRange) -> Self {
        return Self::from(vec![range]);
    }

    pub fn is_empty(&self) -> bool {
        return self.ranges.is_empty();
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut first_iter = self.ranges.iter().peekable();
        let mut second_iter = other.ranges.iter().peekable();
        let mut results = Vec::new();

        loop {
            let first = if let Some(first) = first_iter.peek() {
                first
            } else {
                break;
            };

            let second = if let Some(second) = second_iter.peek() {
                second
            } else {
                break;
            };

            if first.end < second.start {
                // first fully exists before second, take it:
                results.push(first_iter.next().unwrap().to_owned());
                break;
            }

            if second.end < first.start {
                // second fully exists before first, take it:
                results.push(second_iter.next().unwrap().to_owned());
                break;
            }

            // overlap, take the union:
            let lower = min(first.start.to_owned(), second.start.to_owned());
            let upper = max(first.end.to_owned(), second.end.to_owned());

            first_iter.next().unwrap();
            second_iter.next().unwrap();

            results.push(lower..upper);
        }

        results.extend(first_iter.cloned());
        results.extend(second_iter.cloned());

        return Self::from(results);
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut first_iter = self.ranges.iter().peekable();
        let mut second_iter = other.ranges.iter().peekable();
        let mut results = Vec::new();

        loop {
            let first = if let Some(first) = first_iter.peek() {
                first
            } else {
                break;
            };

            let second = if let Some(second) = second_iter.peek() {
                second
            } else {
                break;
            };

            if first.end < second.start {
                // first fully exists before second, take it:
                results.push(first_iter.next().unwrap().to_owned());
                break;
            }

            if second.end < first.start {
                // second fully exists before first, drop it:
                second_iter.next().unwrap();
                break;
            }

            if first.start < second.start {
                // take part of first that exists before second:
                results.push(first.start.to_owned()..second.start.to_owned());
            } else if first.end > second.end {
                // take part of first that exists after second:
                results.push(second.end.to_owned()..first.end.to_owned());
            } else {
                // first fully exists within second, do nothing:
            }

            first_iter.next().unwrap();
            second_iter.next().unwrap();
        }

        results.extend(first_iter.cloned());

        return Self::from(results);
    }

    pub fn max_version() -> Version {
        return Version::new(u64::MAX, u64::MAX, u64::MAX);
    }
}

impl Display for VersionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_version = Self::max_version();

        let mut iter = self.ranges.iter().peekable();

        while let Some(range) = iter.next() {
            if range.start == max_version {
                write!(f, "MAX")?;
            } else {
                write!(f, "{}", range.start)?;
            }

            write!(f, "..")?;

            if range.end == max_version {
                write!(f, "MAX")?;
            } else {
                write!(f, "{}", range.end)?;
            }

            if iter.peek().is_some() {
                write!(f, " | ")?;
            }
        }

        return Ok(());
    }
}
