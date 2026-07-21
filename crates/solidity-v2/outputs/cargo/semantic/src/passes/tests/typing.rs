use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ArrayLengthFractional, ArrayLengthNotConstant, ArrayLengthZero, ConstantArithmeticError,
    IncompatibleConstantOperator, StorageLayoutBaseNotConstant,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};

use super::{build_file, TestFile};
use crate::binder::{Binder, Definition};
use crate::context::{FileNodeMapper, SemanticContext, SemanticFile};
use crate::passes::common::node_id_for_expression_typing;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p5_resolve_references,
};
use crate::types::{
    ArrayType, ByteArrayType, BytesType, ContractType, DataLocation, FixedSizeArrayType,
    IntegerType, LibraryType, LiteralKind, MappingType, MetaType, StringType, StructType,
    TupleType, Type, TypeId, TypeRegistry,
};

struct TypeAnalysis {
    file: TestFile,
    binder: Binder,
    types: TypeRegistry,
    diagnostics: DiagnosticCollection,
}

/// Builds and runs every semantic pass over an arbitrary Solidity `source`,
/// returning the analysis, including any diagnostics produced.
fn analyze_with_diagnostics(language_version: LanguageVersion, source: &str) -> TypeAnalysis {
    let mut id_generator = NodeIdGenerator::default();
    let file = build_file(
        "test.sol".into(),
        source,
        &mut id_generator,
        language_version,
    );
    let files = vec![file];

    let mut binder = Binder::default();
    let mut types = TypeRegistry::new(language_version);
    let mut diagnostics = DiagnosticCollection::default();
    let file_node_mapper = FileNodeMapper::build_from(&files);
    p1_collect_definitions::run(&files, &mut binder, &mut diagnostics);
    p2_linearise_contracts::run(&files, &mut binder, &mut diagnostics);
    p3_type_definitions::run(
        &files,
        &mut binder,
        &mut types,
        &file_node_mapper,
        &mut diagnostics,
    );
    p5_resolve_references::run(
        &files,
        &mut binder,
        &mut types,
        &file_node_mapper,
        &mut diagnostics,
    );

    TypeAnalysis {
        file: files.into_iter().next().unwrap(),
        binder,
        types,
        diagnostics,
    }
}

/// Builds and runs every semantic pass over an arbitrary Solidity `source`,
/// asserting that no diagnostics were produced.
fn analyze(language_version: LanguageVersion, source: &str) -> TypeAnalysis {
    let analysis = analyze_with_diagnostics(language_version, source);
    assert!(
        analysis.diagnostics.is_empty(),
        "Semantic diagnostics: {:?}",
        analysis.diagnostics
    );
    analysis
}

/// Recovers the typing recorded for an expression `node`, resolved to a
/// concrete [`Type`].
fn recover_expression_type(
    node: &ir::Expression,
    binder: &Binder,
    types: &TypeRegistry,
) -> Option<Type> {
    let node_id = node_id_for_expression_typing(node)?;
    binder
        .node_typing(node_id)
        .as_type_id()
        .map(|type_id| types.get_type_by_id(type_id).clone())
}

/// Finds the function named `name` among a contract's or library's `members`.
fn find_function<'a>(
    members: &'a [ir::ContractMember],
    name: &str,
) -> Option<&'a ir::FunctionDefinition> {
    members.iter().find_map(|member| match member {
        ir::ContractMember::FunctionDefinition(function)
            if function.name.as_ref().is_some_and(|n| n.unparse() == name) =>
        {
            Some(function)
        }
        _ => None,
    })
}

/// Finds the top-level contract named `name`.
fn find_contract<'a>(file: &'a TestFile, name: &str) -> &'a ir::ContractDefinition {
    file.ir_root()
        .members
        .iter()
        .find_map(|member| match member {
            ir::SourceUnitMember::ContractDefinition(c) if c.name.unparse() == name => Some(c),
            _ => None,
        })
        .unwrap_or_else(|| panic!("contract `{name}` not found"))
}

/// Finds the top-level library named `name`.
fn find_library<'a>(file: &'a TestFile, name: &str) -> &'a ir::LibraryDefinition {
    file.ir_root()
        .members
        .iter()
        .find_map(|member| match member {
            ir::SourceUnitMember::LibraryDefinition(l) if l.name.unparse() == name => Some(l),
            _ => None,
        })
        .unwrap_or_else(|| panic!("library `{name}` not found"))
}

/// Collects the recovered type of each expression statement in `body`, in
/// source order.
fn expression_statement_types(
    body: &ir::Block,
    binder: &Binder,
    types: &TypeRegistry,
) -> Vec<Option<Type>> {
    body.statements
        .iter()
        .filter_map(|stmt| match stmt {
            ir::Statement::ExpressionStatement(s) => {
                Some(recover_expression_type(&s.expression, binder, types))
            }
            _ => None,
        })
        .collect()
}

/// Wraps each expression in a no-op expression statement inside the body of an
/// `__test()` function of a synthesized `Test` contract, runs every semantic
/// pass, and returns the typing recorded for each expression (in input order)
/// along with the populated type registry. Non-`Resolved` typings come back
/// as `None`.
///
/// `contract_context` is optional contract-level setup — state variables,
/// nested struct definitions, sibling member functions, etc. — inserted
/// before the `__test()` definition.
fn type_of_expressions(
    language_version: LanguageVersion,
    contract_name: Option<&str>,
    contract_context: Option<&str>,
    expressions: &[&str],
) -> (Vec<Option<Type>>, TypeRegistry) {
    let context_block = contract_context.unwrap_or("");
    let contract_name = contract_name.unwrap_or("Test");
    let expression_statements = expressions
        .iter()
        .map(|expr| format!("{expr};"))
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!(
        r#"
        contract {contract_name} {{
            {context_block}
            function __test() internal {{
                {expression_statements}
            }}
        }}
        "#
    );

    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(language_version, &source);

    let contract = find_contract(&file, contract_name);
    let function = find_function(&contract.members, "__test").expect("__test function not found");
    let block = function.body.as_ref().expect("__test has a body");

    let typings = expression_statement_types(block, &binder, &types);

    (typings, types)
}

/// Convenience wrapper for `type_of_expressions` with a single expression and
/// no contract context. Panics if the typing didn't resolve.
fn type_of_expression(expr: &str) -> (Type, TypeRegistry) {
    let (expr_type, types) = try_type_of_expression(expr);
    (
        expr_type.expect("expected resolved type for expression"),
        types,
    )
}

/// Convenience wrapper for `type_of_expressions` with a single expression and
/// no contract context. Returns `None` if the typing didn't resolve.
fn try_type_of_expression(expr: &str) -> (Option<Type>, TypeRegistry) {
    let (typings, types) = type_of_expressions(LanguageVersion::LATEST, None, None, &[expr]);
    let typing = typings.into_iter().next().expect("at least one expression");
    (typing, types)
}

