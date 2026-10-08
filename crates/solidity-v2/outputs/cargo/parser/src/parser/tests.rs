//! Unit tests for the parser, mainly for the `NatSpec` comments attached to the CST.
//!
//! Each test lists the documentable nodes, in source order, with the comment attached to each one.

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;
use slang_solidity_v2_cst::structured_cst::nodes::{
    Block, ContractMember, FunctionBody, SourceUnitMember, Statement,
};
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::{ParseOutput, Parser};

/// The documentable nodes of `source`, in source order, each with the text of the `NatSpec`
/// comment attached to it. A node is described by the first line of its text.
fn documented_nodes(source: &str) -> Vec<(&str, Option<&str>)> {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&"test.sol".into(), source, LanguageVersion::LATEST);
    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let mut nodes = Vec::new();
    let mut push = |node: &dyn TextRange, natspec: &Option<NatSpec>| {
        let range = node.calculate_text_range().expect("a non-empty node");
        let text = source[range].lines().next().unwrap_or_default();
        let natspec = natspec
            .as_ref()
            .map(|natspec| &source[natspec.range.clone()]);
        nodes.push((text, natspec));
    };

    for member in &source_unit.members.elements {
        match member {
            SourceUnitMember::ContractDefinition(contract) => {
                push(contract, &contract.natspec);
                contract_members(&contract.members.elements, &mut push);
            }
            SourceUnitMember::InterfaceDefinition(interface) => {
                push(interface, &interface.natspec);
                contract_members(&interface.members.elements, &mut push);
            }
            SourceUnitMember::LibraryDefinition(library) => {
                push(library, &library.natspec);
                contract_members(&library.members.elements, &mut push);
            }
            SourceUnitMember::FunctionDefinition(function) => {
                push(function, &function.natspec);
                function_body(&function.body, &mut push);
            }
            SourceUnitMember::EventDefinition(event) => push(event, &event.natspec),
            SourceUnitMember::ErrorDefinition(error) => push(error, &error.natspec),
            SourceUnitMember::StructDefinition(node) => push(node, &node.natspec),
            SourceUnitMember::EnumDefinition(node) => push(node, &node.natspec),
            SourceUnitMember::ConstantDefinition(node) => push(node, &node.natspec),
            _ => {}
        }
    }

    nodes
}

type Push<'a> = dyn FnMut(&dyn TextRange, &Option<NatSpec>) + 'a;

fn contract_members(members: &[ContractMember], push: &mut Push<'_>) {
    for member in members {
        match member {
            ContractMember::FunctionDefinition(function) => {
                push(function, &function.natspec);
                function_body(&function.body, push);
            }
            ContractMember::ConstructorDefinition(constructor) => {
                push(constructor, &constructor.natspec);
                block(&constructor.body, push);
            }
            ContractMember::FallbackFunctionDefinition(fallback) => {
                push(fallback, &fallback.natspec);
                function_body(&fallback.body, push);
            }
            ContractMember::ReceiveFunctionDefinition(receive) => {
                push(receive, &receive.natspec);
                function_body(&receive.body, push);
            }
            ContractMember::ModifierDefinition(modifier) => {
                push(modifier, &modifier.natspec);
                function_body(&modifier.body, push);
            }
            ContractMember::StateVariableDefinition(variable) => {
                push(variable, &variable.natspec);
            }
            ContractMember::EventDefinition(event) => push(event, &event.natspec),
            ContractMember::ErrorDefinition(error) => push(error, &error.natspec),
            ContractMember::StructDefinition(node) => push(node, &node.natspec),
            ContractMember::EnumDefinition(node) => push(node, &node.natspec),
            _ => {}
        }
    }
}

fn function_body(body: &FunctionBody, push: &mut Push<'_>) {
    if let FunctionBody::Block(body) = body {
        block(body, push);
    }
}

fn block(block_: &Block, push: &mut Push<'_>) {
    for statement in &block_.statements.elements {
        match statement {
            Statement::AssemblyStatement(assembly) => push(assembly, &assembly.natspec),
            Statement::UncheckedBlock(unchecked) => block(&unchecked.block, push),
            Statement::Block(inner) => block(inner, push),
            Statement::IfStatement(statement) => {
                if let Statement::AssemblyStatement(assembly) = &statement.body {
                    push(assembly, &assembly.natspec);
                }
            }
            _ => {}
        }
    }
}

#[test]
fn declarations_and_statements() {
    // Every kind of documentable node gets the comment right before it
    let source = r#"
/// @title A contract
contract C {
    /// @notice A state variable
    uint public count;

    /** @notice An event */
    event Changed(uint value);

    /// @notice An error
    error Failed();

    /// @notice A struct
    struct S { uint x; }

    /// @notice An enum
    enum E { A }

    /// @notice A constructor
    constructor() {}

    /// @notice A fallback function
    fallback() external {}

    /// @notice A receive function
    receive() external payable {}

    /// @notice A modifier
    modifier m() { _; }

    /// @notice A function
    function f() public {
        /// @solidity memory-safe-assembly
        assembly {}
    }
}

/// @title An interface
interface I {}

/// @title A library
library L {}

/// @dev A constant
uint constant K = 1;
"#;
    assert_eq!(
        documented_nodes(source),
        [
            ("contract C {", Some("/// @title A contract\n")),
            ("uint public count;", Some("/// @notice A state variable\n")),
            (
                "event Changed(uint value);",
                Some("/** @notice An event */")
            ),
            ("error Failed();", Some("/// @notice An error\n")),
            ("struct S { uint x; }", Some("/// @notice A struct\n")),
            ("enum E { A }", Some("/// @notice An enum\n")),
            ("constructor() {}", Some("/// @notice A constructor\n")),
            (
                "fallback() external {}",
                Some("/// @notice A fallback function\n")
            ),
            (
                "receive() external payable {}",
                Some("/// @notice A receive function\n")
            ),
            ("modifier m() { _; }", Some("/// @notice A modifier\n")),
            ("function f() public {", Some("/// @notice A function\n")),
            ("assembly {}", Some("/// @solidity memory-safe-assembly\n")),
            ("interface I {}", Some("/// @title An interface\n")),
            ("library L {}", Some("/// @title A library\n")),
            ("uint constant K = 1;", Some("/// @dev A constant\n")),
        ]
    );
}

