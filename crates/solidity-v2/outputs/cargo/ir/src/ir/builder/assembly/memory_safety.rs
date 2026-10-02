//! Whether an assembly statement is marked as memory safe: with the `("memory-safe")` flag, or
//! with a single `/// @solidity memory-safe-assembly` line.

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateAssemblyFlag;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;
use slang_solidity_v2_cst::structured_cst::nodes as input;

use crate::ir::Source;
use crate::ir::builder::CstToIrBuilder;

/// The only flag the language defines for an assembly statement.
const MEMORY_SAFE: &[u8] = b"memory-safe";

/// The only line that marks an assembly statement as memory safe.
const MEMORY_SAFE_ASSEMBLY_MARKER: &str = "@solidity memory-safe-assembly";

impl<S: Source> CstToIrBuilder<'_, S> {
    /// Scans the flag list for `memory-safe` and returns whether the statement
    /// is marked with it.
    ///
    /// The flag may only be listed once, so every repetition past the first is
    /// reported on the offending flag itself: a flag listed three times yields
    /// two diagnostics, pointing at the second and third occurrences.
    ///
    /// Any other flag is currently unknown to the language and is dropped
    /// without a diagnostic.
    pub(super) fn check_memory_safe_flag(&mut self, source: &input::YulFlagsDeclaration) -> bool {
        let mut is_memory_safe = false;

        for item in &source.flags.elements {
            if !self.string_literal_decodes_to(item.range.clone(), MEMORY_SAFE) {
                continue;
            }

            if is_memory_safe {
                // Assembly flags were introduced in 0.8.13; before that the
                // error-tolerant parser still yields them, but they are already
                // flagged as invalid syntax for the version, so don't pile
                // another diagnostic on top of them.
                if self.language_version >= LanguageVersion::V0_8_13 {
                    self.report(item, DuplicateAssemblyFlag);
                }
                continue;
            }

            is_memory_safe = true;
        }

        is_memory_safe
    }

    /// Whether the `NatSpec` comment attached to the assembly statement marks it as memory safe.
    pub(super) fn check_memory_safe_natspec(&mut self, natspec: Option<&NatSpec>) -> bool {
        if self.language_version < LanguageVersion::V0_8_13 {
            // Before 0.8.13 the comment had no effect
            return false;
        }
        let Some(natspec) = natspec else {
            return false;
        };

        is_memory_safe_marker(self.source.text(natspec.range.clone()))
    }
}

/// Whether the text of a `NatSpec` comment marks the assembly statement it documents as memory
/// safe: only a single `/// @solidity memory-safe-assembly` line does.
fn is_memory_safe_marker(comment: &str) -> bool {
    // A `///` comment includes the line break after its last line
    let comment = comment
        .strip_suffix("\r\n")
        .or_else(|| comment.strip_suffix(['\n', '\r']))
        .unwrap_or(comment);

    comment.strip_prefix("///").is_some_and(|text| {
        // Strict on purpose: only spaces and tabs may surround the marker. The line
        // break check is redundant with the comparison, but spells out the single line rule
        !text.contains(['\r', '\n'])
            && text.trim_matches([' ', '\t']) == MEMORY_SAFE_ASSEMBLY_MARKER
    })
}

#[cfg(test)]
mod tests {
    use super::is_memory_safe_marker;

    #[test]
    fn marker() {
        // Spaces and tabs around the marker don't matter, nor does the line break after it
        for comment in [
            "/// @solidity memory-safe-assembly",
            "///\t @solidity memory-safe-assembly \t\n",
            "/// @solidity memory-safe-assembly\r\n",
        ] {
            assert!(is_memory_safe_marker(comment), "{comment:?}");
        }
    }

    #[test]
    fn anything_else() {
        for comment in [
            "///\n",
            "/** @solidity memory-safe-assembly */",
            "/// @notice Writes to scratch space\n/// @solidity memory-safe-assembly\n",
            "/// @solidity  memory-safe-assembly",
            // Other whitespace looks the same, but it isn't the expected whitespace
            "/// @solidity\u{a0}memory-safe-assembly",
            "/// @solidity memory-safe-assembly foo",
        ] {
            assert!(!is_memory_safe_marker(comment), "{comment:?}");
        }
    }
}
