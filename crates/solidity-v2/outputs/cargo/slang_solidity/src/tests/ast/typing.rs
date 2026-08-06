use super::fixtures;
use crate::ast::visitor::{Visitor, accept_source_unit};
use crate::{ast, define_fixture};

define_fixture!(
    NewDynamic,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function f() public pure {
        bytes memory b = new bytes(35);
        string memory s = new string(4);
        b;
        s;
    }
}
"#,
);

/// Captures the resolved type of every function-call expression in a unit.
#[derive(Default)]
struct FunctionCallTypes {
    types: Vec<Option<ast::Type>>,
}

impl Visitor for FunctionCallTypes {
    fn enter_function_call_expression(&mut self, node: &ast::FunctionCallExpression) -> bool {
        self.types.push(node.get_type());
        true
    }
}

#[test]
fn test_new_bytes_and_string_get_type() {
    let unit = NewDynamic::build_compilation_unit();

    let mut finder = FunctionCallTypes::default();
    for file in unit.files() {
        accept_source_unit(&file.ast(), &mut finder);
    }

    // `new bytes(35)` and `new string(4)` type as `bytes memory` / `string
    // memory` (matching solc), not left unresolved.
    let resolved: Vec<ast::Type> = finder
        .types
        .into_iter()
        .map(|t| t.expect("each `new` call has a resolved type"))
        .collect();
    assert_eq!(resolved.len(), 2, "two `new` calls");

    let ast::Type::Bytes(bytes) = &resolved[0] else {
        panic!("`new bytes(35)` should type as bytes");
    };
    assert_eq!(
        bytes.location(),
        ast::DataLocation::Memory,
        "`new bytes(35)` is `bytes memory`"
    );

    let ast::Type::String(string) = &resolved[1] else {
        panic!("`new string(4)` should type as string");
    };
    assert_eq!(
        string.location(),
        ast::DataLocation::Memory,
        "`new string(4)` is `string memory`"
    );
}

#[test]
fn test_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .next()
        .expect("contract is found");

    let state_variables = ownable
        .members()
        .iter()
        .filter_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(state_variables.len(), 1);
    let owner = &state_variables[0];
    assert_eq!(owner.name().name(), "_owner");

    let owner_type = owner
        .get_type()
        .expect("_owner state variable has resolved type");
    assert!(matches!(owner_type, ast::Type::Address(_)));
}

#[test]
fn test_function_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract is found");

    let increment = counter
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member {
                if function_definition
                    .name()
                    .is_some_and(|name| name.name() == "increment")
                {
                    Some(function_definition)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("increment method is found");

    let increment_type = increment.get_type().expect("increment method has a type");
    let ast::Type::Function(function_type) = increment_type else {
        panic!("method's type is expected to be a function");
    };
    assert!(function_type.is_externally_visible());
    assert!(matches!(function_type.return_type(), ast::Type::Integer(_)));
    assert!(
        function_type
            .associated_definition()
            .is_some_and(|definition| matches!(definition, ast::Definition::Function(_)))
    );
}

define_fixture!(
    PublicMapping,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    mapping(string => uint256) public balances;
}
"#,
);

#[test]
fn test_public_mapping_getter_type() {
    let unit = PublicMapping::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let balances = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::StateVariableDefinition(state_variable) => Some(state_variable),
            _ => None,
        })
        .expect("state variable balances is found");

    let ast::Type::Function(getter) = balances
        .getter_type()
        .expect("a public state variable has a getter")
    else {
        panic!("expected the getter to type as a function");
    };

    assert_eq!(getter.visibility(), ast::FunctionTypeVisibility::External);

    let parameter_types = getter.parameter_types();
    let [key_type] = parameter_types.as_slice() else {
        panic!("expected the getter to take the mapping key as its only parameter");
    };
    let ast::Type::String(key) = key_type else {
        panic!("expected the key to type as a string");
    };
    assert_eq!(key.location(), ast::DataLocation::Memory);
}

define_fixture!(
    LiteralPlusInteger,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function f(uint8 i) internal pure returns (uint256) {
        return 0x1000 + i;
    }
}
"#,
);

define_fixture!(
    CalldataSlice,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function f(string calldata s, bytes calldata b, uint[] calldata u) external pure {
        s[:3];       // string slice
        b[1:2];      // bytes slice
        b[1:2][1];   // byte
        u[3:];       // uint[] slice
        u[3:][0];    // uint
    }
}
"#,
);

