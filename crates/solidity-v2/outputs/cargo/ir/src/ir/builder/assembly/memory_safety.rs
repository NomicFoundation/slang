//! Whether an assembly statement is marked as memory safe, with the `("memory-safe")` flag.

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateAssemblyFlag;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;

use crate::ir::Source;
use crate::ir::builder::CstToIrBuilder;

/// The only flag the language defines for an assembly statement.
const MEMORY_SAFE: &[u8] = b"memory-safe";

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
}
