use anyhow::Result;
use serde_json;
use slang_solidity::backend::ir::ir1_structured_ast;
use slang_solidity::backend::passes::p1_flatten_contracts;
use slang_solidity::parser::Parser;
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_output_ir2_as_json() -> Result<()> {
    let language_version = LanguageFacts::LATEST_VERSION;
    let parser = Parser::create(language_version.clone())?;
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

    let ast = ir1_structured_ast::builder::build_source_unit(output.tree())
        .expect("can build structured AST");
    let source_unit = p1_flatten_contracts::run_file(&language_version, &ast);

    let expected_json = r#"{"members":[{"PragmaDirective":{"pragma":{"VersionPragma":{"sets":[[{"VersionTerm":{"operator":"GreaterThanEqual","literal":{"SimpleVersionLiteral":[{"kind":"VersionSpecifier","text":"0"},{"kind":"VersionSpecifier","text":"8"},{"kind":"VersionSpecifier","text":"17"}]}}}]]}}}},{"ContractDefinition":{"abstract_keyword":false,"name":{"kind":"Identifier","text":"MyContract"},"members":[{"StateVariableDefinition":{"type_name":{"ElementaryType":{"AddressType":{"payable_keyword":false}}},"name":{"kind":"Identifier","text":"owner"},"value":null,"visibility":"Internal","mutability":"Mutable","override_specifier":null}},{"FunctionDefinition":{"parameters":[],"returns":null,"kind":"Constructor","name":null,"body":{"statements":[{"ExpressionStatement":{"expression":{"AssignmentExpression":{"left_operand":{"Identifier":{"kind":"Identifier","text":"owner"}},"operator":{"kind":"Equal","text":"="},"right_operand":{"MemberAccessExpression":{"operand":{"Identifier":{"kind":"Identifier","text":"msg"}},"member":{"kind":"Identifier","text":"sender"}}}}}}}]},"visibility":"Public","mutability":"NonPayable","virtual_keyword":false,"override_specifier":null,"modifier_invocations":[]}},{"FunctionDefinition":{"parameters":[],"returns":[{"type_name":{"ElementaryType":"BoolKeyword"},"storage_location":null,"name":null,"indexed":false}],"kind":"Regular","name":{"kind":"Identifier","text":"test"},"body":{"statements":[{"ReturnStatement":{"expression":{"EqualityExpression":{"left_operand":{"Identifier":{"kind":"Identifier","text":"owner"}},"operator":{"kind":"EqualEqual","text":"=="},"right_operand":{"MemberAccessExpression":{"operand":{"Identifier":{"kind":"Identifier","text":"msg"}},"member":{"kind":"Identifier","text":"sender"}}}}}}}]},"visibility":"Public","mutability":"View","virtual_keyword":false,"override_specifier":null,"modifier_invocations":[]}}],"inheritance_types":[],"storage_layout":null}}]}"#;
    assert_eq!(serde_json::to_string(&source_unit)?, expected_json);

    Ok(())
}
