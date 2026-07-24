use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::structure::DuplicateYulSwitchCase;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::types::literals::{
    value_of_hex_string_literals, value_of_string_literals, yul_literal_value,
};

impl Pass<'_> {
    /// Reports every `case` in a Yul `switch` whose value duplicates an earlier
    /// one. Cases are compared by the 256-bit word each literal evaluates to,
    /// so `0`, `0x0` and `""` all collide.
    pub(super) fn check_duplicate_switch_cases(&mut self, node: &ir::YulSwitchStatement) {
        let mut seen: Set<U256> = Set::default();
        for value_case in node.value_cases.iter() {
            let Some(value) = yul_literal_value(&value_case.value) else {
                continue;
            };
            if !seen.insert(value) {
                let file_id = self
                    .file_node_mapper
                    .file_id_from_node_id(value_case.id())
                    .to_owned();
                self.diagnostics.push(
                    file_id,
                    value_case.range.clone(),
                    DuplicateYulSwitchCase {
                        value: format_yul_literal(&value_case.value),
                    },
                );
            }
        }
    }
}

/// Formats a Yul literal for the duplicate-case message: the original source
/// text for numbers and booleans, and the decoded content (without quotes) for
/// strings.
fn format_yul_literal(literal: &ir::YulLiteral) -> String {
    match literal {
        ir::YulLiteral::TrueKeyword(_)
        | ir::YulLiteral::FalseKeyword(_)
        | ir::YulLiteral::DecimalLiteral(_)
        | ir::YulLiteral::HexLiteral(_) => literal.unparse().to_owned(),
        ir::YulLiteral::StringLiteral(literal) => {
            String::from_utf8_lossy(&value_of_string_literals(std::slice::from_ref(literal)))
                .into_owned()
        }
        ir::YulLiteral::HexStringLiteral(literal) => {
            String::from_utf8_lossy(&value_of_hex_string_literals(std::slice::from_ref(literal)))
                .into_owned()
        }
    }
}
