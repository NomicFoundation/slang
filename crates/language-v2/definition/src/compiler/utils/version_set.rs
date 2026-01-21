use std::fmt::Display;
use std::mem::swap;
use std::ops::Range;

use semver::Version;

use crate::model::{SpannedLanguage, SpannedVersionSpecifier};

const MAX_VERSION: Version = Version::new(u64::MAX, u64::MAX, u64::MAX);

#[derive(Clone, Debug, Default)]
pub struct VersionSet {
    ranges: Vec<Range<Version>>,
}

impl VersionSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn add_specifier(
        &mut self,
        specifier: &SpannedVersionSpecifier,
        language: &SpannedLanguage,
    ) {
        match specifier {
            SpannedVersionSpecifier::Always => {
                self.add_version_range(&language.versions[0], &MAX_VERSION);
            }
            SpannedVersionSpecifier::Never => {
                // Do nothing.
            }
            SpannedVersionSpecifier::From { from } => {
                self.add_version_range(from, &MAX_VERSION);
            }
            SpannedVersionSpecifier::Till { till } => {
                self.add_version_range(&language.versions[0], till);
            }
            SpannedVersionSpecifier::Range { from, till } => {
                self.add_version_range(from, till);
            }
        }
    }

    pub fn add_all_versions(&mut self, language: &SpannedLanguage) {
        self.add_version_range(&language.versions[0], &MAX_VERSION);
    }

    pub fn add_version_range(&mut self, from: &Version, till: &Version) {
        let mut from = from.to_owned();
        let mut till = till.to_owned();
        assert!(from < till, "Invalid range: '{from}..{till}'");

        self.ranges.retain_mut(|range| {
            if till < range.start {
                // 'range' starts later. swap with 'from..till' and keep going:
                swap(&mut from, &mut range.start);
                swap(&mut till, &mut range.end);
                return true;
            }

            if range.end < from {
                // 'range' ends earlier. keep it:
                return true;
            }

            // there is some overlap:
            //  make sure 'range' is included in 'from..till', then remove it:
            if range.start < from {
                from = range.start.clone();
            }
            if till < range.end {
                till = range.end.clone();
            }
            false
        });

        self.ranges.push(from..till);
    }

    pub fn add_version_set(&mut self, other: &Self) {
        for range in &other.ranges {
            self.add_version_range(&range.start, &range.end);
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut ranges = vec![];

        let mut first_iter = self.ranges.iter().cloned().peekable();
        let mut second_iter = other.ranges.iter().peekable();

        loop {
            let (Some(first), Some(second)) = (first_iter.peek_mut(), second_iter.peek()) else {
                break;
            };

            if first.end <= second.start {
                // first fully exists before second: take it, and advance first:
                ranges.push(first_iter.next().unwrap());
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
                ranges.push(first.start.clone()..second.start.clone());
            }

            if first.end <= second.end {
                // first ends before second: advance first, as it's been fully processed:
                first_iter.next();
                continue;
            }

            // keep part of first that exists after second:
            first.start = second.end.clone();

            // advance second, as it's been fully processed:
            second_iter.next();
        }

        // Take anything remaining in first:
        ranges.extend(first_iter);

        Self { ranges }
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

            if range.start == MAX_VERSION {
                "MAX".fmt(f)?;
            } else {
                range.start.fmt(f)?;
            }

            "..".fmt(f)?;

            if range.end == MAX_VERSION {
                "MAX".fmt(f)?;
            } else {
                range.end.fmt(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_ranges_in_order() {
        let mut set = VersionSet::new();
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        set.add_version_range(&Version::new(2, 0, 0), &Version::new(3, 0, 0));
        set.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn connected_ranges_out_of_order() {
        let mut set = VersionSet::new();
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        set.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));
        set.add_version_range(&Version::new(2, 0, 0), &Version::new(3, 0, 0));

        assert_eq!(set.to_string(), "1.0.0..4.0.0");
    }

    #[test]
    fn disconnected_ranges_in_order() {
        let mut set = VersionSet::new();
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        set.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));
        set.add_version_range(&Version::new(5, 0, 0), &Version::new(6, 0, 0));

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn disconnected_ranges_out_of_order() {
        let mut set = VersionSet::new();
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        set.add_version_range(&Version::new(5, 0, 0), &Version::new(6, 0, 0));
        set.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));

        assert_eq!(
            set.to_string(),
            "1.0.0..2.0.0 | 3.0.0..4.0.0 | 5.0.0..6.0.0"
        );
    }

    #[test]
    fn overlap_with_multiple() {
        let mut set = VersionSet::new();
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        set.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));
        set.add_version_range(&Version::new(5, 0, 0), &Version::new(6, 0, 0));
        set.add_version_range(&Version::new(1, 0, 0), &Version::new(5, 0, 0));

        assert_eq!(set.to_string(), "1.0.0..6.0.0");
    }

    #[test]
    fn difference_between_same_sets_is_empty() {
        let mut first = VersionSet::new();
        first.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));

        let mut second = VersionSet::new();
        second.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));

        assert!(first.difference(&second).is_empty());
    }

    #[test]
    fn difference_between_connected_sets() {
        let mut first = VersionSet::new();
        first.add_version_range(&Version::new(1, 0, 0), &Version::new(5, 0, 0));

        let mut second = VersionSet::new();
        second.add_version_range(&Version::new(3, 0, 0), &Version::new(8, 0, 0));

        assert_eq!(first.difference(&second).to_string(), "1.0.0..3.0.0");

        assert_eq!(second.difference(&first).to_string(), "5.0.0..8.0.0");
    }

    #[test]
    fn difference_between_disconnected_sets() {
        let mut first = VersionSet::new();
        first.add_version_range(&Version::new(1, 0, 0), &Version::new(4, 0, 0));

        let mut second = VersionSet::new();
        second.add_version_range(&Version::new(6, 0, 0), &Version::new(10, 0, 0));

        assert_eq!(first.difference(&second).to_string(), "1.0.0..4.0.0");

        assert_eq!(second.difference(&first).to_string(), "6.0.0..10.0.0");
    }

    #[test]
    fn difference_between_contained_sets() {
        let mut first = VersionSet::new();
        first.add_version_range(&Version::new(1, 0, 0), &Version::new(8, 0, 0));

        let mut second = VersionSet::new();
        second.add_version_range(&Version::new(3, 0, 0), &Version::new(5, 0, 0));

        assert_eq!(
            first.difference(&second).to_string(),
            "1.0.0..3.0.0 | 5.0.0..8.0.0"
        );

        assert!(second.difference(&first).is_empty());
    }

    #[test]
    fn difference_between_multiple_contained_sets() {
        let mut first = VersionSet::new();
        first.add_version_range(&Version::new(1, 0, 0), &Version::new(2, 0, 0));
        first.add_version_range(&Version::new(3, 0, 0), &Version::new(4, 0, 0));
        first.add_version_range(&Version::new(5, 0, 0), &Version::new(6, 0, 0));

        let mut second = VersionSet::new();
        second.add_version_range(&Version::new(0, 0, 0), &Version::new(7, 0, 0));

        assert!(first.difference(&second).is_empty());

        assert_eq!(
            second.difference(&first).to_string(),
            "0.0.0..1.0.0 | 2.0.0..3.0.0 | 4.0.0..5.0.0 | 6.0.0..7.0.0"
        );
    }
}
