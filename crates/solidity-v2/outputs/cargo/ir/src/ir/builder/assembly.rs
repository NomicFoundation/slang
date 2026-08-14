use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateAssemblyFlag;
use slang_solidity_v2_common::diagnostics::kinds::syntax::InvalidAssemblyDialect;
use slang_solidity_v2_common::utils::strings::{
    decode_escape_sequences, strip_string_literal_quotes,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use super::CstToIrBuilder;
use crate::ir::{Source, nodes as output};

/// The only dialect an assembly statement is allowed to name.
const EVMASM: &[u8] = b"evmasm";

/// The only flag the language defines for an assembly statement.
const MEMORY_SAFE: &[u8] = b"memory-safe";

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        let id = self.next_id(output::NodeKind::AssemblyStatement);
        let range = source.calculate_text_range().unwrap_or_default();

        if let Some(dialect) = &source.label
            && !self.string_literal_decodes_to(dialect.range.clone(), EVMASM)
        {
            self.report(dialect, InvalidAssemblyDialect);
        }

        let is_memory_safe = match &source.flags {
            Some(flags) => self.check_memory_safe_flag(flags),
            None => false,
        };
        let body = self.build_yul_block(&source.body);

        Arc::new(output::AssemblyStatementStruct {
            id,
            range,
            is_memory_safe,
            body,
        })
    }

    /// Scans the flag list for `memory-safe` and returns whether the statement
    /// is marked with it.
    ///
    /// The flag may only be listed once, so every repetition past the first is
    /// reported on the offending flag itself: a flag listed three times yields
    /// two diagnostics, pointing at the second and third occurrences.
    ///
    /// Any other flag is currently unknown to the language and is dropped
    /// without a diagnostic.
    fn check_memory_safe_flag(&mut self, source: &input::YulFlagsDeclaration) -> bool {
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

    /// Compares the value of the string literal at `range` against `expected`,
    /// decoding escape sequences only when the literal actually contains any.
    fn string_literal_decodes_to(&self, range: Range<usize>, expected: &[u8]) -> bool {
        let content = strip_string_literal_quotes(self.source.text(range));

        if content.contains('\\') {
            decode_escape_sequences(content) == expected
        } else {
            content.as_bytes() == expected
        }
    }
}
