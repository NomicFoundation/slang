use num_bigint::{BigInt, Sign};
use ruint::aliases::U256;
use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::diagnostics::kinds::semantic::CyclicConstantDefinition;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ConstantArithmeticError, IncompatibleConstantOperator, TypeSystemDiagnosticKind,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, ConstantDefinition, Definition, Resolution, Scope, ScopeId};
use crate::built_ins::{BuiltInsResolver, InternalBuiltIn};
use crate::types::literals::numbers::{integer_literal_fits, rational_literal_fits};
use crate::types::{
    literals, BinaryOperator, IntegerType, LiteralKind, Number, Type, TypeId, TypeRegistry,
    UnaryOperator,
};

/// Reason why a constant expression could not be evaluated.
#[derive(Debug)]
pub(crate) enum EvaluationError {
    /// The failure has a diagnostic.
    Diagnostic {
        /// The diagnostic to report.
        kind: DiagnosticKind,
        /// The operation that raised the failure. In some cases we want to
        /// point at the exact expression that produced the error instead of
        /// the caller's own location. `None` means the caller should attach
        /// its own location.
        expression: Option<ir::Expression>,
    },
    /// Evaluation did not produce a constant value (e.g. the expression is
    /// not a compile-time constant).
    CouldNotEvaluate,
}

impl From<DiagnosticKind> for EvaluationError {
    fn from(kind: DiagnosticKind) -> Self {
        Self::Diagnostic {
            kind,
            expression: None,
        }
    }
}

pub(crate) fn evaluate_compile_time_constant<Scope>(
    expression: &ir::Expression,
    start_scope: Scope,
    types: &mut TypeRegistry,
    identifier_resolver: &dyn ConstantIdentifierResolver<Scope>,
) -> Result<Number, EvaluationError> {
    let mut evaluator = CompileConstantEvaluator {
        identifier_resolver,
        types,
        scope_stack: Vec::new(),
    };
    evaluator
        .evaluate_expression_in_scope(expression, start_scope)
        .map(|evaluated| evaluated.value().clone())
}

/// An evaluated value together with its type.
enum TypedValue {
    /// A literal number.
    Literal(Number),
    /// A value with a concrete integer type.
    Integer(Number, IntegerType),
}

impl TypedValue {
    fn value(&self) -> &Number {
        match self {
            Self::Literal(value) | Self::Integer(value, _) => value,
        }
    }

    fn to_type(&self) -> Type {
        match self {
            Self::Literal(value) => Type::Literal(value.to_literal_kind()),
            Self::Integer(_, integer) => Type::Integer(integer.clone()),
        }
    }

    /// Describes the operand for a diagnostic. A typed integer as
    /// `uint256`/`int8`, and an untyped literal by value as
    /// `int_const N` or `rational_const N / D`.
    fn describe(&self) -> String {
        match self {
            Self::Literal(value) => value.to_string(),
            Self::Integer(_, IntegerType { is_signed, bits }) => {
                format!("{}{bits}", if *is_signed { "int" } else { "uint" })
            }
        }
    }

    /// Evaluates an arithmetic, bitwise, shift or exponentiation operator.
    fn binary(
        &self,
        op: BinaryOperator,
        rhs: &Self,
        types: &mut TypeRegistry,
    ) -> Result<Self, EvaluationError> {
        let left = types.register_type(self.to_type());
        let right = types.register_type(rhs.to_type());
        let result_id = types
            .type_of_binary_operator(left, right, op)
            .ok_or_else(|| {
                EvaluationError::from(DiagnosticKind::from(IncompatibleConstantOperator {
                    operator: op.as_str().to_owned(),
                    left_type: self.describe(),
                    right_type: rhs.describe(),
                }))
            })?;
        Self::from_result_type(result_id, types, |integer| {
            let left = self.value().coerce_to_integer(integer)?;
            let right = rhs.value().coerce_to_integer(integer)?;
            op.fold(left.value(), right.value())
        })
    }

    /// Evaluates a negation or bitwise not operator.
    fn unary(&self, op: UnaryOperator, types: &mut TypeRegistry) -> Result<Self, EvaluationError> {
        let operand = types.register_type(self.to_type());
        let result_id = types
            .type_of_unary_operator(operand, op)
            .ok_or(EvaluationError::CouldNotEvaluate)?;
        Self::from_result_type(result_id, types, |integer| {
            let operand = self.value().coerce_to_integer(integer)?;
            op.fold(operand.value())
        })
    }