/// Like `type_of_expression`, but with contract-level setup (state variables,
/// member functions, …) inserted before the `__test()` function.
fn type_of_expression_in_context(context: &str, expr: &str) -> (Type, TypeRegistry) {
    let (expr_type, types) = try_type_of_expression_in_context(context, expr);
    (
        expr_type.expect("expected resolved type for expression"),
        types,
    )
}

fn try_type_of_expression_in_context(context: &str, expr: &str) -> (Option<Type>, TypeRegistry) {
    let (typings, types) =
        type_of_expressions(LanguageVersion::LATEST, None, Some(context), &[expr]);
    let typing = typings.into_iter().next().expect("at least one expression");
    (typing, types)
}

fn register_uint_type(types: &mut TypeRegistry, bits: u32) -> TypeId {
    types.register_type(Type::Integer(IntegerType {
        is_signed: false,
        bits,
    }))
}

/// Returns the kind of the single emitted diagnostic, if any. The inputs in
/// these tests produce at most one diagnostic.
fn diagnostic_kind(diagnostics: &DiagnosticCollection) -> Option<DiagnosticKind> {
    assert!(
        diagnostics.iter().count() <= 1,
        "expected at most one diagnostic: {diagnostics:?}"
    );
    diagnostics.iter().next().map(|d| d.kind().clone())
}

/// Runs the full pipeline over `source` and returns contract `name`'s folded
/// storage base slot together with the diagnostic emitted, if any. A rejected base
/// slot is reported as a diagnostic and leaves `base_slot` unset.
fn contract_base_slot(source: &str, name: &str) -> (Option<U256>, Option<DiagnosticKind>) {
    let TypeAnalysis {
        file,
        binder,
        diagnostics,
        ..
    } = analyze_with_diagnostics(LanguageVersion::LATEST, source);
    let contract = find_contract(&file, name);
    let base_slot = match binder
        .find_definition_by_id(contract.id())
        .expect("contract definition is registered")
    {
        Definition::Contract(contract_definition) => contract_definition.base_slot,
        _ => panic!("expected a contract definition"),
    };
    (base_slot, diagnostic_kind(&diagnostics))
}

/// Folds a fixed-size-array length through the real pipeline, returning the
/// computed `FixedSizeArrayType.size` together with the diagnostic emitted, if any.
/// `context` holds any contract-level constants the length references;
/// `array_type` is the variable type (e.g. `"uint256[10 / B]"`). A rejected
/// length reads back as `0`, same as a length that genuinely folds to `0`.
fn folded_array_length(context: &str, array_type: &str) -> (U256, Option<DiagnosticKind>) {
    let source = format!(
        r#"
        contract Test {{
            {context}
            {array_type} sized_array;
        }}
        "#
    );

    let TypeAnalysis {
        file,
        binder,
        types,
        diagnostics,
    } = analyze_with_diagnostics(LanguageVersion::LATEST, &source);

    let contract = find_contract(&file, "Test");
    let state_variable = contract
        .members
        .iter()
        .find_map(|member| match member {
            ir::ContractMember::StateVariableDefinition(state_variable)
                if state_variable.name.unparse() == "sized_array" =>
            {
                Some(state_variable)
            }
            _ => None,
        })
        .expect("`sized_array` state variable not found");

    let type_id = binder
        .node_typing(state_variable.id())
        .as_type_id()
        .expect("state variable has a resolved type");
    let size = match types.get_type_by_id(type_id) {
        Type::FixedSizeArray(FixedSizeArrayType { size, .. }) => *size,
        other => panic!("expected a FixedSizeArray type, got {other:?}"),
    };
    (size, diagnostic_kind(&diagnostics))
}

#[test]
fn test_value_bearing_integer_literal_types() {
    let (type_, _) = type_of_expression("127");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(127)
        })
    );

    let (type_, _) = type_of_expression("-128");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-128)
        })
    );

    // Hex literals carry source byte width as `HexInteger`, distinct from
    // decimal `Integer` so the byte-array conversion rule can fire.
    let (type_, _) = type_of_expression("0xff");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(255u32),
            bytes: 1,
        })
    );

    // Source byte width is preserved across leading zeros.
    let (type_, _) = type_of_expression("0x0012");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(18u32),
            bytes: 2,
        })
    );

    // Folding a hex literal demotes it to a plain `Integer` (provenance lost).
    let (type_, _) = type_of_expression("0x10 + 0");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(16)
        })
    );
}

#[test]
fn test_binary_arithmetic_folds_to_narrowed_literal() {
    // Addition.
    let (type_, _) = type_of_expression("1 + 1");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(2)
        })
    );

    // Multiplication.
    let (type_, _) = type_of_expression("3 * 4");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(12)
        })
    );

    // Power.
    let (type_, _) = type_of_expression("2 ** 10");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1024)
        })
    );

    // Shift.
    let (type_, _) = type_of_expression("1 << 32");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1u64 << 32)
        })
    );

    // Reducible rational arithmetic normalises back to an integer.
    let (type_, _) = type_of_expression("1.5 * 2");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // Non-reducing rational division stays rational.
    let (type_, _) = type_of_expression("5 / 2");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Rational {
            value: BigRational::new(BigInt::from(5), BigInt::from(2))
        })
    );

    // Negation of a folded constant.
    let (type_, _) = type_of_expression("-(1 + 1)");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );
}

#[test]
fn test_binary_bitwise_folds_to_literal() {
    // OR
    let (type_, _) = type_of_expression("1 | 2");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // AND
    let (type_, _) = type_of_expression("12 & 10");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(8)
        })
    );

    // XOR
    let (type_, _) = type_of_expression("6 ^ 3");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(5)
        })
    );

    // Folding hex operands demotes the result to a plain `Integer`
    // (mirroring the additive folding behaviour).
    let (type_, _) = type_of_expression("0xf0 | 0x0f");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise AND with a negative literal: BigInt uses arbitrary-precision
    // two's-complement, so `-1 & 0xff` masks to the low byte.
    let (type_, _) = type_of_expression("(-1) & 0xff");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise OR of a folded constant feeds further folding.
    let (type_, _) = type_of_expression("(1 | 2) ^ 4");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(7)
        })
    );
}

#[test]
fn test_bitwise_not_folds_to_literal() {
    // ~x = -x - 1 (two's complement on an infinite-precision integer).
    let (type_, _) = type_of_expression("~1");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );

    let (type_, _) = type_of_expression("~0");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-1)
        })
    );

    // Double-complement returns to the original value.
    let (type_, _) = type_of_expression("~(-1)");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0)
        })
    );

    // Folding `~hex` demotes the result to a plain `Integer`.
    let (type_, _) = type_of_expression("~0xff");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-256)
        })
    );

    // `~` of a folded constant.
    let (type_, _) = type_of_expression("~(1 | 2)");
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-4)
        })
    );
}

