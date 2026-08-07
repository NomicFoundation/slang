use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateMemorySafeAssemblyFlag;
use slang_solidity_v2_common::diagnostics::kinds::syntax::InvalidAssemblyDialect;
use slang_solidity_v2_common::utils::{decode_escape_sequences, strip_string_literal_quotes};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use super::CstToIrBuilder;
use crate::ir::{Source, nodes as output};

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_assembly_statement(
        &mut self,
        source: &input::AssemblyStatement,
    ) -> output::AssemblyStatement {
        const EVMASM: &str = "evmasm";

        let id = self.next_id(output::NodeKind::AssemblyStatement);
        let range = source.calculate_text_range().unwrap_or_default();
        if let Some(dialect) = &source.label {
            let decoded_dialect = self.decoded_string_value_from_range(dialect.range.clone());
            if decoded_dialect.as_slice() != EVMASM.as_bytes() {
                self.diagnostics.push(
                    self.file_id.to_owned(),
                    dialect
                        .calculate_text_range()
                        .expect("CST node is expected to have a range."),
                    InvalidAssemblyDialect,
                );
            }
        }
        let flags = source
            .flags
            .as_ref()
            .map(|value| self.build_assembly_flags(value));
        let body = self.build_yul_block(&source.body);

        Arc::new(output::AssemblyStatementStruct {
            id,
            range,
            flags,
            body,
        })
    }

    fn decoded_string_value_from_range(&self, range: Range<usize>) -> Vec<u8> {
        let literal = self.unparse_range(range);
        let content = strip_string_literal_quotes(&literal);
        decode_escape_sequences(content)
    }

    fn build_assembly_flags(
        &mut self,
        source: &input::YulFlagsDeclaration,
    ) -> output::AssemblyFlags {
        const MEMORY_SAFE: &str = "memory-safe";

        let mut flags = Vec::with_capacity(source.flags.elements.len());
        let mut marked_memory_safe = false;
        for item in &source.flags.elements {
            let decoded_flag = self.decoded_string_value_from_range(item.range.clone());
            if decoded_flag.as_slice() == MEMORY_SAFE.as_bytes() {
                if marked_memory_safe {
                    if self.language_version >= LanguageVersion::V0_8_13 {
                        self.diagnostics.push(
                            self.file_id.to_owned(),
                            item.calculate_text_range()
                                .expect("CST node is expected to have a range."),
                            DuplicateMemorySafeAssemblyFlag,
                        );
                    }
                    continue;
                }
                marked_memory_safe = true;
                flags.push(output::AssemblyFlag::MemorySafe);
            }
        }
        flags.into()
    }
}