    /// Materializes an operator result from its computed result type. A
    /// literal result type already carries the folded value. For an integer
    /// result type, `fold_at` coerces the operands to that type and applies
    /// the operator, and the folded result is range-checked against it.
    fn from_result_type(
        result_id: TypeId,
        types: &TypeRegistry,
        fold_at: impl FnOnce(&IntegerType) -> Option<Number>,
    ) -> Result<Self, EvaluationError> {
        match types.get_type_by_id(result_id) {
            Type::Literal(kind) => match kind {
                LiteralKind::Integer { .. } | LiteralKind::Rational { .. } => {
                    Number::from_literal_kind(kind)
                        .map(Self::Literal)
                        .ok_or(EvaluationError::CouldNotEvaluate)
                }
                _ => unreachable!("literal operator result is always integer or rational"),
            },
            Type::Integer(integer) => {
                let result = fold_at(integer).ok_or(EvaluationError::CouldNotEvaluate)?;
                result
                    .coerce_to_integer(integer)
                    .ok_or_else(|| DiagnosticKind::from(ConstantArithmeticError).into())
            }
            _ => Err(EvaluationError::CouldNotEvaluate),
        }
    }
}

impl Number {
    /// Coerces this value to fit the `target` integer type, producing the
    /// corresponding `TypedValue`, or `None` if it does not fit.
    fn coerce_to_integer(&self, target: &IntegerType) -> Option<TypedValue> {
        match self {
            // Already an integer, so just do range check.
            Number::Integer(integer) => {
                integer_literal_fits(integer, target.is_signed, target.bits)
                    .then(|| TypedValue::Integer(self.clone(), target.clone()))
            }
            // Convert the rational number to an integer if it fits within
            // the integer range.
            Number::Rational(rational) => {
                rational_literal_fits(rational, target.is_signed, target.bits).then(|| {
                    TypedValue::Integer(Number::Integer(rational.to_integer()), target.clone())
                })
            }
        }
    }
}

pub(crate) enum ComptimeResolution<Scope> {
    Unresolved,
    Constant {
        value: ir::Expression,
        target_scope: Scope,
        /// Integer type of a constant, or `None` if it is not.
        integer_type: Option<IntegerType>,
    },
    BuiltIn(InternalBuiltIn),
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
    types: &'a mut TypeRegistry,
    scope_stack: Vec<Scope>,
}

