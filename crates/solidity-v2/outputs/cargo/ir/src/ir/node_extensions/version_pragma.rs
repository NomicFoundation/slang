use std::cmp::Ordering;

use semver::Version as SemverVersion;

use crate::ir;

impl ir::VersionPragmaStruct {
    /// Whether this pragma admits `version`, which may be a
    /// [`slang_solidity_v2_common::versions::LanguageVersion`] or any other
    /// version.
    ///
    /// `||` separates alternatives, so it is enough for one of the pragma's sets
    /// to admit the version; within a set, every comparator has to hold at once.
    /// A pragma that named nothing admits nothing.
    pub fn matches_version(&self, version: impl Into<SemverVersion>) -> bool {
        let version = version.into();

        self.sets.iter().any(|set| {
            set.iter()
                .all(|comparator| comparator.matches_version(&version))
        })
    }
}

impl ir::VersionPragmaComparatorStruct {
    /// Whether `version` satisfies this comparator's operator applied to its
    /// specifier.
    ///
    /// Matching walks the components the specifier wrote from the left, skipping
    /// the ones written as a wildcard, and stops at the first component the two
    /// disagree on; the operator then judges that disagreement. So a wildcard
    /// frees its own component without freeing the ones after it: `0.x.36`
    /// admits `0.8.36` but not `0.8.35`.
    pub fn matches_version(&self, version: &SemverVersion) -> bool {
        if !self.specifier.is_valid() {
            return true;
        }

        // `~` and `^` are a lower bound paired with an upper bound that ignores
        // the trailing components, so how far they reach depends on how much of
        // the specifier was written.
        let levels = self.specifier.len();
        let upper_bound_levels = match self.operator {
            // `~0.8.1` reaches to the end of `0.8`, and `~1` to the end of `1`.
            ir::VersionPragmaOperator::Tilde => levels.min(2),
            // `^0.8.1` also reaches to the end of `0.8`, since a zero major
            // makes the minor the significant component — but `^0` on its own
            // wrote no minor to hold on to, so it reaches to the end of `0`.
            ir::VersionPragmaOperator::Caret => {
                if matches!(
                    self.specifier.first(),
                    Some(ir::VersionPragmaComponent::Number(0))
                ) && levels != 1
                {
                    2
                } else {
                    1
                }
            }
            operator => {
                return satisfies(compare(version, &self.specifier, levels), operator);
            }
        };

        satisfies(
            compare(version, &self.specifier, levels),
            ir::VersionPragmaOperator::GreaterThanEqual,
        ) && satisfies(
            compare(version, &self.specifier, upper_bound_levels),
            ir::VersionPragmaOperator::LessThanEqual,
        )
    }
}

/// Extensions for reading a specifier, which is a plain list of components.
pub trait VersionPragmaSpecifierExtensions {
    /// Whether every component names a number or a wildcard, ie. whether this
    /// specifier spells out a version at all. An empty specifier does not.
    fn is_valid(&self) -> bool;
}

impl VersionPragmaSpecifierExtensions for ir::VersionPragmaSpecifier {
    fn is_valid(&self) -> bool {
        !self.is_empty()
            && !self
                .iter()
                .any(|component| matches!(component, ir::VersionPragmaComponent::Unrecognized))
    }
}

/// Extensions for reading a single component.
pub trait VersionPragmaComponentExtensions {
    /// The value this component constrains its position to, or `None` if it
    /// constrains nothing — either because it is a wildcard, or because it does
    /// not name a version at all.
    fn number(&self) -> Option<usize>;
}

impl VersionPragmaComponentExtensions for ir::VersionPragmaComponent {
    fn number(&self) -> Option<usize> {
        match self {
            ir::VersionPragmaComponent::Number(number) => Some(*number),
            ir::VersionPragmaComponent::Wildcard | ir::VersionPragmaComponent::Unrecognized => None,
        }
    }
}

/// Compares `version` against the first `levels` components of `specifier`,
/// skipping the ones written as a wildcard.
fn compare(
    version: &SemverVersion,
    specifier: &ir::VersionPragmaSpecifier,
    levels: usize,
) -> Ordering {
    let theirs = [version.major, version.minor, version.patch];

    let mut ordering = Ordering::Equal;
    let mut compared_any = false;

    for (index, ours) in specifier.iter().take(levels).enumerate() {
        let Some(ours) = ours.number() else {
            continue;
        };

        let Some(theirs) = theirs.get(index) else {
            // The specifier named a component the version does not have, so the
            // version is the shorter of the two and thus the lesser.
            ordering = Ordering::Less;
            break;
        };

        compared_any = true;
        // A version's components are wider than the ones a specifier holds.
        ordering = usize::try_from(*theirs).unwrap_or(usize::MAX).cmp(&ours);

        if ordering != Ordering::Equal {
            break;
        }
    }

    // A pre-release sorts below the release it leads up to, so a specifier that
    // would otherwise have matched it exactly ends up above it.
    if ordering == Ordering::Equal && compared_any && !version.pre.is_empty() {
        ordering = Ordering::Less;
    }

    ordering
}

/// Whether `ordering` — how a version compared against the specifier — satisfies
/// `operator`.
fn satisfies(ordering: Ordering, operator: ir::VersionPragmaOperator) -> bool {
    match operator {
        ir::VersionPragmaOperator::Equal => ordering.is_eq(),
        ir::VersionPragmaOperator::LessThan => ordering.is_lt(),
        ir::VersionPragmaOperator::LessThanEqual => ordering.is_le(),
        ir::VersionPragmaOperator::GreaterThan => ordering.is_gt(),
        ir::VersionPragmaOperator::GreaterThanEqual => ordering.is_ge(),
        ir::VersionPragmaOperator::Caret | ir::VersionPragmaOperator::Tilde => {
            unreachable!("'{operator:?}' is resolved into a pair of bounds")
        }
    }
}
