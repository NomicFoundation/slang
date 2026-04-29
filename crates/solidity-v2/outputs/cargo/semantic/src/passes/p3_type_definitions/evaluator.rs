use slang_solidity_v2_ir::ir;

use crate::types::ConstantValue;

pub(crate) fn evaluate_compile_time_uint_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Option<usize> {
    evaluate_compile_time_constant(expression, start_scope, identifier_resolver)
        .and_then(|value| value.as_usize())
}

fn evaluate_compile_time_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Option<ConstantValue> {
    let mut evaluator = CompileConstantEvaluator {
        identifier_resolver,
        scope_stack: Vec::new(),
    };
    evaluator.evaluate_expression_in_scope(expression, start_scope)
}

pub(crate) trait ConstantIdentifierResolver<Scope> {
    /// Resolve an identifier to a constant in the given context, returning the
    /// value expression of the constant and the scope in which the value
    /// expression should be resolved. Returns `None` if the identifier cannot
    /// be resolved to a constant.
    fn resolve_identifier_in_scope(
        &self,
        identifier: &str,
        scope: &Scope,
    ) -> Option<(ir::Expression, Scope)>;
}

struct CompileConstantEvaluator<'a, Scope> {
    identifier_resolver: &'a dyn ConstantIdentifierResolver<Scope>,
    scope_stack: Vec<Scope>,
}

