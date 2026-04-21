use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;

use super::super::{FunctionCallExpressionStruct, StringExpression};

impl StringExpression {
    /// Returns the concatenated decoded string value as bytes.
    ///
    /// Handles all three variants:
    /// - `StringLiterals` — strips quotes, returns UTF-8 bytes
    /// - `HexStringLiterals` — strips `hex"..."`, decodes hex pairs to bytes
    /// - `UnicodeStringLiterals` — strips `unicode"..."`, returns UTF-8 bytes
    ///
    /// TODO(v2): Escape sequences (`\n`, `\xNN`, `\uNNNN`, etc.) are not yet
    /// decoded — the raw escape text is returned as-is for regular and unicode
    /// strings.
    pub fn value(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            StringExpression::StringLiterals(terminals) => {
                for terminal in terminals {
                    let content = Self::strip_prefix_and_quotes(&terminal.text, "");
                    result.extend_from_slice(content.as_bytes());
                }
            }
            StringExpression::HexStringLiterals(terminals) => {
                for terminal in terminals {
                    let content = Self::strip_prefix_and_quotes(&terminal.text, "hex");
                    result.extend(
                        (0..content.len())
                            .step_by(2)
                            .map(|i| u8::from_str_radix(&content[i..i + 2], 16).unwrap()),
                    );
                }
            }
            StringExpression::UnicodeStringLiterals(terminals) => {
                for terminal in terminals {
                    let content = Self::strip_prefix_and_quotes(&terminal.text, "unicode");
                    result.extend_from_slice(content.as_bytes());
                }
            }
        }
        result
    }

    fn strip_prefix_and_quotes<'a>(text: &'a str, prefix: &str) -> &'a str {
        text.strip_prefix(prefix)
            .and_then(|stripped| {
                stripped
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .or_else(|| {
                        stripped
                            .strip_prefix('\'')
                            .and_then(|s| s.strip_suffix('\''))
                    })
            })
            .unwrap_or(text)
    }
}

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            ir::Expression::ElementaryType(_) | ir::Expression::PayableKeyword => true,
            ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder().node_typing(terminal.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder().node_typing(mae.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            _ => false,
        }
    }
}