#[test]
fn test_bitwise_operations_unresolved_for_rationals() {
    // Bitwise binary operators don't apply to non-reducing rationals.
    let (type_, _) = try_type_of_expression("1.5 | 1");
    assert_eq!(type_, None);

    let (type_, _) = try_type_of_expression("1 & 0.5");
    assert_eq!(type_, None);

    let (type_, _) = try_type_of_expression("0.5 ^ 0.25");
    assert_eq!(type_, None);

    // Likewise for the unary bitwise NOT.
    let (type_, _) = try_type_of_expression("~0.5");
    assert_eq!(type_, None);
}

#[test]
fn test_implicit_conversion_uses_literal_value() {
    let (_, mut types) = type_of_expression("0");

    let int8 = types.register_type(Type::Integer(IntegerType {
        is_signed: true,
        bits: 8,
    }));
    let uint8 = types.uint8();
    let uint256 = types.uint256();

    let lit_127 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(127),
    }));
    let lit_128 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(128),
    }));
    let lit_neg_128 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-128),
    }));
    let lit_neg_129 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-129),
    }));
    let lit_neg_1 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(-1),
    }));
    let lit_0 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(0),
    }));
    let lit_255 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(255),
    }));
    let lit_256 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(256),
    }));
    let lit_big = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(12_345_678),
    }));
    let lit_half = types.register_type(Type::Literal(LiteralKind::Rational {
        value: BigRational::new(BigInt::from(1), BigInt::from(2)),
    }));

    // Within int8 range (signed):
    assert!(types.implicitly_convertible_to(lit_127, int8));
    assert!(types.implicitly_convertible_to(lit_neg_128, int8));

    // Outside int8 range:
    assert!(!types.implicitly_convertible_to(lit_128, int8));
    assert!(!types.implicitly_convertible_to(lit_neg_129, int8));

    // Within uint8 range (unsigned):
    assert!(types.implicitly_convertible_to(lit_0, uint8));
    assert!(types.implicitly_convertible_to(lit_255, uint8));

    // Outside uint8 range:
    assert!(!types.implicitly_convertible_to(lit_256, uint8));
    assert!(!types.implicitly_convertible_to(lit_neg_1, uint8));

    // Anything within uint256 (any non-negative literal we'd use here):
    assert!(types.implicitly_convertible_to(lit_0, uint256));
    assert!(types.implicitly_convertible_to(lit_big, uint256));

    // Non-integer rationals never convert to integers.
    assert!(!types.implicitly_convertible_to(lit_half, uint8));
    assert!(!types.implicitly_convertible_to(lit_half, int8));
    assert!(!types.implicitly_convertible_to(lit_half, uint256));
}

#[test]
fn test_overload_resolution_unsigned_to_signed_argument_is_version_gated() {
    // End-to-end: an overload taking `int16` is only reachable from a `uint8`
    // argument before 0.8.1, where `uint8` still implicitly converts to `int16`.
    let setup = "
        uint8 u;
        function pick(int16 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";

    // 0.8.0: `uint8` -> `int16` is allowed, so the `int16` overload matches.
    let (typings, _) =
        type_of_expressions(LanguageVersion::V0_8_0, None, Some(setup), &["pick(u)"]);
    assert_eq!(
        typings.into_iter().next().unwrap(),
        Some(Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })),
    );

    // 0.8.1: `uint8` -> `int16` is rejected, so neither overload matches.
    let (typings, _) =
        type_of_expressions(LanguageVersion::V0_8_1, None, Some(setup), &["pick(u)"]);
    assert_eq!(typings.into_iter().next().unwrap(), None);
}

#[test]
fn test_hex_literal_to_byte_array_conversion() {
    let (_, mut types) = type_of_expression("0");

    let bytes1 = types.register_type(Type::ByteArray(ByteArrayType { width: 1 }));
    let bytes2 = types.register_type(Type::ByteArray(ByteArrayType { width: 2 }));
    let bytes4 = types.register_type(Type::ByteArray(ByteArrayType { width: 4 }));

    // Hex source literal: byte-width must match the target exactly.
    let hex_0x12 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x12u32),
        bytes: 1,
    }));
    let hex_0x0012 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x12u32),
        bytes: 2,
    }));
    let hex_0x10203040 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0x1020_3040u32),
        bytes: 4,
    }));

    // Same value, different source widths convert to different byte arrays.
    assert!(types.implicitly_convertible_to(hex_0x12, bytes1));
    assert!(!types.implicitly_convertible_to(hex_0x12, bytes2));
    assert!(types.implicitly_convertible_to(hex_0x0012, bytes2));
    assert!(!types.implicitly_convertible_to(hex_0x0012, bytes1));
    assert!(types.implicitly_convertible_to(hex_0x10203040, bytes4));
    assert!(!types.implicitly_convertible_to(hex_0x10203040, bytes2));

    // Decimal-source integer of the same value does NOT convert to a byte
    // array (provenance matters).
    let dec_18 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(18),
    }));
    assert!(!types.implicitly_convertible_to(dec_18, bytes1));
    assert!(!types.implicitly_convertible_to(dec_18, bytes2));

    // Zero in any source — decimal, hex of any width, or folded — converts
    // to a byte array of any width.
    let dec_0 = types.register_type(Type::Literal(LiteralKind::Integer {
        value: BigInt::from(0),
    }));
    let hex_0x0 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0u32),
        bytes: 1,
    }));
    let hex_0x0000 = types.register_type(Type::Literal(LiteralKind::HexInteger {
        value: BigUint::from(0u32),
        bytes: 2,
    }));
    assert!(types.implicitly_convertible_to(dec_0, bytes1));
    assert!(types.implicitly_convertible_to(dec_0, bytes4));
    assert!(types.implicitly_convertible_to(hex_0x0, bytes2));
    assert!(types.implicitly_convertible_to(hex_0x0000, bytes4));
}

#[test]
fn test_conditional_expression_unifies_branch_types() {
    // Both branches reify to uint8 — common type is uint8.
    let (type_, _) = type_of_expression("true ? 1 : 2");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })
    );

    // uint8 (1) widens to uint16 (256).
    let (type_, _) = type_of_expression("true ? 1 : 256");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 16,
        })
    );

    // int8 (-1) and int8 (1) — common type is int8.
    let (type_, _) = type_of_expression("true ? -1 : -128");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: true,
            bits: 8,
        })
    );

    // Both branches are string literals — both reify to `string memory`.
    let (type_, _) = type_of_expression(r#"true ? "abc" : "x""#);
    assert_eq!(
        type_,
        Type::String(StringType {
            location: DataLocation::Memory,
        })
    );
}