#[test]
fn test_literal_plus_integer_get_type() {
    let unit = LiteralPlusInteger::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function.name().is_some_and(|name| name.name() == "f") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("function f is found");

    // `f`'s only statement is `return 0x1000 + i;`.
    let body = function.body().expect("f has a body");
    let ret = body
        .statements()
        .iter()
        .find_map(|statement| match statement {
            ast::Statement::ReturnStatement(ret) => Some(ret),
            _ => None,
        })
        .expect("f has a return statement");

    let ast::Expression::AdditiveExpression(additive) =
        ret.expression().expect("return has an expression")
    else {
        panic!("expected the returned expression to be an addition");
    };

    // `0x1000 + i` adds the integer literal `4096` (mobile type `uint16`, the
    // smallest unsigned type it fits) to a `uint8`; their common type — and so
    // the type of the addition — is `uint16`.
    let ast::Type::Integer(integer) = additive
        .get_type()
        .expect("the addition has a resolved type")
    else {
        panic!("expected the addition to type as an integer");
    };
    assert!(!integer.is_signed(), "result is unsigned");
    assert_eq!(integer.bits(), 16, "result is 16-bit");
}

define_fixture!(
    BinaryOperatorCommonType,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function mul(uint8 i) internal pure returns (uint256) {
        return 0x1000 * i;
    }

    function div(uint8 i) internal pure returns (uint256) {
        return 0x1000 / i;
    }

    function rem(uint8 i) internal pure returns (uint256) {
        return 0x1000 % i;
    }

    function bit_and(uint8 i) internal pure returns (uint256) {
        return 0x1000 & i;
    }

    function bit_or(uint8 i) internal pure returns (uint256) {
        return 0x1000 | i;
    }

    function bit_xor(uint8 i) internal pure returns (uint256) {
        return 0x1000 ^ i;
    }
}
"#,
);

/// Returns the resolved type of the single `return <expr>;` statement in the
/// named function of `contract`.
fn return_expression_type(contract: &ast::ContractDefinition, function_name: &str) -> ast::Type {
    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function
                    .name()
                    .is_some_and(|name| name.name() == function_name) =>
            {
                Some(function)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("function {function_name} is found"));

    let body = function.body().expect("function has a body");
    let ret = body
        .statements()
        .iter()
        .find_map(|statement| match statement {
            ast::Statement::ReturnStatement(ret) => Some(ret),
            _ => None,
        })
        .expect("function has a return statement");

    ret.expression()
        .expect("return has an expression")
        .get_type()
        .expect("the returned expression has a resolved type")
}

#[test]
fn test_binary_operator_common_type() {
    let unit = BinaryOperatorCommonType::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    // The multiplicative (`*`, `/`, `%`) and bitwise (`&`, `|`, `^`) operators
    // share the same result typing as addition: with one non-constant operand,
    // the result is the operands' common type. Here `0x1000` has mobile type
    // `uint16` (the smallest unsigned type holding `4096`) and the other operand
    // is a `uint8`, so every one of these operators resolves to `uint16`.
    for function_name in ["mul", "div", "rem", "bit_and", "bit_or", "bit_xor"] {
        let ast::Type::Integer(integer) = return_expression_type(&contract, function_name) else {
            panic!("expected `{function_name}` to type as an integer");
        };
        assert!(!integer.is_signed(), "`{function_name}` result is unsigned");
        assert_eq!(integer.bits(), 16, "`{function_name}` result is 16-bit");
    }
}

#[test]
fn test_calldata_slice_get_type() {
    let unit = CalldataSlice::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function.name().is_some_and(|name| name.name() == "f") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("function f is found");

    // `f`'s body is five expression statements, each a slice or an index into a
    // slice, in source order.
    let types: Vec<ast::Type> = function
        .body()
        .expect("function has a body")
        .statements()
        .iter()
        .map(|statement| {
            let ast::Statement::ExpressionStatement(statement) = statement else {
                panic!("expected an expression statement");
            };
            let ast::Expression::IndexAccessExpression(index) = statement.expression() else {
                panic!("expected an index-access expression");
            };
            index
                .get_type()
                .expect("the index-access expression has a resolved type")
        })
        .collect();

    let [s_slice, b_slice, b_byte, u_slice, u_element] = types.as_slice() else {
        panic!("expected exactly five index-access expressions");
    };

    // `s[:3]` slices a `string calldata` -> a slice of that `string`.
    let ast::Type::ArraySlice(slice) = s_slice else {
        panic!("expected `s[:3]` to type as an array slice");
    };
    assert!(
        matches!(slice.array_type(), ast::Type::String(_)),
        "`s[:3]` is a slice of a `string`"
    );

    // `b[1:2]` slices a `bytes calldata` -> a slice of that `bytes`.
    let ast::Type::ArraySlice(slice) = b_slice else {
        panic!("expected `b[1:2]` to type as an array slice");
    };
    assert!(
        matches!(slice.array_type(), ast::Type::Bytes(_)),
        "`b[1:2]` is a slice of a `bytes`"
    );

    // Indexing the `bytes` slice yields a single `bytes1`.
    let ast::Type::ByteArray(byte) = b_byte else {
        panic!("expected `b[1:2][1]` to type as a byte array");
    };
    assert_eq!(byte.width(), 1, "`b[1:2][1]` is a `bytes1`");

    // `u[3:]` slices a `uint[] calldata` -> a slice of that array.
    let ast::Type::ArraySlice(slice) = u_slice else {
        panic!("expected `u[3:]` to type as an array slice");
    };
    assert!(
        matches!(slice.array_type(), ast::Type::Array(_)),
        "`u[3:]` is a slice of a `uint[]`"
    );

    // Indexing the `uint[]` slice yields a `uint256`.
    let ast::Type::Integer(integer) = u_element else {
        panic!("expected `u[3:][0]` to type as an integer");
    };
    assert!(!integer.is_signed(), "`u[3:][0]` is unsigned");
    assert_eq!(integer.bits(), 256, "`u[3:][0]` is 256-bit");
}

