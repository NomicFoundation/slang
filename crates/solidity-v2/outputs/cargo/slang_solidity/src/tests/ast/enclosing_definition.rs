use super::fixtures;
use crate::ast::Definition;
use crate::define_fixture;

#[test]
fn test_function_enclosing_definition() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");
    let increment = counter
        .functions()
        .into_iter()
        .find(|function| {
            function
                .name()
                .is_some_and(|name| name.name() == "increment")
        })
        .expect("can find increment function");

    let Some(Definition::Contract(contract)) = increment.enclosing_definition() else {
        panic!("expected increment's enclosing definition to be a contract");
    };
    assert_eq!(contract.name().name(), "Counter");
}

define_fixture!(
    EnclosingScopes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

struct Top { uint256 x; }

library L {
    struct S { uint256 a; }
    enum E { A, B }
}
"#);

#[test]
fn test_struct_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    // A struct declared inside a library resolves to that library.
    let struct_s = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Struct(struct_definition) if struct_definition.name().name() == "S" => {
                Some(struct_definition)
            }
            _ => None,
        })
        .expect("can find struct S");

    let Some(Definition::Library(library)) = struct_s.enclosing_definition() else {
        panic!("expected struct S's enclosing definition to be a library");
    };
    assert_eq!(library.name().name(), "L");
}

#[test]
fn test_enum_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    let enum_e = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Enum(enum_definition) if enum_definition.name().name() == "E" => {
                Some(enum_definition)
            }
            _ => None,
        })
        .expect("can find enum E");

    let Some(Definition::Library(library)) = enum_e.enclosing_definition() else {
        panic!("expected enum E's enclosing definition to be a library");
    };
    assert_eq!(library.name().name(), "L");
}

#[test]
fn test_struct_member_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    // The immediate enclosing definition of a struct member is the struct, not the library.
    let member = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::StructMember(struct_member) if struct_member.name().name() == "a" => {
                Some(struct_member)
            }
            _ => None,
        })
        .expect("can find struct member a");

    let Some(Definition::Struct(struct_definition)) = member.enclosing_definition() else {
        panic!("expected the struct member's enclosing definition to be the struct");
    };
    assert_eq!(struct_definition.name().name(), "S");
}

#[test]
fn test_file_level_definition_has_no_enclosing() {
    let unit = EnclosingScopes::build_compilation_unit();

    // A file-level struct's container is the source unit.
    let top = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Struct(struct_definition) if struct_definition.name().name() == "Top" => {
                Some(struct_definition)
            }
            _ => None,
        })
        .expect("can find file-level struct Top");

    assert!(
        top.enclosing_definition().is_none(),
        "a file-level struct has no enclosing definition"
    );
}

define_fixture!(
    FunctionBody,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract C {
    function f(uint256 p) public pure returns (uint256) {
        uint256 outer = p;
        {
            uint256 inner = outer;
            return inner;
        }
    }
}
"#);

#[test]
fn test_parameter_enclosing_definition() {
    let unit = FunctionBody::build_compilation_unit();

    let parameter = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Parameter(parameter)
                if parameter.name().is_some_and(|name| name.name() == "p") =>
            {
                Some(parameter)
            }
            _ => None,
        })
        .expect("can find parameter p");

    let Some(Definition::Function(function)) = parameter.enclosing_definition() else {
        panic!("expected parameter p's enclosing definition to be a function");
    };
    assert_eq!(function.name().expect("function is named").name(), "f");
}

#[test]
fn test_local_variable_enclosing_definition() {
    let unit = FunctionBody::build_compilation_unit();

    let outer = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Variable(variable) if variable.name().name() == "outer" => Some(variable),
            _ => None,
        })
        .expect("can find local variable outer");

    let Some(Definition::Function(function)) = outer.enclosing_definition() else {
        panic!("expected local variable outer's enclosing definition to be a function");
    };
    assert_eq!(function.name().expect("function is named").name(), "f");
}

#[test]
fn test_block_nested_variable_enclosing_definition() {
    let unit = FunctionBody::build_compilation_unit();

    let inner = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Variable(variable) if variable.name().name() == "inner" => Some(variable),
            _ => None,
        })
        .expect("can find local variable inner");

    let Some(Definition::Function(function)) = inner.enclosing_definition() else {
        panic!("expected nested local variable inner's enclosing definition to be a function");
    };
    assert_eq!(function.name().expect("function is named").name(), "f");
}

define_fixture!(
    ConstructorBody,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract D {
    constructor(uint256 amount) {
        uint256 local = amount;
    }
}
"#);

#[test]
fn test_constructor_parameter_enclosing_definition() {
    let unit = ConstructorBody::build_compilation_unit();

    let parameter = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Parameter(parameter)
                if parameter.name().is_some_and(|name| name.name() == "amount") =>
            {
                Some(parameter)
            }
            _ => None,
        })
        .expect("can find parameter amount");

    let Some(Definition::Contract(contract)) = parameter.enclosing_definition() else {
        panic!("expected an unnamed constructor's parameter to resolve to the enclosing contract");
    };
    assert_eq!(contract.name().name(), "D");
}
