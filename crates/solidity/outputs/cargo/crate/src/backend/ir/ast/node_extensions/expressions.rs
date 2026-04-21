use super::super::{FunctionCallExpressionStruct, StringExpression};
use crate::backend::binder::Typing;
use crate::backend::ir::ir2_flat_contracts as input_ir;

impl StringExpression {
    /// Returns the concatenated decoded string value as bytes.
    ///
    /// Handles all three variants:
    /// - `Strings` — strips quotes, returns UTF-8 bytes
    /// - `HexStrings` — strips `hex"..."`, decodes hex pairs to bytes
    /// - `UnicodeStrings` — strips `unicode"..."`, returns UTF-8 bytes
    ///
    /// TODO(v2): Escape sequences (`\n`, `\xNN`, `\uNNNN`, etc.) are not yet
    /// decoded — the raw escape text is returned as-is for regular and unicode
    /// strings.
    pub fn value(&self) -> Vec<u8> {
        let (terminals, prefix) = match self {
            StringExpression::Strings(terminals) => (terminals, ""),
            StringExpression::HexStrings(terminals) => (terminals, "hex"),
            StringExpression::UnicodeStrings(terminals) => (terminals, "unicode"),
        };
        let total_len: usize = terminals
            .iter()
            .map(|terminal| terminal.text.len().saturating_sub(prefix.len() + 2))
            .sum();
        let mut result = Vec::with_capacity(total_len);
        for terminal in terminals {
            let content = Self::strip_quotes(&terminal.text, prefix);
            if prefix == "hex" {
                result.extend(
                    (0..content.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&content[i..i + 2], 16).unwrap()),
                );
            } else {
                result.extend_from_slice(content.as_bytes());
            }
        }
        result
    }

    pub fn strip_quotes<'a>(text: &'a str, prefix: &str) -> &'a str {
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
            input_ir::Expression::ElementaryType(_) | input_ir::Expression::PayableKeyword => true,
            input_ir::Expression::Identifier(terminal) => matches!(
                self.semantic.binder.node_typing(terminal.id()),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            input_ir::Expression::MemberAccessExpression(mae) => matches!(
                self.semantic.binder.node_typing(mae.node_id),
                Typing::MetaType(_) | Typing::UserMetaType(_)
            ),
            _ => false,
        }
    }
}
