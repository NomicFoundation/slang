use itertools::Itertools;
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
            "MAX".fmt(f)?;
        } else {
            self.inclusive_start.fmt(f)?;
        }

        "..".fmt(f)?;

        if self.exclusive_end == MAX_VERSION {
            "MAX".fmt(f)?;
        } else {
            self.exclusive_end.fmt(f)?;
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

    pub fn from_range(range: VersionRange) -> Self {
        let mut instance = Self::empty();

        instance.add(&range);

        return instance;
    }

    pub fn from_ranges(ranges: impl IntoIterator<Item = VersionRange>) -> Self {
        let mut instance = Self::empty();

        for range in ranges.into_iter() {
            instance.add(&range);
        }

        return instance;
    }

    pub fn is_empty(&self) -> bool {
        return self.ranges.is_empty();
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
            .sorted_by_key(|range| &range.inclusive_start);

        let mut current = input_iter.next().unwrap().to_owned();

        for next in input_iter {
            if current.exclusive_end < next.inclusive_start {
                // current fully exists before next:
                result.push(current);
                current = next.to_owned();
            } else {
                // current and next overlap, combine them:
                current = VersionRange::between(
                    min(&current.inclusive_start, &next.inclusive_start),
                    max(&current.exclusive_end, &next.exclusive_end),
                );
            }
        }

        result.push(current);
        self.ranges = result;
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = vec![];

        let mut first_iter = self.ranges.iter().cloned().peekable();
        let mut second_iter = other.ranges.iter().peekable();

        loop {
            let (Some(first), Some(second)) = (first_iter.peek_mut(), second_iter.peek()) else {
                break;
            };

            if first.exclusive_end <= second.inclusive_start {
                // first fully exists before second: take it, and advance first:
                result.push(first_iter.next().unwrap());
                continue;
            }

            if second.exclusive_end <= first.inclusive_start {
                // second fully exists before first: advance second:
                second_iter.next();
                continue;
            }

            // first and second overlap:

            if first.inclusive_start < second.inclusive_start {
                // take part of first that exists before second:
                result.push(VersionRange::between(
                    &first.inclusive_start,
                    &second.inclusive_start,
                ));
            }

            if first.exclusive_end <= second.exclusive_end {
                // first ends before second: advance first, as it's been fully processed:
                first_iter.next();
                continue;
            }

            // keep part of first that exists after second:
            first.inclusive_start = second.exclusive_end.to_owned();

            // advance second, as it's been fully processed:
            second_iter.next();
        }

        // Take anything remaining in first:
        result.extend(first_iter);

        return Self { ranges: result };
    }
}

impl Display for VersionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return "{EMPTY}".fmt(f);
        }

        for (i, range) in self.ranges.iter().enumerate() {
            if i > 0 {
                " | ".fmt(f)?;
            }

            range.fmt(f)?;
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ranges() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(3, 0, 0)),
            VersionRange::between(&Version::new(4, 0, 0), &Version::new(5, 0, 0)),
        ]);

        assert_eq!(set.to_string(), "1.0.0..2.0.0 | 4.0.0..5.0.0");
    }

    #[test]
    fn connected_ranges_in_order() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(2, 0, 0), &Version::new(3, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
        ]);

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn connected_ranges_out_of_order() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
            VersionRange::between(&Version::new(2, 0, 0), &Version::new(3, 0, 0)),
        ]);

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn disconnected_ranges_in_order() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
            VersionRange::between(&Version::new(5, 0, 0), &Version::new(6, 0, 0)),
        ]);

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn disconnected_ranges_out_of_order() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(5, 0, 0), &Version::new(6, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
        ]);

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn overlap_with_multiple() {
        let set = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
            VersionRange::between(&Version::new(5, 0, 0), &Version::new(6, 0, 0)),
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(5, 0, 0)),
        ]);

        assert_eq!(set.to_string(), "1.0.0..6.0.0");
    }

    #[test]
    fn difference_between_same_sets_is_empty() {
        let first = VersionSet::from_range(VersionRange::between(
            &Version::new(1, 0, 0),
            &Version::new(2, 0, 0),
        ));

        let second = VersionSet::from_range(VersionRange::between(
            &Version::new(1, 0, 0),
            &Version::new(2, 0, 0),
        ));

        assert!(first.difference(&second).is_empty());
    }

    #[test]
    fn difference_between_connected_sets() {
        let first = VersionSet::from_range(VersionRange::between(
            &Version::new(1, 0, 0),
            &Version::new(5, 0, 0),
        ));

        let second = VersionSet::from_range(VersionRange::between(
            &Version::new(3, 0, 0),
            &Version::new(8, 0, 0),
        ));

        assert_eq!(first.difference(&second).to_string(), "1.0.0..3.0.0");

        assert_eq!(second.difference(&first).to_string(), "5.0.0..8.0.0");
    }

    #[test]
    fn difference_between_disconnected_sets() {
        let first = VersionSet::from_range(VersionRange::between(
            &Version::new(1, 0, 0),
            &Version::new(4, 0, 0),
        ));

        let second = VersionSet::from_range(VersionRange::between(
            &Version::new(6, 0, 0),
            &Version::new(10, 0, 0),
        ));

        assert_eq!(first.difference(&second).to_string(), "1.0.0..4.0.0");

        assert_eq!(second.difference(&first).to_string(), "6.0.0..10.0.0");
    }

    #[test]
    fn difference_between_contained_sets() {
        let first = VersionSet::from_range(VersionRange::between(
            &Version::new(1, 0, 0),
            &Version::new(8, 0, 0),
        ));

        let second = VersionSet::from_range(VersionRange::between(
            &Version::new(3, 0, 0),
            &Version::new(5, 0, 0),
        ));

        assert_eq!(
            first.difference(&second).to_string(),
            "1.0.0..3.0.0 | 5.0.0..8.0.0"
        );

        assert!(second.difference(&first).is_empty());
    }

    #[test]
    fn difference_between_multiple_contained_sets() {
        let first = VersionSet::from_ranges([
            VersionRange::between(&Version::new(1, 0, 0), &Version::new(2, 0, 0)),
            VersionRange::between(&Version::new(3, 0, 0), &Version::new(4, 0, 0)),
            VersionRange::between(&Version::new(5, 0, 0), &Version::new(6, 0, 0)),
        ]);

        let second = VersionSet::from_range(VersionRange::between(
            &Version::new(0, 0, 0),
            &Version::new(7, 0, 0),
        ));

        assert!(first.difference(&second).is_empty());

        assert_eq!(
            second.difference(&first).to_string(),
            "0.0.0..1.0.0 | 2.0.0..3.0.0 | 4.0.0..5.0.0 | 6.0.0..7.0.0"
        );
    }
}
