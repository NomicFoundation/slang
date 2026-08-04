use anyhow::{Context, Result};
use serde::Deserialize;

use super::single_target_all_versions::{RawSingleTargetAllVersions, SingleTargetAllVersions};
use super::single_version_all_targets::{RawSingleVersionAllTargets, SingleVersionAllTargets};

/// Configuration controlling how a snapshot test iterates over the
/// `LanguageVersion`/`EvmTarget` matrix. Exactly one axis varies per test —
/// the other is pinned by the config.
#[derive(Clone, Debug)]
pub enum TestMatrix {
    SingleVersionAllTargets(SingleVersionAllTargets),
    SingleTargetAllVersions(SingleTargetAllVersions),
}

/// The matrix as written in a config file. Inheritance only kicks in between
/// configs sharing the same `type` tag, since it decides which axis each field
/// describes — a nested config picking a different one replaces its parent
/// outright, and has to provide every field itself.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub(super) enum RawTestMatrix {
    SingleVersionAllTargets(RawSingleVersionAllTargets),
    SingleTargetAllVersions(RawSingleTargetAllVersions),
}

impl RawTestMatrix {
    /// Absorbs fields from `parent`, a matrix declared further up the traversal,
    /// for every field that `self` doesn't already provide.
    pub(super) fn absorb(&mut self, parent: Self) {
        match (self, parent) {
            (
                Self::SingleVersionAllTargets(matrix),
                Self::SingleVersionAllTargets(parent_matrix),
            ) => {
                matrix.absorb(parent_matrix);
            }
            (
                Self::SingleTargetAllVersions(matrix),
                Self::SingleTargetAllVersions(parent_matrix),
            ) => {
                matrix.absorb(parent_matrix);
            }
            // Different axes: nothing to inherit, so the closer config wins as
            // a whole, and has to provide every field itself.
            _ => {}
        }
    }
}

impl TryFrom<RawTestMatrix> for TestMatrix {
    type Error = anyhow::Error;

    fn try_from(raw: RawTestMatrix) -> Result<Self> {
        Ok(match raw {
            RawTestMatrix::SingleVersionAllTargets(matrix) => Self::SingleVersionAllTargets(
                matrix
                    .try_into()
                    .context("Invalid `SingleVersionAllTargets` matrix.")?,
            ),
            RawTestMatrix::SingleTargetAllVersions(matrix) => Self::SingleTargetAllVersions(
                matrix
                    .try_into()
                    .context("Invalid `SingleTargetAllVersions` matrix.")?,
            ),
        })
    }
}