impl<Scope> CompileConstantEvaluator<'_, Scope> {
    const MAX_SCOPE_DEPTH: usize = 32;

    fn evaluate_expression_in_scope(
        &mut self,
        expression: &ir::Expression,
        scope: Scope,
    ) -> Result<TypedValue, EvaluationError> {
        if self.scope_stack.len() >= Self::MAX_SCOPE_DEPTH {
            return Err(DiagnosticKind::from(CyclicConstantDefinition).into());
        }
        self.scope_stack.push(scope);
        let result = self.evaluate_expression(expression);
        self.scope_stack
            .pop()
            .expect("scope stack should not be empty");
        result
    }

    #[allow(clippy::too_many_lines)]
    fn evaluate_expression(
        &mut self,
        expression: &ir::Expression,
    ) -> Result<TypedValue, EvaluationError> {
        let result = match expression {
            ir::Expression::BitwiseOrExpression(bitwise_or_expression) => self.evaluate_binary(
                &bitwise_or_expression.left_operand,
                &bitwise_or_expression.right_operand,
                BinaryOperator::BitOr,
            ),
            ir::Expression::BitwiseXorExpression(bitwise_xor_expression) => self.evaluate_binary(
                &bitwise_xor_expression.left_operand,
                &bitwise_xor_expression.right_operand,
                BinaryOperator::BitXor,
            ),
            ir::Expression::BitwiseAndExpression(bitwise_and_expression) => self.evaluate_binary(
                &bitwise_and_expression.left_operand,
                &bitwise_and_expression.right_operand,
                BinaryOperator::BitAnd,
            ),
            ir::Expression::ShiftExpression(shift_expression) => match &shift_expression.operator {
                ir::ShiftExpressionOperator::LessThanLessThan(_) => self.evaluate_binary(
                    &shift_expression.left_operand,
                    &shift_expression.right_operand,
                    BinaryOperator::Shl,
                ),
                ir::ShiftExpressionOperator::GreaterThanGreaterThan(_) => self.evaluate_binary(
                    &shift_expression.left_operand,
                    &shift_expression.right_operand,
                    BinaryOperator::Shr,
                ),
                ir::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(_) => {
                    Err(EvaluationError::CouldNotEvaluate)
                }
            },
            ir::Expression::AdditiveExpression(additive_expression) => {
                match &additive_expression.operator {
                    ir::AdditiveExpressionOperator::Plus(_) => self.evaluate_binary(
                        &additive_expression.left_operand,
                        &additive_expression.right_operand,
                        BinaryOperator::Add,
                    ),
                    ir::AdditiveExpressionOperator::Minus(_) => self.evaluate_binary(
                        &additive_expression.left_operand,
                        &additive_expression.right_operand,
                        BinaryOperator::Sub,
                    ),
                }
            }
            ir::Expression::MultiplicativeExpression(multiplicative_expression) => {
                match &multiplicative_expression.operator {
                    ir::MultiplicativeExpressionOperator::Asterisk(_) => self.evaluate_binary(
                        &multiplicative_expression.left_operand,
                        &multiplicative_expression.right_operand,
                        BinaryOperator::Mul,
                    ),
                    ir::MultiplicativeExpressionOperator::Slash(_) => self.evaluate_binary(
                        &multiplicative_expression.left_operand,
                        &multiplicative_expression.right_operand,
                        BinaryOperator::Div,
                    ),
                    ir::MultiplicativeExpressionOperator::Percent(_) => self.evaluate_binary(
                        &multiplicative_expression.left_operand,
                        &multiplicative_expression.right_operand,
                        BinaryOperator::Rem,
                    ),
                }
            }
            // v2 ExponentiationExpression has no explicit operator field (only `**` exists)
            ir::Expression::ExponentiationExpression(exponentiation_expression) => self
                .evaluate_binary(
                    &exponentiation_expression.left_operand,
                    &exponentiation_expression.right_operand,
                    BinaryOperator::Pow,
                ),
            ir::Expression::PrefixExpression(prefix_expression) => {
                match &prefix_expression.operator {
                    ir::PrefixExpressionOperator::Minus(_) => {
                        self.evaluate_unary(&prefix_expression.operand, UnaryOperator::Negate)
                    }
                    ir::PrefixExpressionOperator::Tilde(_) => {
                        self.evaluate_unary(&prefix_expression.operand, UnaryOperator::BitNot)
                    }
                    _ => Err(EvaluationError::CouldNotEvaluate),
                }
            }
            ir::Expression::HexNumberExpression(hex_number_expression) => {
                Number::from_hex_number_expression(hex_number_expression)
                    .map(TypedValue::Literal)
                    .ok_or(EvaluationError::CouldNotEvaluate)
            }
            ir::Expression::DecimalNumberExpression(decimal_number_expression) => {
                Number::from_decimal_number_expression(decimal_number_expression)
                    .map(TypedValue::Literal)
                    .ok_or(EvaluationError::CouldNotEvaluate)
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
                Err(EvaluationError::CouldNotEvaluate)
            }
        };

        // For operator errors we attach the innermost expression where the
        // error happened. A cyclic constant definition is left for the caller
        // to locate, as the `constant_cycles` analysis emits a more helpful
        // diagnostic for it.
        result.map_err(|error| match error {
            EvaluationError::Diagnostic {
                kind:
                    kind @ DiagnosticKind::TypeSystem(
                        TypeSystemDiagnosticKind::IncompatibleConstantOperator(_)
                        | TypeSystemDiagnosticKind::ConstantArithmeticError(_),
                    ),
                expression: None,
            } => EvaluationError::Diagnostic {
                kind,
                expression: Some(expression.clone()),
            },
            other => other,
        })
    }

    fn evaluate_binary(
        &mut self,
        left: &ir::Expression,
        right: &ir::Expression,
        op: BinaryOperator,
    ) -> Result<TypedValue, EvaluationError> {
        let lhs = self.evaluate_expression(left)?;
        let rhs = self.evaluate_expression(right)?;
        lhs.binary(op, &rhs, self.types)
    }

    fn evaluate_unary(
        &mut self,
        operand: &ir::Expression,
        op: UnaryOperator,
    ) -> Result<TypedValue, EvaluationError> {
        let value = self.evaluate_expression(operand)?;
        value.unary(op, self.types)
    }

    fn evaluate_identifier(
        &mut self,
        identifier: &ir::Identifier,
    ) -> Result<TypedValue, EvaluationError> {
        let current_scope = self.scope_stack.last().expect("scope stack is empty");
        let ComptimeResolution::Constant {
            value: target_expression,
            target_scope,
            integer_type,
        } = self
            .identifier_resolver
            .resolve_identifier_in_scope(identifier.unparse(), current_scope)
        else {
            return Err(EvaluationError::CouldNotEvaluate);
        };
        // Reject constants that are not declared with an integer type.
        let integer_type = integer_type.ok_or(EvaluationError::CouldNotEvaluate)?;
        let evaluated = self.evaluate_expression_in_scope(&target_expression, target_scope)?;
        evaluated
            .value()
            .coerce_to_integer(&integer_type)
            .ok_or(EvaluationError::CouldNotEvaluate)
    }

    fn evaluate_tuple_expression(
        &mut self,
        tuple_expression: &ir::TupleExpression,
    ) -> Result<TypedValue, EvaluationError> {
        if tuple_expression.items.len() == 1 {
            let inner_expression = tuple_expression.items[0]
                .expression
                .as_ref()
                .ok_or(EvaluationError::CouldNotEvaluate)?;
            self.evaluate_expression(inner_expression)
        } else {
            Err(EvaluationError::CouldNotEvaluate)
        }
    }

    /// Compile-time evaluation of a built-in function call. Only `erc7201`
    /// is supported as of Solidity 0.8.35. Everything else fails.
    fn evaluate_function_call_expression(
        &mut self,
        call: &ir::FunctionCallExpression,
    ) -> Result<TypedValue, EvaluationError> {
        let ir::Expression::Identifier(operand) = &call.operand else {
            return Err(EvaluationError::CouldNotEvaluate);
        };
        let scope = self.scope_stack.last().expect("scope stack is empty");
        let resolution = self
            .identifier_resolver
            .resolve_identifier_in_scope(operand.unparse(), scope);
        match resolution {
            ComptimeResolution::BuiltIn(InternalBuiltIn::Erc7201) => {
                self.evaluate_erc7201_built_in_call(&call.arguments)
            }
            _ => Err(EvaluationError::CouldNotEvaluate),
        }
    }

    fn evaluate_erc7201_built_in_call(
        &mut self,
        arguments: &ir::ArgumentsDeclaration,
    ) -> Result<TypedValue, EvaluationError> {
        let ir::ArgumentsDeclaration::PositionalArguments(arguments) = arguments else {
            return Err(EvaluationError::CouldNotEvaluate);
        };
        let [argument] = arguments.as_slice() else {
            // Reject other arities
            return Err(EvaluationError::CouldNotEvaluate);
        };
        let value = self.evaluate_expression_as_string(argument)?;
        Ok(TypedValue::Integer(
            Number::Integer(compute_erc7201(&value)),
            IntegerType {
                is_signed: false,
                bits: 256,
            },
        ))
    }

    fn evaluate_expression_as_string(
        &mut self,
        expression: &ir::Expression,
    ) -> Result<Vec<u8>, EvaluationError> {
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
                Ok(value)
            }
            ir::Expression::Identifier(identifier) => {
                self.evaluate_identifier_as_string(identifier)
            }
            _ => Err(EvaluationError::CouldNotEvaluate),
        }
    }

    fn evaluate_identifier_as_string(
        &mut self,
        identifier: &ir::Identifier,
    ) -> Result<Vec<u8>, EvaluationError> {
        let current_scope = self.scope_stack.last().expect("scope stack is empty");
        let ComptimeResolution::Constant {
            value: target_expression,
            target_scope,
            ..
        } = self
            .identifier_resolver
            .resolve_identifier_in_scope(identifier.unparse(), current_scope)
        else {
            return Err(EvaluationError::CouldNotEvaluate);
        };
        if self.scope_stack.len() >= Self::MAX_SCOPE_DEPTH {
            return Err(DiagnosticKind::from(CyclicConstantDefinition).into());
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

/// Returns a constant's integer type from its declared type name. Returns
/// `None` for anything but an elementary integer type.
fn declared_integer_type(type_name: &ir::TypeName) -> Option<IntegerType> {
    let ir::TypeName::ElementaryType(elementary_type) = type_name else {
        return None;
    };
    let parsed_type = match elementary_type {
        ir::ElementaryType::IntKeyword(keyword) => Type::from_int_keyword(keyword.unparse()),
        ir::ElementaryType::UintKeyword(keyword) => Type::from_uint_keyword(keyword.unparse()),
        _ => return None,
    };
    let Type::Integer(integer_type) = parsed_type else {
        unreachable!("int and uint keywords always parse to integer types");
    };
    Some(integer_type)
}

/// Resolves constant identifiers for the compile-time evaluator.
pub(crate) struct ConstantResolver<'a> {
    pub(crate) binder: &'a Binder,
    /// When set to the `(file id, offset)` of the evaluated expression,
    /// same-file constants declared past that offset resolve as `Unresolved`.
    /// This is a hack to match solc behaviour: forward references are
    /// rejected in array lengths, but valid in storage base slots (`None`).
    pub(crate) use_site: Option<(&'a FileId, usize)>,
}

impl ConstantResolver<'_> {
    /// Whether the constant is declared past the use site, in the same file.
    /// The comparison is always against the original use site, so a chain
    /// through an earlier constant is rejected as well.
    fn is_forward_reference(&self, constant_definition: &ConstantDefinition) -> bool {
        let Some((use_file_id, use_offset)) = self.use_site else {
            return false;
        };
        if constant_definition.ir_node.range.start <= use_offset {
            return false;
        }
        // Constants are only declared in file scopes, or in the contract
        // scopes of contracts, interfaces and libraries.
        let file_id = match self
            .binder
            .get_scope_by_id(constant_definition.enclosing_scope_id)
        {
            Scope::File(file_scope) => &file_scope.file_id,
            Scope::Contract(contract_scope) => {
                match self.binder.get_scope_by_id(contract_scope.file_scope_id) {
                    Scope::File(file_scope) => &file_scope.file_id,
                    _ => unreachable!("a contract scope's parent is a file scope"),
                }
            }
            _ => unreachable!("constants are only defined in file or contract scopes"),
        };
        file_id == use_file_id
    }
}

impl ConstantIdentifierResolver<ScopeId> for ConstantResolver<'_> {
    fn resolve_identifier_in_scope(
        &self,
        identifier: &str,
        scope_id: &ScopeId,
    ) -> ComptimeResolution<ScopeId> {
        let resolution = self.binder.resolve_in_scope(*scope_id, identifier);
        // Follow import aliases (eg. `import {A as B} from "..."`).
        match self.binder.follow_symbol_aliases(resolution) {
            Resolution::Definition(definition_id) => {
                match self
                    .binder
                    .find_definition_by_id(definition_id)
                    .expect("resolved definition is found")
                {
                    Definition::Constant(constant_definition) => {
                        // Hack to match solc behavior by rejecting forward references
                        // within the same file for array length.
                        if self.is_forward_reference(constant_definition) {
                            ComptimeResolution::Unresolved
                        } else if let Some(value) = &constant_definition.ir_node.value {
                            let integer_type =
                                declared_integer_type(&constant_definition.ir_node.type_name);
                            ComptimeResolution::Constant {
                                value: value.clone(),
                                target_scope: constant_definition.enclosing_scope_id,
                                integer_type,
                            }
                        } else {
                            // This is a `constant` state variable for which the
                            // grammar allows an absent value expression but
                            // it's a semantic/syntactic error which should be
                            // already reported.
                            ComptimeResolution::Unresolved
                        }
                    }
                    _ => ComptimeResolution::Unresolved,
                }
            }
            Resolution::Ambiguous(_) => {
                // TODO(validation) SDR[1731]: multiple definitions found which is an error
                ComptimeResolution::Unresolved
            }
            Resolution::BuiltIn(_) => unreachable!("the binder doesn't resolve to built-ins"),

            Resolution::Unresolved => {
                // Try to resolve a built-in using the scope as context
                let built_in = match self.binder.get_scope_by_id(*scope_id) {
                    Scope::Block(_)
                    | Scope::Chained(_)
                    | Scope::Contract(_)
                    | Scope::File(_)
                    | Scope::Function(_)
                    | Scope::Modifier(_) => BuiltInsResolver::lookup_global(identifier),

                    Scope::Enum(_) | Scope::Parameters(_) | Scope::Struct(_) | Scope::Using(_) => {
                        None
                    }

                    Scope::YulBlock(_) | Scope::YulFunction(_) => {
                        unreachable!("constant evaluation never enters a Yul scope")
                    }
                };
                built_in.map_or(ComptimeResolution::Unresolved, ComptimeResolution::BuiltIn)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::{BigInt, ToBigInt};
    use num_rational::BigRational;
    use num_traits::Num;
    use slang_solidity_v2_common::collections::Map;
    use slang_solidity_v2_common::versions::LanguageVersion;
    use slang_solidity_v2_parser::{ParseOutput, Parser};

    use super::*;

    struct MapResolver {
        // qualified identifier => (target expression, target scope)
        // qualified identifier is the concatenation of the scope and identifier
        context: Map<String, (ir::Expression, String)>,
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
                    // The tests model every known constant as a `uint256`.
                    integer_type: Some(IntegerType {
                        is_signed: false,
                        bits: 256,
                    }),
                }
            } else if self.recognise_erc7201 && identifier == "erc7201" {
                ComptimeResolution::BuiltIn(InternalBuiltIn::Erc7201)
            } else {
                ComptimeResolution::Unresolved
            }
        }
    }

    impl MapResolver {
        fn build_context(context: &[(&str, &str, &str)]) -> Map<String, (ir::Expression, String)> {
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
        let version = LanguageVersion::LATEST;

        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse(&"test.sol".into(), &source, version);

        assert!(
            diagnostics.is_empty(),
            "Parser diagnostics: {diagnostics:?}"
        );

        let mut id_generator = ir::NodeIdGenerator::default();
        let ir::BuildOutput {
            ir_root,
            diagnostics,
        } = ir::build(&"test.sol".into(), &source_unit, &source, &mut id_generator);

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
        let mut types = TypeRegistry::new(LanguageVersion::LATEST);
        let resolver = MapResolver::with_context(context);
        let expression = parse_expression(input);

        evaluate_compile_time_constant(&expression, String::new(), &mut types, &resolver).ok()
    }

    fn eval_string(input: &str) -> Option<Number> {
        eval_string_with_context(input, &[])
    }

    fn integer(value: i64) -> Number {
        Number::Integer(value.to_bigint().unwrap())
    }

    fn rational(numerator: i64, denominator: i64) -> Number {
        Number::Rational(BigRational::new(
            BigInt::from(numerator),
            BigInt::from(denominator),
        ))
    }

    #[test]
    fn test_literals() {
        assert!(eval_string("1").is_some_and(|value| value == integer(1)));
        assert!(eval_string("42").is_some_and(|value| value == integer(42)));
        assert!(eval_string("0x01").is_some_and(|value| value == integer(1)));
        assert!(eval_string("0xa0").is_some_and(|value| value == integer(160)));
    }

    #[test]
    fn test_literals_with_digit_separators() {
        assert!(eval_string("1_000").is_some_and(|value| value == integer(1_000)));
        assert!(eval_string("1_000_000").is_some_and(|value| value == integer(1_000_000)));
        assert!(eval_string("0xdead_beef").is_some_and(|value| value == integer(0xdead_beef)));
        assert!(eval_string("0x1234_5678").is_some_and(|value| value == integer(0x1234_5678)));
    }

    #[test]
    fn test_literals_with_number_units() {
        assert!(eval_string("1 wei").is_some_and(|value| value == integer(1)));
        assert!(eval_string("1 gwei").is_some_and(|value| value == integer(1_000_000_000)));
        assert!(
            eval_string("1 ether").is_some_and(|value| value == integer(1_000_000_000_000_000_000))
        );
        assert!(
            eval_string("2 ether").is_some_and(|value| value == integer(2_000_000_000_000_000_000))
        );
        assert!(eval_string("1 seconds").is_some_and(|value| value == integer(1)));
        assert!(eval_string("1 minutes").is_some_and(|value| value == integer(60)));
        assert!(eval_string("1 hours").is_some_and(|value| value == integer(3_600)));
        assert!(eval_string("1 days").is_some_and(|value| value == integer(86_400)));
        assert!(eval_string("1 weeks").is_some_and(|value| value == integer(604_800)));
    }

    #[test]
    fn test_literals_with_scientific_notation() {
        assert!(eval_string("1e3").is_some_and(|value| value == integer(1_000)));
        assert!(eval_string("2e10").is_some_and(|value| value == integer(20_000_000_000)));
        assert!(
            eval_string("1e18").is_some_and(|value| value == integer(1_000_000_000_000_000_000))
        );
        assert!(eval_string("1.5e3").is_some_and(|value| value == integer(1_500)));
    }

    #[test]
    fn test_reducible_rational_literals() {
        assert!(eval_string("1.5 ether")
            .is_some_and(|value| value == integer(1_500_000_000_000_000_000)));
        assert!(
            eval_string("0.5 ether").is_some_and(|value| value == integer(500_000_000_000_000_000))
        );
        assert!(eval_string("0.5 gwei").is_some_and(|value| value == integer(500_000_000)));
    }

    #[test]
    fn test_non_reducible_rational_literals() {
        assert!(eval_string("0.5").is_some_and(|value| value == rational(1, 2)));
        assert!(eval_string("3.14").is_some_and(|value| value == rational(157, 50)));
        assert!(eval_string("1e-1").is_some_and(|value| value == rational(1, 10)));
    }

    #[test]
    fn test_rational_arithmetic_folds() {
        // Reducible rational result normalises back to an Integer.
        assert!(eval_string("1.5 * 2").is_some_and(|value| value == integer(3)));
        // Integer division between literals stays an exact rational (no typed
        // integer involved): `5 / 2` is `5/2`, not the truncated `2`.
        assert!(eval_string("5 / 2").is_some_and(|value| value == rational(5, 2)));
        // Negation of a non-reducing rational stays Rational.
        assert!(eval_string("-0.5").is_some_and(|value| value == rational(-1, 2)));
    }

    #[test]
    fn test_prefix_expression() {
        assert!(eval_string("-42").is_some_and(|value| value == integer(-42)));

        // Bitwise NOT on an arbitrary-precision integer: `~x = -x - 1`.
        assert!(eval_string("~1").is_some_and(|value| value == integer(-2)));
        assert!(eval_string("~0").is_some_and(|value| value == integer(-1)));
        assert!(eval_string("~(-1)").is_some_and(|value| value == integer(0)));
        // Hex source: `~` operates on the integer value (provenance is irrelevant here).
        assert!(eval_string("~0xff").is_some_and(|value| value == integer(-256)));
        // Bitwise NOT is not defined on rationals.
        assert!(eval_string("~0.5").is_none());
    }

    #[test]
    fn test_binary_expression() {
        assert!(eval_string("1 + 2").is_some_and(|value| value == integer(3)));
        assert!(eval_string("2 - 2").is_some_and(|value| value == integer(0)));
        assert!(eval_string("1 - 2").is_some_and(|value| value == integer(-1)));
        assert!(eval_string("1 * 2").is_some_and(|value| value == integer(2)));
        assert!(eval_string("4 / 2").is_some_and(|value| value == integer(2)));
        assert!(eval_string("5 % 2").is_some_and(|value| value == integer(1)));
        assert!(eval_string("2 ** 5").is_some_and(|value| value == integer(32)));
        assert!(eval_string("32 << 2").is_some_and(|value| value == integer(128)));
        assert!(eval_string("32 >> 2").is_some_and(|value| value == integer(8)));
        assert!(eval_string("32 | 16").is_some_and(|value| value == integer(48)));
        assert!(eval_string("15 ^ 31").is_some_and(|value| value == integer(16)));
        assert!(eval_string("15 & 31").is_some_and(|value| value == integer(15)));
    }

    #[test]
    fn test_nesting_expressions() {
        assert!(eval_string("1 + (2 + 3)").is_some_and(|value| value == integer(6)));
        assert!(eval_string("3 * (2 + 1)").is_some_and(|value| value == integer(9)));
    }

    #[test]
    fn test_identifier_lookup() {
        assert!(
            eval_string_with_context("FOO", &[("FOO", "1", "")]).is_some_and(|v| v == integer(1))
        );
        assert!(
            eval_string_with_context("FOO + 2*BAR", &[("FOO", "1", ""), ("BAR", "5", "")])
                .is_some_and(|value| value == integer(11))
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
        .is_some_and(|value| value == integer(42)));
    }

    #[test]
    fn test_integer_typed_division_truncates() {
        // Division involving a typed integer constant (`B`) truncates toward
        // zero, matching Solidity's runtime integer division.
        assert_eq!(
            eval_string_with_context("10 / B", &[("B", "3", "")]),
            Some(integer(3)),
        );
        assert_eq!(
            eval_string_with_context("1 / B", &[("B", "2", "")]),
            Some(integer(0)),
        );
        // Truncation happens at the integer-typed division, so the fractional
        // intermediate is gone: `(1 / B) * B` is `0`, not `1`.
        assert_eq!(
            eval_string_with_context("(1 / B) * B", &[("B", "2", "")]),
            Some(integer(0)),
        );
        // Whole literals combine with a typed integer fine: `B * 7 / 2` is
        // `14 / 2 = 7`.
        assert_eq!(
            eval_string_with_context("B * 7 / 2", &[("B", "2", "")]),
            Some(integer(7)),
        );
        // The contrast that matters: `10 / B * 3` with a typed `B = 3` truncates
        // at the division (`10 / 3 = 3`), so the result is `3 * 3 = 9`.
        assert_eq!(
            eval_string_with_context("10 / B * 3", &[("B", "3", "")]),
            Some(integer(9)),
        );

        // The same shape as above but all-literal: `10 / 3` stays the exact
        // rational `10/3`, so `* 3` brings it back to `10` (no truncation).
        assert_eq!(eval_string("10 / 3 * 3"), Some(integer(10)));
        // All-literal division stays an exact rational (no typed operand).
        assert_eq!(eval_string("10 / 4"), Some(rational(5, 2)));
    }

    #[test]
    fn test_negative_literal_meets_unsigned_is_rejected() {
        // A negative literal has no common type with an unsigned integer, so the
        // result is rejected rather than truncated.
        assert_eq!(
            eval_string_with_context("(0 - 7) / B", &[("B", "2", "")]),
            None
        );
        // A fractional literal likewise cannot meet a typed integer.
        assert_eq!(
            eval_string_with_context("(7 / 2) * B", &[("B", "2", "")]),
            None
        );
        assert_eq!(eval_string_with_context("1.5 / B", &[("B", "2", "")]), None);
    }

    #[test]
    fn test_integer_overflow_is_rejected() {
        // `~B` for an unsigned `B` is negative, which is out of range for the
        // unsigned type, so the conversion fails.
        assert_eq!(eval_string_with_context("~B", &[("B", "3", "")]), None);
        // Unary negation of an unsigned integer has no result type.
        assert_eq!(eval_string_with_context("-B", &[("B", "3", "")]), None);
        // A literal that exceeds 256 bits cannot meet a typed integer, since
        // `2 ** 256` does not fit any integer type.
        assert_eq!(
            eval_string_with_context("2 ** 256 / B", &[("B", "3", "")]),
            None
        );
    }

    #[test]
    fn test_integer_typed_pow() {
        // Integer exponentiation with a typed base evaluates to an integer.
        assert_eq!(
            eval_string_with_context("B ** 2", &[("B", "3", "")]),
            Some(integer(9)),
        );
        // A negative exponent on a typed integer is invalid (it would otherwise
        // evaluate to a fraction); it is rejected, not truncated to `0`.
        assert_eq!(eval_string_with_context("B ** -1", &[("B", "3", "")]), None);
        // A fractional base cannot meet a typed-integer exponent.
        assert_eq!(
            eval_string_with_context("(7 / 2) ** B", &[("B", "3", "")]),
            None
        );
        // All-literal negative exponent stays an exact rational (it is only
        // rejected once it reaches an integer context, not during evaluation).
        assert_eq!(eval_string("2 ** -1"), Some(rational(1, 2)));
    }

    #[test]
    fn test_literal_unbounded_then_integer_typed() {
        // All-literal arithmetic is exact even past 256 bits, as long as it
        // never meets a typed integer, so `2 ** 256 * 500e-3` evaluates to `2 ** 255`.
        assert_eq!(
            eval_string("2 ** 256 * 500e-3"),
            Some(Number::Integer(BigInt::from(2).pow(255))),
        );
    }

    fn eval_string_with_erc7201_and_context(
        input: &str,
        context: &[(&str, &str, &str)],
    ) -> Option<Number> {
        let mut types = TypeRegistry::new(LanguageVersion::LATEST);
        let resolver = MapResolver::recognise_erc7201_with_context(context);
        let expression = parse_expression(input);
        evaluate_compile_time_constant(&expression, String::new(), &mut types, &resolver).ok()
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
