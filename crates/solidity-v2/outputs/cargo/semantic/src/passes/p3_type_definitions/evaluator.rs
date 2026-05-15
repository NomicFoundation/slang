use num_bigint::{BigInt, Sign};
use ruint::aliases::U256;
use sha3::{Digest, Keccak256};
use slang_solidity_v2_ir::ir;

use crate::built_ins::BuiltIn;
use crate::types::{literals, Number};

pub(crate) fn evaluate_compile_time_usize_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Option<usize> {
    evaluate_compile_time_number_constant(expression, start_scope, identifier_resolver)
        .and_then(|value| value.as_usize())
}

pub(crate) fn evaluate_compile_time_integer_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Option<BigInt> {
    evaluate_compile_time_number_constant(expression, start_scope, identifier_resolver)
        .and_then(Number::into_integer)
}

fn evaluate_compile_time_number_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Option<Number> {
    let mut evaluator = CompileConstantEvaluator {
        identifier_resolver,
        scope_stack: Vec::new(),
    };
    evaluator.evaluate_expression_in_scope(expression, start_scope)
}

pub(crate) enum ComptimeResolution<Scope> {
    Unresolved,
    Constant {
        value: ir::Expression,
        target_scope: Scope,
    },
    BuiltIn(BuiltIn),
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
    ) -> ComptimeResolution<Scope>;
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
    ) -> Option<Number> {
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

    fn evaluate_expression(&mut self, expression: &ir::Expression) -> Option<Number> {
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
                Number::from_hex_number_expression(hex_number_expression)
            }
            ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                Number::from_decimal_number_expression(decimal_number_expression)
            }
            ir::Expression::Identifier(identifier) => self.evaluate_identifier(identifier),
            ir::Expression::TupleExpression(tuple_expression) => {
                self.evaluate_tuple_expression(tuple_expression)
            }
            ir::Expression::FunctionCallExpression(call) => {
                self.evaluate_function_call_expression(call)
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
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&bitwise_or_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_or_expression.right_operand)?;
        lhs.bit_or(&rhs)
    }

    fn evaluate_bitwise_xor_expression(
        &mut self,
        bitwise_xor_expression: &ir::BitwiseXorExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&bitwise_xor_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_xor_expression.right_operand)?;
        lhs.bit_xor(&rhs)
    }

    fn evaluate_bitwise_and_expression(
        &mut self,
        bitwise_and_expression: &ir::BitwiseAndExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&bitwise_and_expression.left_operand)?;
        let rhs = self.evaluate_expression(&bitwise_and_expression.right_operand)?;
        lhs.bit_and(&rhs)
    }

    fn evaluate_shift_expression(
        &mut self,
        shift_expression: &ir::ShiftExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&shift_expression.left_operand)?;
        let rhs = self.evaluate_expression(&shift_expression.right_operand)?;
        match &shift_expression.operator {
            ir::ShiftExpressionOperator::LessThanLessThan(_) => lhs.shl(&rhs),
            ir::ShiftExpressionOperator::GreaterThanGreaterThan(_) => lhs.shr(&rhs),
            ir::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(_) => None,
        }
    }

    fn evaluate_additive_expression(
        &mut self,
        additive_expression: &ir::AdditiveExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&additive_expression.left_operand)?;
        let rhs = self.evaluate_expression(&additive_expression.right_operand)?;
        match &additive_expression.operator {
            ir::AdditiveExpressionOperator::Plus(_) => Some(lhs.add(&rhs)),
            ir::AdditiveExpressionOperator::Minus(_) => Some(lhs.sub(&rhs)),
        }
    }

    fn evaluate_multiplicative_expression(
        &mut self,
        multiplicative_expression: &ir::MultiplicativeExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&multiplicative_expression.left_operand)?;
        let rhs = self.evaluate_expression(&multiplicative_expression.right_operand)?;
        match &multiplicative_expression.operator {
            ir::MultiplicativeExpressionOperator::Asterisk(_) => Some(lhs.mul(&rhs)),
            ir::MultiplicativeExpressionOperator::Slash(_) => lhs.div(&rhs),
            ir::MultiplicativeExpressionOperator::Percent(_) => lhs.rem(&rhs),
        }
    }

    fn evaluate_exponentiation_expression(
        &mut self,
        exponentiation_expression: &ir::ExponentiationExpression,
    ) -> Option<Number> {
        let lhs = self.evaluate_expression(&exponentiation_expression.left_operand)?;
        let rhs = self.evaluate_expression(&exponentiation_expression.right_operand)?;
        // v2 ExponentiationExpression has no explicit operator field (only `**` exists)
        lhs.pow(&rhs)
    }

    fn evaluate_prefix_expression(
        &mut self,
        prefix_expression: &ir::PrefixExpression,
    ) -> Option<Number> {
        let operand = self.evaluate_expression(&prefix_expression.operand)?;
        match &prefix_expression.operator {
            ir::PrefixExpressionOperator::Minus(_) => Some(operand.negate()),
            ir::PrefixExpressionOperator::Tilde(_) => Some(operand.bit_not()?),
            _ => None,
        }
    }

    fn evaluate_identifier(&mut self, identifier: &ir::Identifier) -> Option<Number> {
        let current_scope = self.scope_stack.last().expect("scope stack is empty");
        let ComptimeResolution::Constant {
            value: target_expression,
            target_scope,
        } = self
            .identifier_resolver
            .resolve_identifier_in_scope(identifier.unparse(), current_scope)
        else {
            return None;
        };
        self.evaluate_expression_in_scope(&target_expression, target_scope)
    }

    fn evaluate_tuple_expression(
        &mut self,
        tuple_expression: &ir::TupleExpression,
    ) -> Option<Number> {
        if tuple_expression.items.len() == 1 {
            let inner_expression = tuple_expression.items[0].expression.as_ref()?;
            self.evaluate_expression(inner_expression)
        } else {
            None
        }
    }

    /// Compile-time evaluation of a built-in function call. Only `erc7201`
    /// is supported as of Solidity 0.8.35. Everything else returns `None`.
    fn evaluate_function_call_expression(
        &mut self,
        call: &ir::FunctionCallExpression,
    ) -> Option<Number> {
        let ir::Expression::Identifier(operand) = &call.operand else {
            return None;
        };
        let scope = self.scope_stack.last().expect("scope stack is empty");
        let resolution = self
            .identifier_resolver
            .resolve_identifier_in_scope(operand.unparse(), scope);
        match resolution {
            ComptimeResolution::BuiltIn(BuiltIn::Erc7201) => {
                self.evaluate_erc7201_built_in_call(&call.arguments)
            }
            _ => None,
        }
    }

    fn evaluate_erc7201_built_in_call(
        &mut self,
        arguments: &ir::ArgumentsDeclaration,
    ) -> Option<Number> {
        let ir::ArgumentsDeclaration::PositionalArguments(arguments) = arguments else {
            return None;
        };
        let [argument] = arguments.as_slice() else {
            // Reject other arities
            return None;
        };
        let value = self.evaluate_expression_as_string(argument)?;
        Some(Number::Integer(compute_erc7201(&value)))
    }

    fn evaluate_expression_as_string(&mut self, expression: &ir::Expression) -> Option<Vec<u8>> {
        match expression {
            ir::Expression::StringExpression(string_expression) => {
                let value = match string_expression {
                    ir::StringExpression::StringLiterals(literals) => {
                        literals::value_of_string_literals(literals)
                    }
                    ir::StringExpression::HexStringLiterals(literals) => {
                        literals::value_of_hex_string_literals(literals)
                    }
                    ir::StringExpression::UnicodeStringLiterals(literals) => {
                        literals::value_of_unicode_string_literals(literals)
                    }
                };
                Some(value)
            }
            ir::Expression::Identifier(identifier) => {
                self.evaluate_identifier_as_string(identifier)
            }
            _ => None,
        }
    }

    fn evaluate_identifier_as_string(&mut self, identifier: &ir::Identifier) -> Option<Vec<u8>> {
        let current_scope = self.scope_stack.last().expect("scope stack is empty");
        let ComptimeResolution::Constant {
            value: target_expression,
            target_scope,
        } = self
            .identifier_resolver
            .resolve_identifier_in_scope(identifier.unparse(), current_scope)
        else {
            return None;
        };
        if self.scope_stack.len() >= Self::MAX_SCOPE_DEPTH {
            // TODO(validation): cyclic dependency in constant resolution or max depth reached
            return None;
        }
        self.scope_stack.push(target_scope);
        let result = self.evaluate_expression_as_string(&target_expression);
        self.scope_stack
            .pop()
            .expect("scope stack should not be empty");
        result
    }
}

