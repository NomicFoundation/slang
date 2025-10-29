use anyhow::Result;
use semver::Version;
use slang_solidity::backend::ir::ir1_structured_ast;
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

    let ast = ir1_structured_ast::builder::build_source_unit(&output.syntax_tree()).unwrap();
    assert_eq!(2, ast.members.len());
    assert!(matches!(
        ast.members[0],
        ir1_structured_ast::SourceUnitMember::PragmaDirective(_)
    ));
    assert!(matches!(
        ast.members[1],
        ir1_structured_ast::SourceUnitMember::ContractDefinition(_)
    ));

    let ir1_structured_ast::SourceUnitMember::ContractDefinition(ref contract) = ast.members[1]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("MyContract", contract.name.unparse());
    assert_eq!(3, contract.members.len());

    let ir1_structured_ast::ContractMember::StateVariableDefinition(ref state_var) =
        contract.members[0]
    else {
        panic!("Expected StateVariableDefinition");
    };
    assert_eq!("owner", state_var.name.unparse());
    assert!(matches!(
        state_var.type_name,
        ir1_structured_ast::TypeName::ElementaryType(_)
    ));

    let ir1_structured_ast::ContractMember::ConstructorDefinition(ref ctor) = contract.members[1]
    else {
        panic!("Expected ConstructorDefinition");
    };
    assert_eq!(1, ctor.body.statements.len());

    let ir1_structured_ast::ContractMember::FunctionDefinition(ref function) = contract.members[2]
    else {
        panic!("Expected FunctionDefinition");
    };
    let ir1_structured_ast::FunctionName::Identifier(ref name) = function.name else {
        panic!("Expected identifier in FunctionName");
    };
    assert_eq!("test", name.unparse());
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
    assert!(matches!(
        function.body,
        ir1_structured_ast::FunctionBody::Block(_)
    ));
    let ir1_structured_ast::FunctionBody::Block(ref block) = function.body else {
        panic!("Expected Block");
    };
    assert_eq!(1, block.statements.len());

    Ok(())
}

#[test]
fn test_can_handle_unrecognized() -> Result<()> {
    let parser = Parser::create(Version::parse("0.4.11")?)?;
    let output = parser.parse_file_contents(
        r###"
contract Test {
    function test(address payable to) public {}
}
    "###,
    );
    assert!(!output.is_valid());
    assert_eq!(output.errors().len(), 1);

    let ast = ir1_structured_ast::builder::build_source_unit(&output.syntax_tree()).unwrap();
    assert_eq!(1, ast.members.len());
    assert!(matches!(
        ast.members[0],
        ir1_structured_ast::SourceUnitMember::ContractDefinition(_)
    ));

    let ir1_structured_ast::SourceUnitMember::ContractDefinition(ref contract) = ast.members[0]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!(1, contract.members.len());
    assert!(matches!(
        contract.members[0],
        ir1_structured_ast::ContractMember::FunctionDefinition(_)
    ));

    let ir1_structured_ast::ContractMember::FunctionDefinition(ref function) = contract.members[0]
    else {
        panic!("Expected FunctionDefinition");
    };
    assert_eq!(1, function.parameters.parameters.len());

    let parameter = &function.parameters.parameters[0];
    // parameter name cannot be parsed
    assert!(parameter.name.is_none());

    // but the parameter type can
    let ir1_structured_ast::TypeName::ElementaryType(ref parameter_type) = parameter.type_name
    else {
        panic!("Expected ElementaryType");
    };
    let ir1_structured_ast::ElementaryType::AddressType(ref address_type) = parameter_type else {
        panic!("Expected AddressType");
    };
    assert!(!address_type.payable_keyword);

    Ok(())
}

#[test]
fn test_can_handle_missing() -> Result<()> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION)?;
    let output = parser.parse_file_contents(
        r###"
contract Test {
    "###,
    );
    assert!(!output.is_valid());
    assert_eq!(output.errors().len(), 1);

    let ast = ir1_structured_ast::builder::build_source_unit(&output.syntax_tree()).unwrap();
    // the contract definition cannot be parsed fully
    assert_eq!(0, ast.members.len());

    Ok(())
}
