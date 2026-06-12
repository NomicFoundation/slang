use num_bigint::BigInt;

use super::super::{
    DecimalNumberExpressionStruct, Expression, FunctionCallExpressionStruct,
    HexNumberExpressionStruct, StringExpression, Type,
};
use crate::backend::binder::Typing;
use crate::backend::ir::ir2_flat_contracts as input_ir;
use crate::backend::types::ConstantValue;

impl Expression {
    /// Returns the slang type of the value produced by this expression.
    ///
    /// Dispatches to each variant's `get_type()`. For `Identifier`, follows
    /// the binder via `resolved_type()` because slang types value-bearing
    /// references at the definition node, not at the use-site identifier.
    /// Returns `None` for keyword expressions (`this`, `true`, `false`, etc.)
    /// and for type-position variants (`ElementaryType`, `StringExpression`).
    pub fn get_type(&self) -> Option<Type> {
        match self {
            Expression::AssignmentExpression(expression) => expression.get_type(),
            Expression::ConditionalExpression(expression) => expression.get_type(),
            Expression::OrExpression(expression) => expression.get_type(),
            Expression::AndExpression(expression) => expression.get_type(),
            Expression::EqualityExpression(expression) => expression.get_type(),
            Expression::InequalityExpression(expression) => expression.get_type(),
            Expression::BitwiseOrExpression(expression) => expression.get_type(),
            Expression::BitwiseXorExpression(expression) => expression.get_type(),
            Expression::BitwiseAndExpression(expression) => expression.get_type(),
            Expression::ShiftExpression(expression) => expression.get_type(),
            Expression::AdditiveExpression(expression) => expression.get_type(),
            Expression::MultiplicativeExpression(expression) => expression.get_type(),
            Expression::ExponentiationExpression(expression) => expression.get_type(),
            Expression::PostfixExpression(expression) => expression.get_type(),
            Expression::PrefixExpression(expression) => expression.get_type(),
            Expression::FunctionCallExpression(expression) => expression.get_type(),
            Expression::CallOptionsExpression(expression) => expression.get_type(),
            Expression::MemberAccessExpression(expression) => expression.get_type(),
            Expression::IndexAccessExpression(expression) => expression.get_type(),
            Expression::NewExpression(expression) => expression.get_type(),
            Expression::TupleExpression(expression) => expression.get_type(),
            Expression::TypeExpression(expression) => expression.get_type(),
            Expression::ArrayExpression(expression) => expression.get_type(),
            Expression::HexNumberExpression(expression) => expression.get_type(),
            Expression::DecimalNumberExpression(expression) => expression.get_type(),
            Expression::Identifier(identifier) => identifier.resolved_type(),
            Expression::StringExpression(_)
            | Expression::ElementaryType(_)
            | Expression::PayableKeyword
            | Expression::ThisKeyword
            | Expression::SuperKeyword
            | Expression::TrueKeyword
            | Expression::FalseKeyword => None,
        }
    }
}

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

impl DecimalNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` for rationals
    /// that do not reduce to an integer after unit multiplication.
    // TODO: support returning rational values (numerator/denominator) once
    // the evaluator exposes them.
    pub fn integer_value(&self) -> Option<BigInt> {
        ConstantValue::from_decimal_number(&self.ir_node)
            .map(|ConstantValue::Integer(integer)| integer)
    }
}

impl HexNumberExpressionStruct {
    /// Returns the integer value of this literal, or `None` if the literal
    /// cannot be evaluated (e.g. a malformed hex digit sequence).
    // TODO: support returning rational values once the evaluator exposes them.
    pub fn integer_value(&self) -> Option<BigInt> {
        ConstantValue::from_hex_number(&self.ir_node).map(|ConstantValue::Integer(integer)| integer)
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