/// Computes the storage slot defined by ERC-7201 for the given value bytes:
/// `keccak256(abi.encode(uint256(keccak256(value)) - 1)) & ~bytes32(uint256(0xff))`.
fn compute_erc7201(value: &[u8]) -> BigInt {
    let inner: [u8; 32] = Keccak256::digest(value).into();
    // Wrapping subtraction matches Solidity's `uint256(...) - 1` semantics —
    // the inner hash being zero is negligibly rare but well-defined.
    let minus_one = U256::from_be_bytes(inner).wrapping_sub(U256::from(1u8));
    // `abi.encode(uint256(...))` is the value's 32-byte big-endian encoding.
    let encoded = minus_one.to_be_bytes::<32>();
    let mut outer: [u8; 32] = Keccak256::digest(encoded).into();
    // Mask off the lowest byte (`& ~bytes32(uint256(0xff))`).
    outer[31] = 0;
    BigInt::from_bytes_be(Sign::Plus, &outer)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use num_bigint::{BigInt, ToBigInt};
    use num_rational::BigRational;
    use num_traits::Num;
    use slang_solidity_v2_common::versions::LanguageVersion;
    use slang_solidity_v2_parser::{ParseOutput, Parser};

    use super::*;

    struct MapResolver {
        // qualified identifier => (target expression, target scope)
        // qualified identifier is the concatenation of the scope and identifier
        context: HashMap<String, (ir::Expression, String)>,
        // Tests that exercise the erc7201 evaluation path opt in by toggling
        // this flag, mirroring what the production resolver does when the
        // language version enables the built-in and it isn't shadowed.
        recognise_erc7201: bool,
    }

    impl ConstantIdentifierResolver<String> for MapResolver {
        fn resolve_identifier_in_scope(
            &self,
            identifier: &str,
            scope: &String,
        ) -> ComptimeResolution<String> {
            if let Some((target_expression, target_scope)) =
                self.context.get(&format!("{scope}{identifier}"))
            {
                ComptimeResolution::Constant {
                    value: target_expression.clone(),
                    target_scope: target_scope.clone(),
                }
            } else if self.recognise_erc7201 && identifier == "erc7201" {
                ComptimeResolution::BuiltIn(BuiltIn::Erc7201)
            } else {
                ComptimeResolution::Unresolved
            }
        }
    }

    impl MapResolver {
        fn build_context(
            context: &[(&str, &str, &str)],
        ) -> HashMap<String, (ir::Expression, String)> {
            context
                .iter()
                .map(|(name, input, target_scope)| {
                    (
                        (*name).to_string(),
                        (parse_expression(input), (*target_scope).to_string()),
                    )
                })
                .collect()
        }

        fn with_context(context: &[(&str, &str, &str)]) -> Self {
            Self {
                context: Self::build_context(context),
                recognise_erc7201: false,
            }
        }

        fn recognise_erc7201_with_context(context: &[(&str, &str, &str)]) -> Self {
            Self {
                context: Self::build_context(context),
                recognise_erc7201: true,
            }
        }
    }

    fn parse_expression(input: &str) -> ir::Expression {
        // NB. the declaration is only a harness to contain the expression
        let source = format!("uint constant x = {input};");
        let version = LanguageVersion::V0_8_35;

        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse("test.sol", &source, version);

        assert!(
            diagnostics.is_empty(),
            "Parser diagnostics: {diagnostics:?}"
        );

        let mut id_generator = ir::NodeIdGenerator::default();
        let ir::BuildOutput {
            ir_root,
            diagnostics,
        } = ir::build("test.sol", &source_unit, &source, &mut id_generator);

        assert!(
            diagnostics.is_empty(),
            "IR builder diagnostics: {diagnostics:?}"
        );

        let member = ir_root.members.first().expect("no source unit members");
        match member {
            ir::SourceUnitMember::ConstantDefinition(definition) => definition
                .value
                .clone()
                .expect("constant definition has no value expression"),
            _ => panic!("expected a ConstantDefinition"),
        }
    }

    // `context` is given as a list of constants defined as:
    // - the qualified identifier
    // - the target expression (to be parsed)
    // - the target scope
    // Resolution always starts at the empty string scope in these tests.
    fn eval_string_with_context(input: &str, context: &[(&str, &str, &str)]) -> Option<Number> {
        let resolver = MapResolver::with_context(context);
        let expression = parse_expression(input);

        evaluate_compile_time_number_constant(&expression, String::new(), &resolver)
    }

    fn eval_string(input: &str) -> Option<Number> {
        eval_string_with_context(input, &[])
    }

    #[test]
    fn test_literals() {
        assert!(
            eval_string("1").is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap()))
        );
        assert!(eval_string("42")
            .is_some_and(|value| value == Number::Integer(42.to_bigint().unwrap())));
        assert!(eval_string("0x01")
            .is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("0xa0")
            .is_some_and(|value| value == Number::Integer(160.to_bigint().unwrap())));
    }

    #[test]
    fn test_literals_with_digit_separators() {
        assert!(eval_string("1_000")
            .is_some_and(|value| value == Number::Integer(1_000.to_bigint().unwrap())));
        assert!(eval_string("1_000_000")
            .is_some_and(|value| value == Number::Integer(1_000_000.to_bigint().unwrap())));
        assert!(eval_string("0xdead_beef")
            .is_some_and(|value| value == Number::Integer(0xdead_beef_u64.to_bigint().unwrap())));
        assert!(eval_string("0x1234_5678")
            .is_some_and(|value| value == Number::Integer(0x1234_5678_u64.to_bigint().unwrap())));
    }

    #[test]
    fn test_literals_with_number_units() {
        assert!(eval_string("1 wei")
            .is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("1 gwei")
            .is_some_and(|value| value == Number::Integer(1_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("1 ether").is_some_and(
            |value| value == Number::Integer(1_000_000_000_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("2 ether").is_some_and(
            |value| value == Number::Integer(2_000_000_000_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("1 seconds")
            .is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("1 minutes")
            .is_some_and(|value| value == Number::Integer(60.to_bigint().unwrap())));
        assert!(eval_string("1 hours")
            .is_some_and(|value| value == Number::Integer(3_600.to_bigint().unwrap())));
        assert!(eval_string("1 days")
            .is_some_and(|value| value == Number::Integer(86_400.to_bigint().unwrap())));
        assert!(eval_string("1 weeks")
            .is_some_and(|value| value == Number::Integer(604_800.to_bigint().unwrap())));
    }

    #[test]
    fn test_literals_with_scientific_notation() {
        assert!(eval_string("1e3")
            .is_some_and(|value| value == Number::Integer(1_000.to_bigint().unwrap())));
        assert!(eval_string("2e10")
            .is_some_and(|value| value == Number::Integer(20_000_000_000u64.to_bigint().unwrap())));
        assert!(eval_string("1e18").is_some_and(
            |value| value == Number::Integer(1_000_000_000_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("1.5e3")
            .is_some_and(|value| value == Number::Integer(1_500.to_bigint().unwrap())));
    }

    #[test]
    fn test_reducible_rational_literals() {
        assert!(eval_string("1.5 ether").is_some_and(
            |value| value == Number::Integer(1_500_000_000_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("0.5 ether").is_some_and(
            |value| value == Number::Integer(500_000_000_000_000_000u64.to_bigint().unwrap())
        ));
        assert!(eval_string("0.5 gwei")
            .is_some_and(|value| value == Number::Integer(500_000_000u64.to_bigint().unwrap())));
    }

    #[test]
    fn test_non_reducible_rational_literals() {
        assert!(eval_string("0.5")
            .is_some_and(|value| value
                == Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(2)))));
        assert!(eval_string("3.14").is_some_and(|value| value
            == Number::Rational(BigRational::new(BigInt::from(157), BigInt::from(50)))));
        assert!(eval_string("1e-1")
            .is_some_and(|value| value
                == Number::Rational(BigRational::new(BigInt::from(1), BigInt::from(10)))));
    }

    #[test]
    fn test_rational_arithmetic_folds() {
        // Reducible rational result normalises back to an Integer.
        assert!(
            eval_string("1.5 * 2").is_some_and(|value| value == Number::Integer(BigInt::from(3)))
        );
        // Integer division that does not divide evenly yields a Rational.
        assert!(eval_string("5 / 2")
            .is_some_and(|value| value
                == Number::Rational(BigRational::new(BigInt::from(5), BigInt::from(2)))));
        // Negation of a non-reducing rational stays Rational.
        assert!(eval_string("-0.5")
            .is_some_and(|value| value
                == Number::Rational(BigRational::new(BigInt::from(-1), BigInt::from(2)))));
    }

    #[test]
    fn test_prefix_expression() {
        assert!(eval_string("-42")
            .is_some_and(|value| value == Number::Integer((-42).to_bigint().unwrap())));

        // Bitwise NOT on an arbitrary-precision integer: `~x = -x - 1`.
        assert!(eval_string("~1")
            .is_some_and(|value| value == Number::Integer((-2).to_bigint().unwrap())));
        assert!(eval_string("~0")
            .is_some_and(|value| value == Number::Integer((-1).to_bigint().unwrap())));
        assert!(eval_string("~(-1)")
            .is_some_and(|value| value == Number::Integer(0.to_bigint().unwrap())));
        // Hex source: `~` operates on the integer value (provenance is irrelevant here).
        assert!(eval_string("~0xff")
            .is_some_and(|value| value == Number::Integer((-256).to_bigint().unwrap())));
        // Bitwise NOT is not defined on rationals.
        assert!(eval_string("~0.5").is_none());
    }

    #[test]
    fn test_binary_expression() {
        assert!(eval_string("1 + 2")
            .is_some_and(|value| value == Number::Integer(3.to_bigint().unwrap())));
        assert!(eval_string("2 - 2")
            .is_some_and(|value| value == Number::Integer(0.to_bigint().unwrap())));
        assert!(eval_string("1 - 2")
            .is_some_and(|value| value == Number::Integer((-1).to_bigint().unwrap())));
        assert!(eval_string("1 * 2")
            .is_some_and(|value| value == Number::Integer(2.to_bigint().unwrap())));
        assert!(eval_string("4 / 2")
            .is_some_and(|value| value == Number::Integer(2.to_bigint().unwrap())));
        assert!(eval_string("5 % 2")
            .is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap())));
        assert!(eval_string("2 ** 5")
            .is_some_and(|value| value == Number::Integer(32.to_bigint().unwrap())));
        assert!(eval_string("32 << 2")
            .is_some_and(|value| value == Number::Integer(128.to_bigint().unwrap())));
        assert!(eval_string("32 >> 2")
            .is_some_and(|value| value == Number::Integer(8.to_bigint().unwrap())));
        assert!(eval_string("32 | 16")
            .is_some_and(|value| value == Number::Integer(48.to_bigint().unwrap())));
        assert!(eval_string("15 ^ 31")
            .is_some_and(|value| value == Number::Integer(16.to_bigint().unwrap())));
        assert!(eval_string("15 & 31")
            .is_some_and(|value| value == Number::Integer(15.to_bigint().unwrap())));
    }

    #[test]
    fn test_nesting_expressions() {
        assert!(eval_string("1 + (2 + 3)")
            .is_some_and(|value| value == Number::Integer(6.to_bigint().unwrap())));
        assert!(eval_string("3 * (2 + 1)")
            .is_some_and(|value| value == Number::Integer(9.to_bigint().unwrap())));
    }

    #[test]
    fn test_identifier_lookup() {
        assert!(eval_string_with_context("FOO", &[("FOO", "1", "")])
            .is_some_and(|value| value == Number::Integer(1.to_bigint().unwrap())));
        assert!(
            eval_string_with_context("FOO + 2*BAR", &[("FOO", "1", ""), ("BAR", "5", "")])
                .is_some_and(|value| value == Number::Integer(11.to_bigint().unwrap()))
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
        .is_some_and(|value| value == Number::Integer(42.to_bigint().unwrap())));
    }

    fn eval_string_with_erc7201_and_context(
        input: &str,
        context: &[(&str, &str, &str)],
    ) -> Option<Number> {
        let expression = parse_expression(input);
        let resolver = MapResolver::recognise_erc7201_with_context(context);
        evaluate_compile_time_number_constant(&expression, String::new(), &resolver)
    }

    fn eval_string_with_erc7201(input: &str) -> Option<Number> {
        eval_string_with_erc7201_and_context(input, &[])
    }

    #[test]
    fn test_erc7201_reference_vector() {
        // EIP-7201 test vector: `erc7201("example.main")` →
        // 0x183a6125c38840424c4a85fa12bab2ab606c4b6d0e7cc73c0c06ba5300eab500
        let expected = BigInt::from_str_radix(
            "183a6125c38840424c4a85fa12bab2ab606c4b6d0e7cc73c0c06ba5300eab500",
            16,
        )
        .map(Number::Integer)
        .unwrap();

        assert!(eval_string_with_erc7201(r#"erc7201("example.main")"#)
            .is_some_and(|value| value == expected));
        assert!(
            eval_string_with_erc7201(r#"erc7201(unicode"example.main")"#)
                .is_some_and(|value| value == expected)
        );
        assert!(
            eval_string_with_erc7201(r#"erc7201(hex"6578616D706C652E6D61696E")"#)
                .is_some_and(|value| value == expected)
        );
    }

    #[test]
    fn test_erc7201_folds_inside_arithmetic() {
        // Confirms the function-call result feeds further folding
        let expected = BigInt::from_str_radix("b500", 16).unwrap();
        assert!(
            eval_string_with_erc7201(r#"erc7201("example.main") & 0xffff"#)
                .is_some_and(|value| value == Number::Integer(expected))
        );
    }

    #[test]
    fn test_erc7201_not_recognised_without_resolver_opt_in() {
        // Same expression, but the default resolver doesn't claim "erc7201"
        // is the built-in; evaluation falls through to `None`.
        assert!(eval_string(r#"erc7201("example.main")"#).is_none());
    }

    #[test]
    fn test_erc7201_rejects_wrong_arity() {
        assert!(eval_string_with_erc7201(r#"erc7201()"#).is_none());
        assert!(eval_string_with_erc7201(r#"erc7201("a", "b")"#).is_none());
    }

    #[test]
    fn test_erc7201_resolves_identifier_arguments() {
        let expected = BigInt::from_str_radix(
            "183a6125c38840424c4a85fa12bab2ab606c4b6d0e7cc73c0c06ba5300eab500",
            16,
        )
        .unwrap();
        assert!(eval_string_with_erc7201_and_context(
            r#"erc7201(NAME)"#,
            &[
                ("NAME", "NAME", "SUB."),
                ("SUB.NAME", "\"example.main\"", "")
            ],
        )
        .is_some_and(|value| value == Number::Integer(expected)));
    }
}
