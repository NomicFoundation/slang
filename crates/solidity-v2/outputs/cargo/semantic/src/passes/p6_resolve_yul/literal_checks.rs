use slang_solidity_v2_common::diagnostics::kinds::type_system::LiteralTooLarge;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::types::literals::yul_literal_value;

impl Pass<'_> {
    /// Checks Yul literals.
    ///
    /// Reports number literals (decimal or hex) whose value does not fit
    /// into 256 bits.
    pub(super) fn check_yul_literal(&mut self, node: &ir::YulLiteral) {
        match node {
            ir::YulLiteral::DecimalLiteral(_) | ir::YulLiteral::HexLiteral(_) => {
                if yul_literal_value(node).is_none() {
                    self.push_diagnostic(node, LiteralTooLarge);
                }
            }
            ir::YulLiteral::TrueKeyword(_)
            | ir::YulLiteral::FalseKeyword(_)
            | ir::YulLiteral::StringLiteral(_)
            | ir::YulLiteral::HexStringLiteral(_) => {}
        }
    }
}