#[test]
fn test_conditional_expression_unresolved_when_branches_incompatible() {
    // uint8 (1) and int8 (-1): neither converts to the other at the same
    // bit width, so unification fails and the conditional is unresolved.
    let (type_, _) = try_type_of_expression("true ? 1 : -1");
    assert_eq!(type_, None);

    // A non-reducing rational has no `reified` type yet, so any conditional
    // involving one is unresolved.
    let (type_, _) = try_type_of_expression("true ? 0.5 : 1");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_unifies_element_types() {
    // Homogeneous uint8 elements.
    let (expr_type, types) = type_of_expression("[1, 2, 3]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(3));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.uint8());

    // Mixed widths widen to the largest required.
    let (expr_type, mut types) = type_of_expression("[1, 256, 3]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(3));
    assert_eq!(element_type, register_uint_type(&mut types, 16));

    // Negative values force the result to a signed type.
    let (expr_type, mut types) = type_of_expression("[-1, -2]");
    let Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) = expr_type else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(
        element_type,
        types.register_type(Type::Integer(IntegerType {
            is_signed: true,
            bits: 8,
        }))
    );

    // String literal arrays reify each element to `string memory`.
    let (expr_type, types) = type_of_expression(r#"["abc", "x"]"#);
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, types.string_memory());
}

#[test]
fn test_array_literal_unresolved_when_elements_incompatible() {
    // uint8 (1) and int8 (-1) cannot be unified (same bit width, opposite sign).
    let (type_, _) = try_type_of_expression("[1, -1]");
    assert_eq!(type_, None);

    // Non-reducing rationals don't reify yet — array unification fails.
    let (type_, _) = try_type_of_expression("[0.5, 1]");
    assert_eq!(type_, None);
}

#[test]
fn test_conditional_expression_unifies_byte_arrays() {
    let (expr_type, types) = type_of_expression("true ? bytes32(0) : bytes32(1)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_conditional_expression_widens_byte_arrays() {
    let (expr_type, types) = type_of_expression("true ? bytes20(0) : bytes32(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));

    let (expr_type, types) = type_of_expression("true ? bytes32(0) : bytes20(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_array_literal_unifies_byte_array_elements() {
    let (expr_type, types) = type_of_expression("[bytes32(0), bytes32(1)]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.bytes32());
}

#[test]
fn test_array_literal_unifies_byte_array_and_literal_zero() {
    let (expr_type, types) = type_of_expression("[bytes32(0), 0]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type,
        size,
        location,
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.bytes32());
}

#[test]
fn test_conditional_expression_does_not_unify_byte_array_and_literal_zero() {
    let (type_, _) = try_type_of_expression("true ? bytes32(0) : 0");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_does_not_unify_when_literal_is_first_and_byte_array_follows() {
    // The first element of the array is used to find the common type
    // Matches solc behaviour
    let (type_, _) = try_type_of_expression("[0, bytes32(0)]");
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_widens_past_first_element_integer_type() {
    let (expr_type, mut types) = type_of_expression("[uint8(0), 256]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, register_uint_type(&mut types, 16));
}

#[test]
fn test_array_literal_unifies_byte_array_and_matching_hex_literal() {
    let (expr_type, types) = type_of_expression("[bytes1(0x01), 0x01]");
    let Type::FixedSizeArray(FixedSizeArrayType {
        element_type, size, ..
    }) = expr_type
    else {
        panic!("expected FixedSizeArray, got {expr_type:?}");
    };
    assert_eq!(size, U256::from(2));
    assert_eq!(element_type, types.bytes1());
}

#[test]
fn test_conditional_expression_loses_hex_literal_specialness() {
    let (type_, _) = try_type_of_expression("true ? bytes1(0x01) : 0x01");
    assert_eq!(type_, None);
}

#[test]
fn test_conditional_expression_widens_literal_to_concrete_integer() {
    let (expr_type, types) = type_of_expression("true ? uint256(0) : 0");
    assert_eq!(expr_type, *types.get_type_by_id(types.uint256()));

    let (expr_type, types) = type_of_expression("true ? 0 : uint256(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.uint256()));
}

#[test]
fn test_conditional_expression_unifies_mappings() {
    let (expr_type, types) = try_type_of_expression_in_context(
        "mapping(uint => uint) m1; mapping(uint => uint) m2;",
        "true ? m1 : m2",
    );
    let Some(Type::Mapping(MappingType {
        key_type_id,
        value_type_id,
    })) = expr_type
    else {
        panic!("expected Mapping, got {expr_type:?}");
    };
    assert_eq!(key_type_id, types.uint256());
    assert_eq!(value_type_id, types.uint256());
}

#[test]
fn test_conditional_expression_unifies_literal_tuples() {
    let (expr_type, types) = type_of_expression("true ? (1, 2) : (3, 4)");
    let Type::Tuple(TupleType { types: tuple_types }) = expr_type else {
        panic!("expected Tuple, got {expr_type:?}");
    };

    assert_eq!(tuple_types.len(), 2);
    assert_eq!(tuple_types[0], types.uint8());
    assert_eq!(tuple_types[1], types.uint8());
}

#[test]
fn test_mappings_only_unify_on_equal_elements() {
    // Mappings must match on key and value types
    let (expr_type, _) = try_type_of_expression_in_context(
        "mapping(uint => int128) m1; mapping(uint => int256) m2;",
        "true ? m1 : m2",
    );
    assert_eq!(None, expr_type);
}

#[test]
fn test_array_literal_rejects_mapping_element() {
    let (type_, _) = try_type_of_expression_in_context(
        "mapping(uint => uint) m1; mapping(uint => uint) m2;",
        "[m1, m2]",
    );
    assert_eq!(type_, None);
}

#[test]
fn test_array_literal_does_not_unify_byte_array_and_non_zero_literal() {
    let (type_, _) = try_type_of_expression("[bytes32(0), 1]");
    assert_eq!(type_, None);
}

#[test]
fn test_bitwise_or_widens_byte_arrays() {
    let (expr_type, types) = type_of_expression("bytes20(0) | bytes32(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));

    let (expr_type, types) = type_of_expression("bytes32(0) | bytes20(0)");
    assert_eq!(expr_type, *types.get_type_by_id(types.bytes32()));
}

#[test]
fn test_overload_resolution_widens_byte_array_argument() {
    let setup = "
        function pick(bytes32 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";
    let (type_, _) = type_of_expression_in_context(setup, "pick(bytes20(0))");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })
    );
}

#[test]
fn test_overload_resolution_rejects_byte_array_narrowing() {
    let setup = "
        function pick(bytes20 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";
    let (type_, _) = try_type_of_expression_in_context(setup, "pick(bytes32(0))");
    // Neither overload matches: `bytes32` does not convert to `bytes20` nor
    // to `string`. The call is unresolved.
    assert_eq!(type_, None);
}

#[test]
fn test_conditional_expression_unifies_booleans() {
    let (type_, _) = type_of_expression("true ? true : false");
    assert_eq!(type_, Type::Boolean);
}

#[test]
fn test_string_literal_byte_count_with_escapes() {
    // Plain ASCII: one byte per char.
    let (type_, _) = type_of_expression(r#""abc""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // Each `\n`, `\t`, etc. decodes to a single byte.
    let (type_, _) = type_of_expression(r#""\n\t\\""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // `\xNN` escapes decode to one byte each, regardless of the 4-char source
    // length per escape.
    let (type_, _) = type_of_expression(r#""\x41\x42""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 2 }));

    // Line continuations (`\<newline>`) decode to nothing.
    let (type_, _) = type_of_expression("\"a\\\nb\"");
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 2 }));

    // Concatenated string literals: byte counts add up across pieces.
    let (type_, _) = type_of_expression(r#""abc" "de""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 5 }));
}

#[test]
fn test_unicode_string_literal_byte_count() {
    // ASCII unicode-string literal: one byte per char.
    let (type_, _) = type_of_expression(r#"unicode"abc""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // Multi-byte UTF-8 passes through with its full byte length:
    // `€` is 3 bytes in UTF-8.
    let (type_, _) = type_of_expression(r#"unicode"€""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 3 }));

    // `\uNNNN` escapes decode to their UTF-8 byte length:
    // `\u20AC` (€) → 3 bytes, `\u00A2` (¢) → 2 bytes, `\u0024` ($) → 1 byte.
    let (type_, _) = type_of_expression(r#"unicode"\u20AC\u00A2\u0024""#);
    assert_eq!(type_, Type::Literal(LiteralKind::String { bytes: 6 }));
}

#[test]
fn test_hex_string_literal_byte_count() {
    // Pairs of hex digits, no separators: one byte per pair.
    let (type_, _) = type_of_expression(r#"hex"414243""#);
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 3 }));

    // Underscore separators don't contribute to the decoded length.
    let (type_, _) = type_of_expression(r#"hex"41_42""#);
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 2 }));

    // Concatenated hex string literals: byte counts add up across pieces.
    let (type_, _) = type_of_expression(r#"hex"4142" hex"43""#);
    assert_eq!(type_, Type::Literal(LiteralKind::HexString { bytes: 3 }));
}

#[test]
fn test_data_locations_of_state_variable_and_getter_accesses() {
    // In source order:
    //  - `bs` — internal access to a `bytes` storage variable: `bytes storage`.
    //  - `foo.xs` — `xs` is declared with `Inherited` location inside the
    //    struct; the member access propagates the operand's storage location.
    //  - `t.bs()` — external call to the auto-generated getter of `bytes bs`;
    //    the returned reference type lives in memory.
    //  - `t.foo()` — external call to the auto-generated getter of `Foo foo`.
    //    `Foo` has a single returnable field (`bytes xs`), so the getter
    //    returns just `bytes`, again in memory.
    let (typings, _) = type_of_expressions(
        LanguageVersion::LATEST,
        None,
        Some(
            r#"
            struct Foo { bytes xs; }
            bytes public bs;
            Foo public foo;
            Test t;
            "#,
        ),
        &["bs", "foo.xs", "t.bs()", "t.foo()"],
    );
    let expected = vec![
        Some(Type::Bytes(BytesType {
            location: DataLocation::Storage,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Storage,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Memory,
        })),
        Some(Type::Bytes(BytesType {
            location: DataLocation::Memory,
        })),
    ];
    assert_eq!(typings, expected);
}

#[test]
fn test_cast_address_to_library_is_library_typed() {
    // Casting an address to a library (`MyLib(x)`) is valid Solidity and
    // yields a value of the library type, which can then be compared against
    // another library value.
    let source = r#"
        library MyLib {
            function f() public pure returns (uint) { return 1; }
        }
        contract Test {
            function probe(address x, address y) internal pure {
                MyLib(x);
                MyLib(x) == MyLib(y);
            }
        }
    "#;
    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);

    let contract = find_contract(&file, "Test");
    let probe = find_function(&contract.members, "probe").expect("probe function");
    let body = probe.body.as_ref().expect("probe has a body");

    let typings = expression_statement_types(body, &binder, &types);
    let [cast, comparison] = typings.as_slice() else {
        panic!("expected two expression statements, got {typings:?}");
    };

    assert!(
        matches!(cast, Some(Type::Library { .. })),
        "expected `MyLib(x)` to be typed as the library, got {cast:?}",
    );
    assert_eq!(
        comparison,
        &Some(Type::Boolean),
        "expected `MyLib(x) == MyLib(y)` to be a boolean",
    );
}

#[test]
fn test_getter_of_struct_with_function_member() {
    // The auto-generated getter of a public struct state variable returns a
    // tuple of its value-type fields.
    let (getter_type, types) = type_of_expression_in_context(
        r#"
        struct S { uint a; function() external fn; }
        S public s;
        Test other;
        "#,
        "other.s()",
    );

    let Type::Tuple(TupleType { types: elements }) = getter_type else {
        panic!("expected the getter to return a tuple, got {getter_type:?}");
    };
    let element_types: Vec<&Type> = elements
        .iter()
        .map(|type_id| types.get_type_by_id(*type_id))
        .collect();

    assert!(
        matches!(
            element_types.as_slice(),
            [
                Type::Integer(IntegerType {
                    is_signed: false,
                    bits: 256
                }),
                Type::Function(_),
            ]
        ),
        "expected getter tuple `(uint256, function() external)`, got {element_types:?}",
    );
}

#[test]
fn test_getter_of_struct_with_struct_member() {
    // The auto-generated getter of a public struct state variable returns a
    // tuple of its value-type fields.
    let (getter_type, types) = type_of_expression_in_context(
        r#"
        struct P { bool a; }
        struct S { P p; uint a; }
        S public s;
        Test other;
        "#,
        "other.s()",
    );

    let Type::Tuple(TupleType { types: elements }) = getter_type else {
        panic!("expected the getter to return a tuple, got {getter_type:?}");
    };
    let element_types: Vec<&Type> = elements
        .iter()
        .map(|type_id| types.get_type_by_id(*type_id))
        .collect();

    assert!(
        matches!(
            element_types.as_slice(),
            [
                Type::Struct(_),
                Type::Integer(IntegerType {
                    is_signed: false,
                    bits: 256
                }),
            ]
        ),
        "expected getter tuple `(Struct, uint256)`, got {element_types:?}",
    );
}

#[test]
fn test_this_in_library_is_library_typed() {
    // `this` inside a library function is valid Solidity and has the library
    // type
    let source = r#"
        library MyLib {
            function probe() internal view {
                this;
            }
        }
        contract Test {}
        "#;

    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);

    let library = find_library(&file, "MyLib");
    let probe = find_function(&library.members, "probe").expect("probe function");
    let body = probe.body.as_ref().expect("probe has a body");

    let typings = expression_statement_types(body, &binder, &types);
    assert!(
        matches!(typings.as_slice(), [Some(Type::Library(LibraryType { definition_id }))] if definition_id == &library.id()),
        "expected `this` to be typed as the library, got {typings:?}",
    );
}

#[test]
fn test_this_inside_contract() {
    let source = r#"
        contract MyContract {
            function probe() internal view {
                this;
            }
        }
        contract Test {}
        "#;

    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);

    let contract = find_contract(&file, "MyContract");
    let probe = find_function(&contract.members, "probe").expect("probe function");
    let body = probe.body.as_ref().expect("probe has a body");

    let typings = expression_statement_types(body, &binder, &types);

    assert!(
        matches!(typings.as_slice(), [Some(Type::Contract(ContractType { definition_id }))] if definition_id == &contract.id())
    );
}

#[test]
fn test_partially_applied_function_does_not_unify_into_array() {
    // `L.inc` is attached to `uint` via `using for`, so `t.inc` binds the
    // receiver and becomes a partially applied function with no mobile type.
    let source = r#"
        library L {
            function inc(uint x) internal pure returns (uint) { return x + 1; }
        }
        contract Test {
            using L for uint;
            function inc_method(uint x) internal pure returns (uint) { return x; }
            function foo() external {}
            function __test() internal {
                uint t = 1;
                [inc_method, inc_method];
                [inc_method, t.inc];
                [this.foo, this.foo];
                [this.foo, this.foo{ gas: 4 }];
            }
        }
        "#;

    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);

    let contract = find_contract(&file, "Test");
    let function = find_function(&contract.members, "__test").expect("__test function");
    let body = function.body.as_ref().expect("__test has a body");

    let mut typings = expression_statement_types(body, &binder, &types).into_iter();

    // Control: plain function pointers of the same signature still unify into a
    // fixed-size array.
    assert!(
        matches!(typings.next(), Some(Some(Type::FixedSizeArray(_)))),
        "plain function pointers should unify into an array",
    );

    // The bound element has no mobile type, so the array does not type.
    assert_eq!(
        typings.next(),
        Some(None::<Type>),
        "an array with a partially applied element should not type",
    );

    // Control: plain function pointers of the same signature still unify into a
    // fixed-size array.
    assert!(
        matches!(typings.next(), Some(Some(Type::FixedSizeArray(_)))),
        "plain function pointers should unify into an array",
    );

    // The bound element has no mobile type, so the array does not type.
    assert_eq!(
        typings.next(),
        Some(None::<Type>),
        "an array with a partially applied element should not type",
    );
}

// A partially applied function (bound first argument or pre-applied call
// options) is not
// implicitly convertible to its plain function pointer counterpart, even
// though they share the same signature.
#[test]
fn test_partially_applied_function_is_not_convertible() {
    let source = r#"
        library L {
            function inc(uint x) internal pure {}
        }
        contract Test {
            using L for uint;
            function foo() external {}
            
            function take_internal(function(uint) internal pure f) internal pure returns (bool) {}
            function take_internal(uint f) internal pure returns (uint) {}
            
            function take_external(function() external g) internal pure returns (bool) {}
            function take_external(uint g) internal pure returns (uint) {}
            
            function __test() internal view {
                uint t = 1;
        
                take_internal(L.inc); // <------------- works
                take_internal(t.inc); // <------------- fails
                take_external(this.foo); // <---------- works
                take_external(this.foo{gas: 4}); // <-- fails
            }
        }        
        "#;

    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);

    let contract = find_contract(&file, "Test");
    let function = find_function(&contract.members, "__test").expect("__test function");
    let body = function.body.as_ref().expect("__test has a body");

    let mut typings = expression_statement_types(body, &binder, &types).into_iter();

    assert!(
        matches!(typings.next(), Some(Some(Type::Boolean))),
        "plain library function should be convertible",
    );

    assert!(
        matches!(typings.next(), Some(None)),
        "partially applied function pointers should not be convertible",
    );

    assert!(
        matches!(typings.next(), Some(Some(Type::Boolean))),
        "plain function pointers should be convertible",
    );

    assert!(
        matches!(typings.next(), Some(None)),
        "partially applied function pointers should not be convertible",
    );
}

#[test]
fn test_index_access_on_elementary_meta_type_yields_array_meta_type() {
    // Control: indexing the meta-type of an elementary type (`uint[]`) yields
    // the meta-type of an array of that elementary type.
    let (meta, types) = type_of_expression("uint[]");

    let Type::MetaType(MetaType { type_id: array_id }) = meta else {
        panic!("expected the `uint[]` expression to be a MetaType, got {meta:?}");
    };
    let Type::Array(ArrayType {
        element_type,
        location,
    }) = types.get_type_by_id(array_id).clone()
    else {
        panic!(
            "expected the meta-type to wrap an Array, got {:?}",
            types.get_type_by_id(array_id)
        );
    };
    assert_eq!(location, DataLocation::Memory);
    assert_eq!(element_type, types.uint256());
}

#[test]
fn test_index_access_on_user_meta_type_yields_array_meta_type() {
    // `MyStruct[]` is a *type expression*: indexing the user meta-type of a
    // struct produces the meta-type of an array whose element is that struct.
    let (meta, types) = type_of_expression_in_context("struct MyStruct { uint a; }", "MyStruct[]");

    let Type::MetaType(MetaType { type_id: array_id }) = meta else {
        panic!("expected the `MyStruct[]` expression to be a MetaType, got {meta:?}");
    };
    let Type::Array(ArrayType {
        element_type,
        location,
    }) = types.get_type_by_id(array_id).clone()
    else {
        panic!(
            "expected the meta-type to wrap an Array, got {:?}",
            types.get_type_by_id(array_id)
        );
    };
    assert_eq!(location, DataLocation::Memory);

    // The array element is the struct's own value type.
    assert!(
        matches!(
            types.get_type_by_id(element_type),
            Type::Struct(StructType { .. })
        ),
        "expected the array element to be the struct type, got {:?}",
        types.get_type_by_id(element_type),
    );
}

#[test]
fn reference_type_constant_is_indexable() {
    let (element_type, _types) =
        type_of_expression_in_context(r#"bytes constant B = hex"1234";"#, "B[0]");
    assert_eq!(element_type, Type::ByteArray(ByteArrayType { width: 1 }));
}

#[test]
fn test_array_length_folds_with_typed_constants() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // Division by a typed integer constant truncates toward zero (`10 / 3 = 3`).
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[10 / B]"),
        (U256::from(3), None)
    );
    // `7 / 3` truncates to `2`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[7 / B]"),
        (U256::from(2), None)
    );
    // The fractional intermediate of `(1 / B) * B` is discarded at the typed
    // division, folding to `0` rather than `1`, which is then a zero length.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(1 / B) * B]"),
        (U256::ZERO, Some(ArrayLengthZero.into())),
    );
    // Whole literals combine with a typed integer fine: `2 * 7 / 2 = 7`.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(B * 7) / 2]"),
        (U256::from(7), None)
    );
    // Exponentiation with a typed base: `3 ** 2 = 9`.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[B ** 2]"),
        (U256::from(9), None)
    );

    // A small integer type widens the result to the literal's mobile type, so
    // `300 / B` (`B: uint8`) is `uint16` and folds to `100` instead of
    // overflowing `uint8`.
    assert_eq!(
        folded_array_length("uint8 constant B = 3;", "uint256[300 / B]"),
        (U256::from(100), None),
    );

    // But when no widening applies, an overflow of the result type is rejected:
    // `A + 255` (`A: uint8`) has common type `uint8` (255 fits `uint8`), and
    // `1 + 255 = 256` does not fit `uint8`, so it is an arithmetic overflow.
    assert_eq!(
        folded_array_length("uint8 constant A = 1;", "uint256[A + 255]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
}

#[test]
fn test_array_length_folds_all_literal_arithmetic() {
    // No typed constant: exact rational arithmetic, then the whole result feeds
    // the length. `100 / 8 * 2 = 25` (`100 / 8` is the exact `25/2`).
    assert_eq!(
        folded_array_length("", "uint256[(100 / 8) * 2]"),
        (U256::from(25), None)
    );
    assert_eq!(
        folded_array_length("", "uint256[2 ** 8]"),
        (U256::from(256), None)
    );
    // Lengths above the machine word are valid. The maximum is `2**256 - 1`.
    assert_eq!(
        folded_array_length("", "uint256[2 ** 64]"),
        (U256::from(1) << 64, None)
    );
}

#[test]
fn test_array_length_rejected_inputs_default_to_zero() {
    let uint256_b = |value: &str| format!("uint256 constant B = {value};");

    // A negative literal has no common type with an unsigned integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("2"), "uint256[(0 - 7) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const -7".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // `~B` over `uint256` folds to a negative value that overflows the
    // unsigned result type: an arithmetic error, not a plain non-constant
    // length.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[~B]"),
        (U256::ZERO, Some(ConstantArithmeticError.into())),
    );
    // Unary negation of an unsigned integer has no result type.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[-B]"),
        (U256::ZERO, Some(ArrayLengthNotConstant.into())),
    );
    // A literal exceeding 256 bits cannot meet a typed integer, so the
    // operator has no result type and is reported as incompatible.
    assert_eq!(
        folded_array_length(&uint256_b("3"), "uint256[(2 ** 256) / B]"),
        (
            U256::ZERO,
            Some(
                IncompatibleConstantOperator {
                    operator: "/".to_owned(),
                    left_type: "int_const 1157...(70 digits omitted)...9936".to_owned(),
                    right_type: "uint256".to_owned(),
                }
                .into()
            ),
        ),
    );
    // All-literal division stays an exact rational, which is not a valid
    // (integer) length.
    assert_eq!(
        folded_array_length("", "uint256[10 / 4]"),
        (U256::ZERO, Some(ArrayLengthFractional.into())),
    );
}

#[test]
fn test_storage_base_slot_evaluation() {
    // A base slot referencing a constant declared *after* the contract resolves,
    // because base slots are evaluated in `p5_resolve_references` once every
    // constant is typed.
    assert_eq!(
        contract_base_slot("contract C layout at N {} uint256 constant N = 42;", "C"),
        (Some(U256::from(42)), None),
    );
    // Backward reference and a plain literal still resolve.
    assert_eq!(
        contract_base_slot("uint256 constant N = 42; contract C layout at N {}", "C"),
        (Some(U256::from(42)), None),
    );
    assert_eq!(
        contract_base_slot("contract C layout at 7 {}", "C"),
        (Some(U256::from(7)), None),
    );
    // A non-integer constant (here `address`) is not foldable to an integer, so
    // it is rejected as a non-constant base slot, even when forward-referenced.
    assert_eq!(
        contract_base_slot(
            "contract C layout at N {} address constant N = address(0);",
            "C",
        ),
        (None, Some(StorageLayoutBaseNotConstant.into())),
    );
}

#[test]
fn test_event_selector() {
    // `.selector` on an event name types as `bytes4`.
    let (type_, _) = type_of_expression_in_context("event E(uint a);", "E.selector");
    assert_eq!(type_, Type::ByteArray(ByteArrayType { width: 4 }));

    // With *overloaded* events the name is ambiguous; we currently resolve the
    // member against the first candidate (both candidates expose `selector`,
    // so the typing is still `bytes4`). solc reports an ambiguity error here —
    // that diagnostic is part of the SDR[37] validation backlog.
    let (type_, _) =
        type_of_expression_in_context("event E(uint a); event E(bool b);", "E.selector");
    assert_eq!(type_, Type::ByteArray(ByteArrayType { width: 4 }));
}

#[test]
fn test_bytes_and_string_concat_typing() {
    // `concat` resolves as a member of the *meta-type* of `bytes`/`string`,
    // and the two built-ins stay distinct: `bytes.concat` yields
    // `bytes memory` while `string.concat` yields `string memory`.
    let (type_, _) = type_of_expression(r#"bytes.concat(hex"01", hex"02")"#);
    assert_eq!(
        type_,
        Type::Bytes(BytesType {
            location: DataLocation::Memory
        })
    );

    let (type_, _) = type_of_expression(r#"string.concat("a", "b")"#);
    assert_eq!(
        type_,
        Type::String(StringType {
            location: DataLocation::Memory
        })
    );
}

#[test]
fn test_static_library_call_is_not_partially_applied() {
    // With a matching `using` directive in scope, a *static* call through the
    // library name must still resolve to the full function: the type name `L`
    // is not a value receiver, so it must not bind the first parameter as a
    // partial application.
    let source = r#"
        library L {
            function f(uint x) internal pure returns (bool) { return x > 0; }
        }
        contract Test {
            using L for uint;
            function __test() internal pure {
                L.f(1);
            }
        }
        "#;
    let TypeAnalysis {
        file,
        binder,
        types,
        ..
    } = analyze(LanguageVersion::LATEST, source);
    let contract = find_contract(&file, "Test");
    let function = find_function(&contract.members, "__test").expect("__test function");
    let body = function.body.as_ref().expect("__test has a body");
    let typings = expression_statement_types(body, &binder, &types);
    assert_eq!(typings, vec![Some(Type::Boolean)]);
}

#[test]
fn test_meta_type_argument_does_not_match_overloads() {
    // Passing a type name as an argument must not match any overload
    // candidate during disambiguation.
    let context = r#"
        function f(uint x) internal pure returns (bool) { return x > 0; }
        function f(bool x) internal pure returns (uint) { return x ? 1 : 0; }
    "#;
    let (type_, _) = try_type_of_expression_in_context(context, "f(uint)");
    assert_eq!(type_, None);
}

#[test]
fn test_user_meta_type_built_in_members() {
    // Built-in members of a *type name* resolve through its meta-type: errors
    // expose `selector`, and UDVTs expose `wrap`/`unwrap`.
    let (type_, _) = type_of_expression_in_context("error Err(uint x);", "Err.selector");
    assert_eq!(type_, Type::ByteArray(ByteArrayType { width: 4 }));

    let (type_, _) = type_of_expression_in_context("type T is uint256;", "T.wrap(1)");
    assert!(
        matches!(type_, Type::UserDefinedValue(_)),
        "expected `T.wrap(1)` to type as the UDVT, got {type_:?}",
    );

    let (type_, _) = type_of_expression_in_context("type T is uint256;", "T.unwrap(T.wrap(1))");
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 256
        })
    );
}