#[test]
fn consecutive_lines_are_a_single_comment() {
    let source = "
/// @notice a
    /// @dev b
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice a\n    /// @dev b\n"))]
    );
}

#[test]
fn only_the_last_comment_counts() {
    // A blank line splits `///` lines into two comments, and like in solc, only the last one
    // documents the code
    let source = "
/// @notice a

/// @notice b
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice b\n"))]
    );

    let source = "
/** @notice a */ /// @notice b
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice b\n"))]
    );

    // Even an empty one
    let source = "
/// @notice a
/** */
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/** */"))]
    );
}

#[test]
fn whitespace_and_regular_comments_can_come_in_between() {
    let source = "
/// @notice a

// A regular comment
/* Another one */ //// And another one
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice a\n"))]
    );
}

#[test]
fn code_in_between_detaches_the_comment() {
    // Only the node right after the comment is documented, not the ones after it
    let source = "
contract C {
    /// @notice a
    uint x;
    function f() public {}
}
";
    assert_eq!(
        documented_nodes(source),
        [
            ("contract C {", None),
            ("uint x;", Some("/// @notice a\n")),
            ("function f() public {}", None),
        ]
    );

    // Including an enclosing declaration
    let source = "
/// @notice a
contract C {
    function f() public {}
}
";
    assert_eq!(
        documented_nodes(source),
        [
            ("contract C {", Some("/// @notice a\n")),
            ("function f() public {}", None),
        ]
    );

    // Or an enclosing statement, even one that isn't documentable
    let source = "
function f() {
    /// @notice a
    unchecked { assembly {} }
}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {", None), ("assembly {}", None)]
    );

    // Or a statement before it
    let source = "
function f() {
    /// @notice a
    uint x = 1;
    assembly {}
}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {", None), ("assembly {}", None)]
    );
}

#[test]
fn statements_without_braces() {
    // The comment documents the statement right after it
    let source = "
function f() {
    if (true)
        /// @notice a
        assembly {}
}
";
    assert_eq!(
        documented_nodes(source),
        [
            ("function f() {", None),
            ("assembly {}", Some("/// @notice a\n"))
        ]
    );
}

#[test]
fn comments_inside_a_node_do_not_document_the_next_one() {
    let source = "
function f() {
    /// @notice a
}
function g() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {", None), ("function g() {}", None)]
    );

    let source = "
function f() {
    assembly {
        /// @notice a
    }
    assembly {}
}
";
    assert_eq!(
        documented_nodes(source),
        [
            ("function f() {", None),
            ("assembly {", None),
            ("assembly {}", None),
        ]
    );
}

#[test]
fn comments_inside_an_assembly_statement_do_not_document_it() {
    // They document the token after them instead, which doesn't start the statement
    for (source, assembly) in [
        (
            "
function f() {
    assembly /// @solidity memory-safe-assembly
    {}
}
",
            "assembly /// @solidity memory-safe-assembly",
        ),
        (
            r#"
function f() {
    assembly /// @solidity memory-safe-assembly
    ("memory-safe") {}
}
"#,
            "assembly /// @solidity memory-safe-assembly",
        ),
        (
            r#"
function f() {
    assembly ("memory-safe") /// @solidity memory-safe-assembly
    {}
}
"#,
            r#"assembly ("memory-safe") /// @solidity memory-safe-assembly"#,
        ),
        (
            r#"
function f() {
    assembly "evmasm" /// @solidity memory-safe-assembly
    {}
}
"#,
            r#"assembly "evmasm" /// @solidity memory-safe-assembly"#,
        ),
    ] {
        assert_eq!(
            documented_nodes(source),
            [("function f() {", None), (assembly, None)],
            "{source}"
        );
    }
}

#[test]
fn comments_without_a_node_after_them() {
    let source = "
function f() {}
/// @notice a
";
    assert_eq!(documented_nodes(source), [("function f() {}", None)]);
}

#[test]
fn trailing_empty_line_includes_its_line_break() {
    // Like any other last line, an empty one includes its line break
    let source = "
/// @notice a
///
function f() {}
";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice a\n///\n"))]
    );

    let source = "/// @notice a\r\n///\r\nfunction f() {}\n";
    assert_eq!(
        documented_nodes(source),
        [("function f() {}", Some("/// @notice a\r\n///\r\n"))]
    );
}

#[test]
fn line_endings() {
    // Line endings can't be tested with snapshots, since the repository normalizes them to `\n`
    for line_break in ["\n", "\r\n", "\r"] {
        let source = format!("/// @notice a{line_break}/// @dev b{line_break}function f() {{}}");
        let comment = format!("/// @notice a{line_break}/// @dev b{line_break}");
        assert_eq!(
            documented_nodes(&source),
            [("function f() {}", Some(&comment[..]))],
            "{source:?}"
        );
    }
}
