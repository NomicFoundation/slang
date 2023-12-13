use std::cmp::{max, min};
use std::fmt::Display;
use std::iter::once;
use std::ops::Range;

use itertools::Itertools;
use semver::Version;

pub type VersionRange = Range<Version>;

#[derive(Clone, Debug)]
pub struct VersionSet {
    ranges: Vec<VersionRange>,
}

impl VersionSet {
    pub fn empty() -> Self {
        Self { ranges: vec![] }
    }

    pub fn from_range(range: VersionRange) -> Self {
        let mut instance = Self::empty();

        instance.add(&range);

        instance
    }

    #[cfg(test)]
    fn from_ranges(ranges: impl IntoIterator<Item = VersionRange>) -> Self {
        let mut instance = Self::empty();

        for range in ranges {
            instance.add(&range);
        }

        instance
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn add(&mut self, range: &VersionRange) {
        if range.is_empty() {
            return;
        }

        let mut result = vec![];

        // Iterate on both (existing and new) in-order, combining them if they overlap:
        let mut input_iter = self
            .ranges
            .iter()
            .chain(once(range))
            .sorted_by_key(|range| &range.start);

        let mut current = input_iter.next().unwrap().to_owned();

        for next in input_iter {
            if current.end < next.start {
                // current fully exists before next:
                result.push(current);
                current = next.to_owned();
            } else {
                // current and next overlap, combine them:
                current = Range {
                    start: min(&current.start, &next.start).to_owned(),
                    end: max(&current.end, &next.end).to_owned(),
                };
            }
        }

        result.push(current);
        self.ranges = result;
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.to_owned();

        for range in &other.ranges {
            result.add(range);
        }

        result
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = vec![];

        let mut first_iter = self.ranges.iter().cloned().peekable();
        let mut second_iter = other.ranges.iter().peekable();

        loop {
            let (Some(first), Some(second)) = (first_iter.peek_mut(), second_iter.peek()) else {
                break;
            };

            if first.end <= second.start {
                // first fully exists before second: take it, and advance first:
                result.push(first_iter.next().unwrap());
                continue;
            }

            if second.end <= first.start {
                // second fully exists before first: advance second:
                second_iter.next();
                continue;
            }

            // first and second overlap:

            if first.start < second.start {
                // take part of first that exists before second:
                result.push(Range {
                    start: first.start.to_owned(),
                    end: second.start.to_owned(),
                });
            }

            if first.end <= second.end {
                // first ends before second: advance first, as it's been fully processed:
                first_iter.next();
                continue;
            }

            // keep part of first that exists after second:
            first.start = second.end.to_owned();

            // advance second, as it's been fully processed:
            second_iter.next();
        }

        // Take anything remaining in first:
        result.extend(first_iter);

        Self { ranges: result }
    }

    pub fn max_version() -> Version {
        Version::new(u64::MAX, u64::MAX, u64::MAX)
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ranges() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(3, 0, 0)..Version::new(3, 0, 0),
            Version::new(4, 0, 0)..Version::new(5, 0, 0),
        ]);

        assert_eq!(set.to_string(), "1.0.0..2.0.0 | 4.0.0..5.0.0");
    }

    #[test]
    fn connected_ranges_in_order() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(2, 0, 0)..Version::new(3, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
        ]);

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn connected_ranges_out_of_order() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
            Version::new(2, 0, 0)..Version::new(3, 0, 0),
        ]);

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn disconnected_ranges_in_order() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
            Version::new(5, 0, 0)..Version::new(6, 0, 0),
        ]);

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn disconnected_ranges_out_of_order() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(5, 0, 0)..Version::new(6, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
        ]);

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn overlap_with_multiple() {
        let set = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
            Version::new(5, 0, 0)..Version::new(6, 0, 0),
            Version::new(1, 0, 0)..Version::new(5, 0, 0),
        ]);

        assert_eq!(set.to_string(), "1.0.0..6.0.0");
    }

    #[test]
    fn difference_between_same_sets_is_empty() {
        let first = VersionSet::from_ranges(vec![Version::new(1, 0, 0)..Version::new(2, 0, 0)]);

        let second = VersionSet::from_ranges(vec![Version::new(1, 0, 0)..Version::new(2, 0, 0)]);

        assert!(first.difference(&second).is_empty());
    }

    #[test]
    fn difference_between_connected_sets() {
        let first = VersionSet::from_ranges(vec![Version::new(1, 0, 0)..Version::new(5, 0, 0)]);

        let second = VersionSet::from_ranges(vec![Version::new(3, 0, 0)..Version::new(8, 0, 0)]);

        assert_eq!(first.difference(&second).to_string(), "1.0.0..3.0.0");

        assert_eq!(second.difference(&first).to_string(), "5.0.0..8.0.0");
    }

    #[test]
    fn difference_between_disconnected_sets() {
        let first = VersionSet::from_ranges(vec![Version::new(1, 0, 0)..Version::new(4, 0, 0)]);

        let second = VersionSet::from_ranges(vec![Version::new(6, 0, 0)..Version::new(10, 0, 0)]);

        assert_eq!(first.difference(&second).to_string(), "1.0.0..4.0.0");

        assert_eq!(second.difference(&first).to_string(), "6.0.0..10.0.0");
    }

    #[test]
    fn difference_between_contained_sets() {
        let first = VersionSet::from_ranges(vec![Version::new(1, 0, 0)..Version::new(8, 0, 0)]);

        let second = VersionSet::from_ranges(vec![Version::new(3, 0, 0)..Version::new(5, 0, 0)]);

        assert_eq!(
            first.difference(&second).to_string(),
            "1.0.0..3.0.0 | 5.0.0..8.0.0"
        );

        assert!(second.difference(&first).is_empty());
    }

    #[test]
    fn difference_between_multiple_contained_sets() {
        let first = VersionSet::from_ranges(vec![
            Version::new(1, 0, 0)..Version::new(2, 0, 0),
            Version::new(3, 0, 0)..Version::new(4, 0, 0),
            Version::new(5, 0, 0)..Version::new(6, 0, 0),
        ]);

        let second = VersionSet::from_ranges(vec![Version::new(0, 0, 0)..Version::new(7, 0, 0)]);

        assert!(first.difference(&second).is_empty());

        assert_eq!(
            second.difference(&first).to_string(),
            "0.0.0..1.0.0 | 2.0.0..3.0.0 | 4.0.0..5.0.0 | 6.0.0..7.0.0"
        );
    }
}
