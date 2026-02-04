use std::rc::Rc;
use std::str::FromStr;

use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use num_traits::Num;

use crate::backend::ir::ir2_flat_contracts::{self as input_ir};
use crate::cst::{TerminalKind, TerminalNode};

pub(crate) fn evaluate_fixed_array_size(
    expression: &input_ir::Expression,
    identifier_resolver: &dyn ConstantIdentifierResolver,
) -> Option<usize> {
    evaluate_compile_time_constant(expression, identifier_resolver)
        .and_then(|value| value.as_usize())
}

fn evaluate_compile_time_constant(
    expression: &input_ir::Expression,
    identifier_resolver: &dyn ConstantIdentifierResolver,
) -> Option<ConstantValue> {
    let mut evaluator = CompileConstantEvaluator {
        identifier_resolver,
        depth: 0,
    };
    evaluator.evaluate_expression(expression)
}

#[derive(Clone, Debug, PartialEq)]
enum ConstantValue {
    Integer(BigInt),
}

impl ConstantValue {
    fn as_usize(&self) -> Option<usize> {
        let Self::Integer(value) = self;
        value.to_usize()
    }
}

pub(crate) trait ConstantIdentifierResolver {
    fn resolve_identifier(&self, identifier: &str) -> Option<input_ir::Expression>;
}

struct CompileConstantEvaluator<'a> {
    identifier_resolver: &'a dyn ConstantIdentifierResolver,
    depth: usize,
}

