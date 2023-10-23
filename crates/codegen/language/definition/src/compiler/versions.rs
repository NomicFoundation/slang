use semver::Version;
use std::{
    cmp::{max, min},
    fmt::Display,
    iter::once,
};

const MAX_VERSION: Version = Version::new(u64::MAX, u64::MAX, u64::MAX);

#[derive(Clone, Debug)]
pub struct VersionRange {
    pub inclusive_start: Version,
    pub exclusive_end: Version,
}

impl VersionRange {
    pub fn starting_from(inclusive_start: &Version) -> Self {
        return Self {
            inclusive_start: inclusive_start.to_owned(),
            exclusive_end: MAX_VERSION,
        };
    }

    pub fn between(inclusive_start: &Version, exclusive_end: &Version) -> Self {
        if inclusive_start <= exclusive_end {
            return Self {
                inclusive_start: inclusive_start.to_owned(),
                exclusive_end: exclusive_end.to_owned(),
            };
        } else {
            // This is an invalid state, which we still produce errors for.
            // However, for validation to continue, let's correct the order:
            return Self {
                inclusive_start: exclusive_end.to_owned(),
                exclusive_end: inclusive_start.to_owned(),
            };
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.inclusive_start == self.exclusive_end;
    }
}

impl Display for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inclusive_start == MAX_VERSION {
            write!(f, "MAX")?;
        } else {
            write!(f, "{}", self.inclusive_start)?;
        }

        write!(f, "..")?;

        if self.exclusive_end == MAX_VERSION {
            write!(f, "MAX")?;
        } else {
            write!(f, "{}", self.exclusive_end)?;
        }

        return Ok(());
    }
}

#[derive(Debug)]
pub struct VersionSet {
    ranges: Vec<VersionRange>,
}

impl VersionSet {
    pub fn empty() -> Self {
        return Self { ranges: vec![] };
    }

    pub fn single(range: VersionRange) -> Self {
        let mut instance = Self::empty();
        instance.add(&range);
        return instance;
    }

    pub fn is_empty(&self) -> bool {
        return self.ranges.is_empty();
    }

    pub fn add(&mut self, range: &VersionRange) {
        if range.is_empty() {
            return;
        }

        let mut first_iter = self.ranges.iter().peekable();
        let mut second_iter = once(range).into_iter().peekable();
        let mut ranges = vec![];

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

            if first.exclusive_end < second.inclusive_start {
                // first fully exists before second, take it:
                ranges.push(first_iter.next().unwrap().to_owned());
                break;
            }

            if second.exclusive_end < first.inclusive_start {
                // second fully exists before first, take it:
                ranges.push(second_iter.next().unwrap().to_owned());
                break;
            }

            // overlap, take the union:
            let lower = min(&first.inclusive_start, &second.inclusive_start);
            let upper = max(&first.exclusive_end, &second.exclusive_end);

            first_iter.next().unwrap();
            second_iter.next().unwrap();

            ranges.push(VersionRange::between(lower, upper));
        }

        ranges.extend(first_iter.cloned());
        ranges.extend(second_iter.cloned());

        self.ranges = ranges;
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut first_iter = self.ranges.iter().peekable();
        let mut second_iter = other.ranges.iter().peekable();
        let mut ranges = vec![];

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

            if first.exclusive_end < second.inclusive_start {
                // first fully exists before second, take it:
                ranges.push(first_iter.next().unwrap().to_owned());
                break;
            }

            if second.exclusive_end < first.inclusive_start {
                // second fully exists before first, drop it:
                second_iter.next().unwrap();
                break;
            }

            if first.inclusive_start < second.inclusive_start {
                // take part of first that exists before second:
                ranges.push(VersionRange::between(
                    &first.inclusive_start,
                    &second.inclusive_start,
                ));
            } else if first.exclusive_end > second.exclusive_end {
                // take part of first that exists after second:
                ranges.push(VersionRange::between(
                    &second.exclusive_end,
                    &first.exclusive_end,
                ));
            } else {
                // first fully exists within second, do nothing:
            }

            first_iter.next().unwrap();
            second_iter.next().unwrap();
        }

        ranges.extend(first_iter.cloned());

        return Self { ranges };
    }
}

impl Display for VersionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.ranges.iter().peekable();

        while let Some(range) = iter.next() {
            write!(f, "{range}")?;

            if iter.peek().is_some() {
                write!(f, " | ")?;
            }
        }

        return Ok(());
    }
}