#[test]
fn test_explicit_enum_cast() {
    // Explicit conversion from an integer to an enum is valid Solidity and
    // types as the enum.
    let (type_, _) = type_of_expression_in_context("enum E { A, B }", "E(1)");
    assert!(
        matches!(type_, Type::Enum(_)),
        "expected `E(1)` to type as the enum, got {type_:?}",
    );

    // User defined value types are not castable by name: conversion goes
    // through `wrap`/`unwrap`.
    let (type_, _) = try_type_of_expression_in_context("type T is uint256;", "T(1)");
    assert_eq!(type_, None);
}

#[test]
fn test_meta_type_internal_names() {
    // Meta-types print in solc's `type(T)` notation: `type(uint256)` for an
    // elementary type, `type(E)` for a named definition.
    let mut id_generator = NodeIdGenerator::default();
    let source = r#"
        contract C {
            enum E { A }
            function g() internal pure {
                uint(1);
                E;
            }
        }
    "#;
    let file = build_file(
        "test.sol".into(),
        source,
        &mut id_generator,
        LanguageVersion::LATEST,
    );
    let files = vec![file];
    let mut diagnostics = DiagnosticCollection::default();
    let context = SemanticContext::build_from(
        LanguageVersion::LATEST,
        EvmTarget::LATEST,
        &files,
        None,
        &mut diagnostics,
    );
    assert!(diagnostics.is_empty(), "{diagnostics:?}");

    let contract = find_contract(&files[0], "C");
    let function = find_function(&contract.members, "g").expect("g function");
    let body = function.body.as_ref().expect("g has a body");
    let mut expressions = body.statements.iter().filter_map(|stmt| match stmt {
        ir::Statement::ExpressionStatement(s) => Some(&s.expression),
        _ => None,
    });

    // `uint(1)`: the call operand `uint` carries the elementary meta-type.
    let cast = expressions.next().expect("cast statement");
    let ir::Expression::FunctionCallExpression(call) = cast else {
        panic!("expected a function call expression");
    };
    let operand_node_id =
        node_id_for_expression_typing(&call.operand).expect("operand has a typing node");
    let uint_meta_id = context
        .binder()
        .node_typing(operand_node_id)
        .as_type_id()
        .expect("cast operand is typed");
    assert_eq!(context.type_internal_name(uint_meta_id), "type(uint256)");

    // `E`: the bare enum name carries the user meta-type.
    let enum_expression = expressions.next().expect("enum statement");
    let enum_node_id =
        node_id_for_expression_typing(enum_expression).expect("expression has a typing node");
    let enum_meta_id = context
        .binder()
        .node_typing(enum_node_id)
        .as_type_id()
        .expect("enum name is typed");
    assert_eq!(context.type_internal_name(enum_meta_id), "type(E)");
}