define_fixture!(
    OpenEndedSliceOfNonSliceable,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    mapping(uint256 => uint256) m;
    bytes32 b;

    function f() internal view {
        b[1:];   // slicing a fixed `bytes32` is not valid
        m[1:];   // slicing a mapping is not valid
    }
}
"#,
);

#[test]
fn test_open_ended_slice_of_non_sliceable_is_unresolved() {
    let unit = OpenEndedSliceOfNonSliceable::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function.name().is_some_and(|name| name.name() == "f") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("function f is found");

    // `f`'s body is two open-ended slice expressions, in source order.
    let types: Vec<Option<ast::Type>> = function
        .body()
        .expect("function has a body")
        .statements()
        .iter()
        .map(|statement| {
            let ast::Statement::ExpressionStatement(statement) = statement else {
                panic!("expected an expression statement");
            };
            let ast::Expression::IndexAccessExpression(index) = statement.expression() else {
                panic!("expected an index-access expression");
            };
            assert!(
                index.is_slice(),
                "the expression is an open-ended slice `x[1:]`"
            );
            index.get_type()
        })
        .collect();

    let [b_slice, m_slice] = types.as_slice() else {
        panic!("expected exactly two slice expressions");
    };

    assert!(
        b_slice.is_none(),
        "slicing a fixed `bytes32` does not resolve to a type"
    );
    assert!(
        m_slice.is_none(),
        "slicing a mapping does not resolve to a type"
    );
}

define_fixture!(
    ConditionalTernary,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

contract C {
    function pair() internal pure returns (uint256, uint256) {
        return (1, 2);
    }

    function f(bool c) internal pure returns (uint256, uint256) {
        (uint256 a, uint256 b) = c ? pair() : (3, uint256(4));
        return (a, b);
    }
}
"#,
);

#[test]
fn test_conditional_expression_get_type() {
    let unit = ConditionalTernary::build_compilation_unit();

    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("contract C is found");

    let function = contract
        .members()
        .iter()
        .find_map(|member| match member {
            ast::ContractMember::FunctionDefinition(function)
                if function.name().is_some_and(|name| name.name() == "f") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("function f is found");

    // `f`'s first statement is `(uint256 a, uint256 b) = c ? pair() : (3, uint256(4));`.
    let declaration = function
        .body()
        .expect("f has a body")
        .statements()
        .iter()
        .find_map(|statement| match statement {
            ast::Statement::VariableDeclarationStatement(declaration) => Some(declaration),
            _ => None,
        })
        .expect("f has a variable declaration statement");

    // Its right-hand side (the multi-variable declaration's value) is the ternary.
    let ast::VariableDeclarationTarget::MultiTypedDeclaration(target) = declaration.target() else {
        panic!("expected a multi-variable declaration target");
    };
    let ast::Expression::ConditionalExpression(conditional) = target.value() else {
        panic!("expected the initializer to be a conditional expression");
    };

    // The ternary `c ? pair() : (3, uint256(4))` unifies the function-call
    // tuple `(uint256, uint256)` with the literal tuple, widening `3`, so the
    // whole conditional types as `(uint256, uint256)`.
    let ast::Type::Tuple(tuple) = conditional.get_type().expect("ternary has a resolved type")
    else {
        panic!("expected the ternary to type as a tuple");
    };

    let elements = tuple.types();
    assert_eq!(elements.len(), 2, "tuple has two elements");
    for element in elements {
        let ast::Type::Integer(integer) = element else {
            panic!("expected each tuple element to be an integer");
        };
        assert!(!integer.is_signed(), "element is unsigned");
        assert_eq!(integer.bits(), 256, "element is 256-bit");
    }
}