impl CompileConstantEvaluator<'_> {
    fn evaluate_expression(&mut self, expression: &input_ir::Expression) -> Option<ConstantValue> {
        match expression {
            input_ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.evaluate_bitwise_or_expression(bitwise_or_expression)
            }
            input_ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.evaluate_bitwise_xor_expression(bitwise_xor_expression)
            }
            input_ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.evaluate_bitwise_and_expression(bitwise_and_expression)
            }
            input_ir::Expression::ShiftExpression(shift_expression) => {
                self.evaluate_shift_expression(shift_expression)
            }
            input_ir::Expression::AdditiveExpression(additive_expression) => {
                self.evaluate_additive_expression(additive_expression)
            }
            input_ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.evaluate_multiplicative_expression(multiplicative_expression)
            }
            input_ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.evaluate_exponentiation_expression(exponentiation_expression)
            }
            input_ir::Expression::PrefixExpression(prefix_expression) => {
                self.evaluate_prefix_expression(prefix_expression)
            }
            input_ir::Expression::HexNumberExpression(hex_number_expression) => {
                Self::evaluate_hex_number_expression(hex_number_expression)
            }
            input_ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                Self::evaluate_decimal_number_expression(decimal_number_expression)
            }
            input_ir::Expression::Identifier(terminal_node) => {
                self.evaluate_identifier(terminal_node)
            }
            input_ir::Expression::TupleExpression(tuple_expression) => {
                self.evaluate_tuple_expression(tuple_expression)
            }
            _ => {
                // all other variants of expression cannot be evaluated at
                // compile time, or are not relevant for computing constant
                // integers (eg. string literals)
                None
            }
        }
    }

    fn evaluate_bitwise_or_expression(
        &mut self,
        bitwise_or_expression: &input_ir::BitwiseOrExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_or_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_or_expression.right_operand)?;
        match (lhs, rhs) {
            (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                Some(ConstantValue::Integer(lhs | rhs))
            }
        }
    }

    fn evaluate_bitwise_xor_expression(
        &mut self,
        bitwise_xor_expression: &input_ir::BitwiseXorExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_xor_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_xor_expression.right_operand)?;
        match (lhs, rhs) {
            (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                Some(ConstantValue::Integer(lhs ^ rhs))
            }
        }
    }

    fn evaluate_bitwise_and_expression(
        &mut self,
        bitwise_and_expression: &input_ir::BitwiseAndExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_and_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_and_expression.right_operand)?;
        match (lhs, rhs) {
            (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                Some(ConstantValue::Integer(lhs & rhs))
            }
        }
    }

    fn evaluate_shift_expression(
        &mut self,
        shift_expression: &input_ir::ShiftExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&shift_expression.left_operand)?;
        let rhs = self.evaluate_expression(&shift_expression.right_operand)?;
        match &shift_expression.operator.kind {
            TerminalKind::LessThanLessThan => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs << rhs.to_u32()?))
                }
            },
            TerminalKind::GreaterThanGreaterThan => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs >> rhs.to_u32()?))
                }
            },
            TerminalKind::GreaterThanGreaterThanGreaterThan => None,
            _ => unreachable!("invalid operator in shift expression"),
        }
    }

    fn evaluate_additive_expression(
        &mut self,
        additive_expression: &input_ir::AdditiveExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&additive_expression.left_operand)?;
        let rhs = self.evaluate_expression(&additive_expression.right_operand)?;
        match &additive_expression.operator.kind {
            TerminalKind::Plus => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs + rhs))
                }
            },
            TerminalKind::Minus => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs - rhs))
                }
            },
            _ => unreachable!("invalid operator in additive expression"),
        }
    }

    fn evaluate_multiplicative_expression(
        &mut self,
        multiplicative_expression: &input_ir::MultiplicativeExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&multiplicative_expression.left_operand)?;
        let rhs = self.evaluate_expression(&multiplicative_expression.right_operand)?;
        match &multiplicative_expression.operator.kind {
            TerminalKind::Asterisk => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs * rhs))
                }
            },
            TerminalKind::Slash => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs / rhs))
                }
            },
            TerminalKind::Percent => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs % rhs))
                }
            },
            _ => unreachable!("invalid operator in multiplicative expression"),
        }
    }

    fn evaluate_exponentiation_expression(
        &mut self,
        exponentiation_expression: &input_ir::ExponentiationExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&exponentiation_expression.left_operand)?;
        let rhs = self.evaluate_expression(&exponentiation_expression.right_operand)?;
        match &exponentiation_expression.operator.kind {
            TerminalKind::AsteriskAsterisk => match (lhs, rhs) {
                (ConstantValue::Integer(lhs), ConstantValue::Integer(rhs)) => {
                    Some(ConstantValue::Integer(lhs.pow(rhs.to_u32()?)))
                }
            },
            _ => unreachable!("invalid operator in exponentiation expression"),
        }
    }

    fn evaluate_prefix_expression(
        &mut self,
        prefix_expression: &input_ir::PrefixExpression,
    ) -> Option<ConstantValue> {
        let operand = self.evaluate_expression(&prefix_expression.operand)?;
        match &prefix_expression.operator.kind {
            TerminalKind::Minus => match operand {
                ConstantValue::Integer(value) => Some(ConstantValue::Integer(BigInt::ZERO - value)),
            },
            TerminalKind::Plus => {
                // This is only valid in Solidity < 0.5.0, but the parser should
                // not generate this variant for later versions.
                match operand {
                    ConstantValue::Integer(value) => Some(ConstantValue::Integer(value)),
                }
            }
            _ => None,
        }
    }

    fn evaluate_hex_number_expression(
        hex_number_expression: &input_ir::HexNumberExpression,
    ) -> Option<ConstantValue> {
        let hex = hex_number_expression.literal.unparse();
        // skip `0x` prefix and parse the hexadecimal number
        BigInt::from_str_radix(&hex[2..], 16)
            .ok()
            .map(ConstantValue::Integer)
    }

    fn evaluate_decimal_number_expression(
        decimal_number_expression: &input_ir::DecimalNumberExpression,
    ) -> Option<ConstantValue> {
        // TODO: this only handles integers but not rational numbers
        // TODO: handle number units
        let decimal = decimal_number_expression.literal.unparse();
        BigInt::from_str(&decimal).ok().map(ConstantValue::Integer)
    }

    const MAX_RESOLUTION_DEPTH: usize = 10;

    fn evaluate_expression_recursively(
        &mut self,
        expression: &input_ir::Expression,
    ) -> Option<ConstantValue> {
        self.depth += 1;
        if self.depth >= Self::MAX_RESOLUTION_DEPTH {
            // TODO(validation): cyclic dependency in constant resolution or max depth reached
            return None;
        }
        let result = self.evaluate_expression(expression);
        self.depth -= 1;
        result
    }

    fn evaluate_identifier(&mut self, identifier: &Rc<TerminalNode>) -> Option<ConstantValue> {
        let target_expression = self
            .identifier_resolver
            .resolve_identifier(&identifier.unparse())?;
        self.evaluate_expression_recursively(&target_expression)
    }

    fn evaluate_tuple_expression(
        &mut self,
        tuple_expression: &input_ir::TupleExpression,
    ) -> Option<ConstantValue> {
        if tuple_expression.items.len() == 1 {
            let inner_expression = tuple_expression.items[0].expression.as_ref()?;
            self.evaluate_expression(inner_expression)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use num_bigint::ToBigInt;
    use semver::Version;

    use super::*;
    use crate::backend::ir::ir1_structured_ast::builder::build_expression;
    use crate::backend::passes::p1_flatten_contracts::transform_expression;
    use crate::cst::NonterminalKind;
    use crate::parser;
    use crate::utils::versions::{VERSION_0_4_26, VERSION_0_5_0};
    use crate::utils::LanguageFacts;

    #[derive(Default)]
    struct MapResolver {
        context: HashMap<String, input_ir::Expression>,
    }
    impl ConstantIdentifierResolver for MapResolver {
        fn resolve_identifier(&self, identifier: &str) -> Option<input_ir::Expression> {
            self.context.get(identifier).cloned()
        }
    }

    fn parse_string_in_version(input: &str, version: &Version) -> Option<input_ir::Expression> {
        let parser = parser::Parser::create(version.clone()).expect("parser can be created");
        let output = parser.parse_nonterminal(NonterminalKind::Expression, input);
        let ir1 = build_expression(output.tree())?;
        Some(transform_expression(version, &ir1))
    }

    fn eval_string_in_version(input: &str, version: &Version) -> Option<ConstantValue> {
        let ir2 = parse_string_in_version(input, version)?;
        evaluate_compile_time_constant(&ir2, &MapResolver::default())
    }

    fn eval_string(input: &str) -> Option<ConstantValue> {
        eval_string_in_version(input, &LanguageFacts::LATEST_VERSION)
    }

    fn eval_string_with_context(input: &str, context: &[(&str, &str)]) -> Option<ConstantValue> {
        let version = &LanguageFacts::LATEST_VERSION;
        let context: HashMap<String, input_ir::Expression> = context
            .iter()
            .filter_map(|(name, input)| {
                parse_string_in_version(input, version)
                    .map(|expression| ((*name).to_string(), expression))
            })
            .collect();
        let ir2 = parse_string_in_version(input, version)?;

        evaluate_compile_time_constant(&ir2, &MapResolver { context })
    }

    #[test]
    fn test_literals() {
        assert!(eval_string("1")
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("42")
            .is_some_and(|value| value == ConstantValue::Integer(42.to_bigint().unwrap())));
        assert!(eval_string("0x01")
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("0xa0")
            .is_some_and(|value| value == ConstantValue::Integer(160.to_bigint().unwrap())));
    }

    #[test]
    fn test_prefix_expression() {
        assert!(eval_string("-42")
            .is_some_and(|value| value == ConstantValue::Integer(-42.to_bigint().unwrap())));
        assert!(eval_string_in_version("+42", &VERSION_0_4_26)
            .is_some_and(|value| value == ConstantValue::Integer(42.to_bigint().unwrap())));
        assert!(eval_string_in_version("+42", &VERSION_0_5_0).is_none());
    }

    #[test]
    fn test_binary_expression() {
        assert!(eval_string("1 + 2")
            .is_some_and(|value| value == ConstantValue::Integer(3.to_bigint().unwrap())));
        assert!(eval_string("2 - 2")
            .is_some_and(|value| value == ConstantValue::Integer(0.to_bigint().unwrap())));
        assert!(eval_string("1 - 2")
            .is_some_and(|value| value == ConstantValue::Integer(-1.to_bigint().unwrap())));
        assert!(eval_string("1 * 2")
            .is_some_and(|value| value == ConstantValue::Integer(2.to_bigint().unwrap())));
        assert!(eval_string("4 / 2")
            .is_some_and(|value| value == ConstantValue::Integer(2.to_bigint().unwrap())));
        assert!(eval_string("5 % 2")
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("2 ** 5")
            .is_some_and(|value| value == ConstantValue::Integer(32.to_bigint().unwrap())));
        assert!(eval_string("32 << 2")
            .is_some_and(|value| value == ConstantValue::Integer(128.to_bigint().unwrap())));
        assert!(eval_string("32 >> 2")
            .is_some_and(|value| value == ConstantValue::Integer(8.to_bigint().unwrap())));
        assert!(eval_string("32 | 16")
            .is_some_and(|value| value == ConstantValue::Integer(48.to_bigint().unwrap())));
        assert!(eval_string("15 ^ 31")
            .is_some_and(|value| value == ConstantValue::Integer(16.to_bigint().unwrap())));
        assert!(eval_string("15 & 31")
            .is_some_and(|value| value == ConstantValue::Integer(15.to_bigint().unwrap())));
    }

    #[test]
    fn test_nesting_expressions() {
        assert!(eval_string("1 + (2 + 3)")
            .is_some_and(|value| value == ConstantValue::Integer(6.to_bigint().unwrap())));
        assert!(eval_string("3 * (2 + 1)")
            .is_some_and(|value| value == ConstantValue::Integer(9.to_bigint().unwrap())));
    }

    #[test]
    fn test_identifier_lookup() {
        assert!(eval_string_with_context("FOO", &[("FOO", "1")])
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(
            eval_string_with_context("FOO + 2*BAR", &[("FOO", "1"), ("BAR", "5")])
                .is_some_and(|value| value == ConstantValue::Integer(11.to_bigint().unwrap()))
        );
        // undefined symbols
        assert!(eval_string_with_context("FOO", &[]).is_none());
        // cyclic references
        assert!(eval_string_with_context("FOO", &[("FOO", "BAR"), ("BAR", "FOO")]).is_none());
    }
}