#[test]
fn test_abi_decode_tuple_of_types() {
    // Multi-element `abi.decode` types as the tuple of the *decoded* value
    // types, unwrapping each element's meta-type.
    let (decoded, types) =
        type_of_expression_in_context("bytes b; struct S { uint a; }", "abi.decode(b, (uint, S))");
    let Type::Tuple(TupleType { types: element_ids }) = decoded else {
        panic!("expected a tuple type, got {decoded:?}");
    };
    assert_eq!(element_ids.len(), 2);
    assert_eq!(element_ids[0], types.uint256());
    assert!(
        matches!(types.get_type_by_id(element_ids[1]), Type::Struct(_)),
        "expected the second element to decode to the struct",
    );

    // A tuple element that is not a type name doesn't decode.
    let (decoded, _) = try_type_of_expression_in_context("bytes b;", "abi.decode(b, (uint, 5))");
    assert_eq!(decoded, None);

    // A nested tuple element is not a type name, so it doesn't decode either
    // (matching solc, which rejects `abi.decode(b, (uint, (bool, bool)))`).
    let (decoded, _) =
        try_type_of_expression_in_context("bytes b;", "abi.decode(b, (uint, (bool, bool)))");
    assert_eq!(decoded, None);

    // Neither does a second argument that is not a type or tuple of types.
    let (decoded, _) = try_type_of_expression_in_context("bytes b; uint x;", "abi.decode(b, x)");
    assert_eq!(decoded, None);
}

#[test]
fn test_tuple_of_type_names_is_a_tuple_of_meta_types() {
    // A tuple of type names is a *tuple of meta-types* (not a meta-type
    // itself): `(uint, bool)` types as `Tuple(type(uint256), type(bool))`.
    // This matches solc, which rejects a nested tuple element (it is not a
    // type name) — see `test_abi_decode_tuple_of_types`.
    let (type_, registry) = type_of_expression("(uint, bool)");
    let Type::Tuple(TupleType { types: element_ids }) = type_ else {
        panic!("expected `(uint, bool)` to type as a tuple, got {type_:?}");
    };
    assert!(matches!(
        registry.get_type_by_id(element_ids[0]),
        Type::MetaType(_)
    ));
    assert!(matches!(
        registry.get_type_by_id(element_ids[1]),
        Type::MetaType(_)
    ));
}