impl<Scope> CompileConstantEvaluator<'_, Scope> {
    const MAX_SCOPE_DEPTH: usize = 10;

    fn evaluate_expression_in_scope(
        &mut self,
        expression: &ir::Expression,
        scope: Scope,
    ) -> Option<ConstantValue> {
        if self.scope_stack.len() >= Self::MAX_SCOPE_DEPTH {
            // TODO(validation): cyclic dependency in constant resolution or max depth reached
            return None;
        }
        self.scope_stack.push(scope);
        let result = self.evaluate_expression(expression);
        self.scope_stack
            .pop()
            .expect("scope stack should not be empty");
        result
    }

    fn evaluate_expression(&mut self, expression: &ir::Expression) -> Option<ConstantValue> {
        match expression {
            ir::Expression::BitwiseOrExpression(bitwise_or_expression) => {
                self.evaluate_bitwise_or_expression(bitwise_or_expression)
            }
            ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => {
                self.evaluate_bitwise_xor_expression(bitwise_xor_expression)
            }
            ir::Expression::BitwiseAndExpression(bitwise_and_expression) => {
                self.evaluate_bitwise_and_expression(bitwise_and_expression)
            }
            ir::Expression::ShiftExpression(shift_expression) => {
                self.evaluate_shift_expression(shift_expression)
            }
            ir::Expression::AdditiveExpression(additive_expression) => {
                self.evaluate_additive_expression(additive_expression)
            }
            ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                self.evaluate_multiplicative_expression(multiplicative_expression)
            }
            ir::Expression::ExponentiationExpression(exponentiation_expression) => {
                self.evaluate_exponentiation_expression(exponentiation_expression)
            }
            ir::Expression::PrefixExpression(prefix_expression) => {
                self.evaluate_prefix_expression(prefix_expression)
            }
            ir::Expression::HexNumberExpression(hex_number_expression) => {
                ConstantValue::from_hex_number_expression(hex_number_expression)
            }
            ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                ConstantValue::from_decimal_number_expression(decimal_number_expression)
            }
            ir::Expression::Identifier(identifier) => self.evaluate_identifier(identifier),
            ir::Expression::TupleExpression(tuple_expression) => {
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
        bitwise_or_expression: &ir::BitwiseOrExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_or_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_or_expression.right_operand)?;
        lhs.bit_or(&rhs)
    }

    fn evaluate_bitwise_xor_expression(
        &mut self,
        bitwise_xor_expression: &ir::BitwiseXorExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_xor_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_xor_expression.right_operand)?;
        lhs.bit_xor(&rhs)
    }

    fn evaluate_bitwise_and_expression(
        &mut self,
        bitwise_and_expression: &ir::BitwiseAndExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&bitwise_and_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_and_expression.right_operand)?;
        lhs.bit_and(&rhs)
    }

    fn evaluate_shift_expression(
        &mut self,
        shift_expression: &ir::ShiftExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&shift_expression.left_operand)?;
        let rhs = self.evaluate_expression(&shift_expression.right_operand)?;
        match &shift_expression.expression_shift_expression_operator {
            ir::Expression_ShiftExpression_Operator::LessThanLessThan => lhs.shl(&rhs),
            ir::Expression_ShiftExpression_Operator::GreaterThanGreaterThan => lhs.shr(&rhs),
            ir::Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan => None,
        }
    }

    fn evaluate_additive_expression(
        &mut self,
        additive_expression: &ir::AdditiveExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&additive_expression.left_operand)?;
        let rhs = self.evaluate_expression(&additive_expression.right_operand)?;
        match &additive_expression.expression_additive_expression_operator {
            ir::Expression_AdditiveExpression_Operator::Plus => Some(lhs.add(&rhs)),
            ir::Expression_AdditiveExpression_Operator::Minus => Some(lhs.sub(&rhs)),
        }
    }

    fn evaluate_multiplicative_expression(
        &mut self,
        multiplicative_expression: &ir::MultiplicativeExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&multiplicative_expression.left_operand)?;
        let rhs = self.evaluate_expression(&multiplicative_expression.right_operand)?;
        match &multiplicative_expression.expression_multiplicative_expression_operator {
            ir::Expression_MultiplicativeExpression_Operator::Asterisk => Some(lhs.mul(&rhs)),
            ir::Expression_MultiplicativeExpression_Operator::Slash => lhs.div(&rhs),
            ir::Expression_MultiplicativeExpression_Operator::Percent => lhs.rem(&rhs),
        }
    }

    fn evaluate_exponentiation_expression(
        &mut self,
        exponentiation_expression: &ir::ExponentiationExpression,
    ) -> Option<ConstantValue> {
        let lhs = self.evaluate_expression(&exponentiation_expression.left_operand)?;
        let rhs = self.evaluate_expression(&exponentiation_expression.right_operand)?;
        // v2 ExponentiationExpression has no explicit operator field (only `**` exists)
        lhs.pow(&rhs)
    }

    fn evaluate_prefix_expression(
        &mut self,
        prefix_expression: &ir::PrefixExpression,
    ) -> Option<ConstantValue> {
        let operand = self.evaluate_expression(&prefix_expression.operand)?;
        match &prefix_expression.expression_prefix_expression_operator {
            ir::Expression_PrefixExpression_Operator::Minus => Some(operand.negate()),
            // No unary plus in Solidity >= 0.5.0 (v2 only supports >= 0.8.0)
            _ => None,
        }
    }

    fn evaluate_identifier(&mut self, identifier: &ir::Identifier) -> Option<ConstantValue> {
        let current_scope = self.scope_stack.last().expect("scope stack is empty");
        let (target_expression, target_scope) = self
            .identifier_resolver
            .resolve_identifier_in_scope(identifier.unparse(), current_scope)?;
        self.evaluate_expression_in_scope(&target_expression, target_scope)
    }

    fn evaluate_tuple_expression(
        &mut self,
        tuple_expression: &ir::TupleExpression,
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

    use num_bigint::{BigInt, ToBigInt};
    use num_rational::BigRational;
    use slang_solidity_v2_common::versions::LanguageVersion;
    use slang_solidity_v2_parser::{ParseOutput, Parser};

    use super::*;

    #[derive(Default)]
    struct MapResolver {
        // qualified identifier => (target expression, target scope)
        // qualified identifier is the concatenation of the scope and identifier
        context: HashMap<String, (ir::Expression, String)>,
    }

    impl ConstantIdentifierResolver<String> for MapResolver {
        fn resolve_identifier_in_scope(
            &self,
            identifier: &str,
            scope: &String,
        ) -> Option<(ir::Expression, String)> {
            self.context.get(&format!("{scope}{identifier}")).cloned()
        }
    }

    fn parse_expression(input: &str) -> ir::Expression {
        let source = format!("uint constant x = {input};");
        let version = LanguageVersion::V0_8_30;

        let ParseOutput {
            source_unit,
            errors,
        } = Parser::parse(&source, version);
        assert!(errors.is_empty(), "Parser errors: {errors:?}");

        let mut id_generator = ir::NodeIdGenerator::default();
        let source_unit = ir::build(&source_unit, &source, &mut id_generator);

        let member = source_unit.members.first().expect("no source unit members");
        match member {
            ir::SourceUnitMember::ConstantDefinition(definition) => definition
                .value
                .clone()
                .expect("constant definition has no value expression"),
            _ => panic!("expected a ConstantDefinition"),
        }
    }

    fn eval_string(input: &str) -> Option<ConstantValue> {
        let expression = parse_expression(input);
        evaluate_compile_time_constant(&expression, String::new(), &MapResolver::default())
    }

    // `context` is given as a list of constants defined as:
    // - the qualified identifier
    // - the target expression (to be parsed)
    // - the target scope
    // Resolution always starts at the empty string scope in these tests.
    fn eval_string_with_context(
        input: &str,
        context: &[(&str, &str, &str)],
    ) -> Option<ConstantValue> {
        let context: HashMap<String, (ir::Expression, String)> = context
            .iter()
            .map(|(name, input, target_scope)| {
                (
                    (*name).to_string(),
                    (parse_expression(input), (*target_scope).to_string()),
                )
            })
            .collect();
        let expression = parse_expression(input);

        evaluate_compile_time_constant(&expression, String::new(), &MapResolver { context })
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
    fn test_literals_with_digit_separators() {
        assert!(eval_string("1_000")
            .is_some_and(|value| value == ConstantValue::Integer(1_000.to_bigint().unwrap())));
        assert!(eval_string("1_000_000")
            .is_some_and(|value| value == ConstantValue::Integer(1_000_000.to_bigint().unwrap())));
        assert!(eval_string("0xdead_beef").is_some_and(
            |value| value == ConstantValue::Integer(0xdead_beef_u64.to_bigint().unwrap())
        ));
        assert!(eval_string("0x1234_5678").is_some_and(
            |value| value == ConstantValue::Integer(0x1234_5678_u64.to_bigint().unwrap())
        ));
    }

    #[test]
    fn test_literals_with_number_units() {
        assert!(eval_string("1 wei")
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("1 gwei").is_some_and(
            |value| value == ConstantValue::Integer(1_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("1 ether").is_some_and(|value| value
            == ConstantValue::Integer(1_000_000_000_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("2 ether").is_some_and(|value| value
            == ConstantValue::Integer(2_000_000_000_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("1 seconds")
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("1 minutes")
            .is_some_and(|value| value == ConstantValue::Integer(60.to_bigint().unwrap())));
        assert!(eval_string("1 hours")
            .is_some_and(|value| value == ConstantValue::Integer(3_600.to_bigint().unwrap())));
        assert!(eval_string("1 days")
            .is_some_and(|value| value == ConstantValue::Integer(86_400.to_bigint().unwrap())));
        assert!(eval_string("1 weeks")
            .is_some_and(|value| value == ConstantValue::Integer(604_800.to_bigint().unwrap())));
    }

    #[test]
    fn test_literals_with_scientific_notation() {
        assert!(eval_string("1e3")
            .is_some_and(|value| value == ConstantValue::Integer(1_000.to_bigint().unwrap())));
        assert!(eval_string("2e10").is_some_and(
            |value| value == ConstantValue::Integer(20_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("1e18").is_some_and(|value| value
            == ConstantValue::Integer(1_000_000_000_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("1.5e3")
            .is_some_and(|value| value == ConstantValue::Integer(1_500.to_bigint().unwrap())));
    }

    #[test]
    fn test_reducible_rational_literals() {
        assert!(eval_string("1.5 ether").is_some_and(|value| value
            == ConstantValue::Integer(1_500_000_000_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("0.5 ether").is_some_and(|value| value
            == ConstantValue::Integer(500_000_000_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("0.5 gwei").is_some_and(
            |value| value == ConstantValue::Integer(500_000_000u64.to_bigint().unwrap())
        ));
    }

    #[test]
    fn test_non_reducible_rational_literals() {
        assert!(eval_string("0.5").is_some_and(|value| value
            == ConstantValue::Rational(BigRational::new(BigInt::from(1), BigInt::from(2)))));
        assert!(eval_string("3.14").is_some_and(|value| value
            == ConstantValue::Rational(BigRational::new(BigInt::from(157), BigInt::from(50)))));
        assert!(eval_string("1e-1").is_some_and(|value| value
            == ConstantValue::Rational(BigRational::new(BigInt::from(1), BigInt::from(10)))));
    }

    #[test]
    fn test_rational_arithmetic_folds() {
        // Reducible rational result normalises back to an Integer.
        assert!(eval_string("1.5 * 2")
            .is_some_and(|value| value == ConstantValue::Integer(BigInt::from(3))));
        // Integer division that does not divide evenly yields a Rational.
        assert!(eval_string("5 / 2").is_some_and(|value| value
            == ConstantValue::Rational(BigRational::new(BigInt::from(5), BigInt::from(2)))));
        // Negation of a non-reducing rational stays Rational.
        assert!(eval_string("-0.5").is_some_and(|value| value
            == ConstantValue::Rational(BigRational::new(BigInt::from(-1), BigInt::from(2)))));
    }

    #[test]
    fn test_prefix_expression() {
        assert!(eval_string("-42")
            .is_some_and(|value| value == ConstantValue::Integer((-42).to_bigint().unwrap())));
    }

    #[test]
    fn test_binary_expression() {
        assert!(eval_string("1 + 2")
            .is_some_and(|value| value == ConstantValue::Integer(3.to_bigint().unwrap())));
        assert!(eval_string("2 - 2")
            .is_some_and(|value| value == ConstantValue::Integer(0.to_bigint().unwrap())));
        assert!(eval_string("1 - 2")
            .is_some_and(|value| value == ConstantValue::Integer((-1).to_bigint().unwrap())));
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
        assert!(eval_string_with_context("FOO", &[("FOO", "1", "")])
            .is_some_and(|value| value == ConstantValue::Integer(1.to_bigint().unwrap())));
        assert!(
            eval_string_with_context("FOO + 2*BAR", &[("FOO", "1", ""), ("BAR", "5", "")])
                .is_some_and(|value| value == ConstantValue::Integer(11.to_bigint().unwrap()))
        );
        // undefined symbols
        assert!(eval_string_with_context("FOO", &[]).is_none());
        // cyclic references
        assert!(
            eval_string_with_context("FOO", &[("FOO", "BAR", ""), ("BAR", "FOO", "")]).is_none()
        );
        // switching contexts: the value of FOO should resolve in the CTX.
        // context, where BAR is defined
        assert!(eval_string_with_context(
            "FOO",
            &[("FOO", "BAR", "CTX."), ("CTX.BAR", "42", "CTX.")]
        )
        .is_some_and(|value| value == ConstantValue::Integer(42.to_bigint().unwrap())));
    }
}
