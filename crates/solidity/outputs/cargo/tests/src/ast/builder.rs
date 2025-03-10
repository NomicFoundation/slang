use anyhow::{anyhow, Result};
use slang_solidity::ast;
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_build_source_unit_ast() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
// SPDX-License-Identifier: UNLICENSED
pragma solidity >= 0.8.17;

contract MyContract {
    address owner;
    constructor() {
         owner = msg.sender;
    }
    function test() view public returns (bool) {
         return owner == msg.sender;
    }
}
    "###,
    );
    assert!(output.is_valid());

    let ast =
        ast::builder::build_source_unit(output.create_tree_cursor()).map_err(|s| anyhow!(s))?;
    assert_eq!(2, ast.members.len());
    assert!(matches!(
        ast.members[0],
        ast::SourceUnitMember::PragmaDirective(_)
    ));
    assert!(matches!(
        ast.members[1],
        ast::SourceUnitMember::ContractDefinition(_)
    ));

    let ast::SourceUnitMember::ContractDefinition(ref contract) = ast.members[1] else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    let ast::ContractMember::StateVariableDefinition(ref state_var) = contract.members[0] else {
        panic!("Expected StateVariableDefinition");
    };
    assert_eq!("owner", state_var.name.unparse());
    assert!(matches!(
        state_var.type_name,
        ast::TypeName::ElementaryType(_)
    ));

    let ast::ContractMember::ConstructorDefinition(ref ctor) = contract.members[1] else {
        panic!("Expected ConstructorDefinition");
    };
    assert_eq!(1, ctor.body.statements.len());

    let ast::ContractMember::FunctionDefinition(ref function) = contract.members[2] else {
        panic!("Expected FunctionDefinition");
    };
    assert_eq!("test", function.name.0.unparse());
    assert_eq!(2, function.attributes.len());
    assert_eq!(0, function.parameters.parameters.len());
    assert!(function.returns.is_some());
    assert_eq!(
        1,
        function
            .returns
            .as_ref()
            .unwrap()
            .variables
            .parameters
            .len()
    );
    assert!(matches!(function.body, ast::FunctionBody::Block(_)));
    let ast::FunctionBody::Block(ref block) = function.body else {
        panic!("Expected Block");
    };
    assert_eq!(1, block.statements.len());

    Ok(())
}
