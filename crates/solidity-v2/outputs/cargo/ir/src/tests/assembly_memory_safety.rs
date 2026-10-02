//! Which assembly statements are memory safe: either with the `("memory-safe")` flag, or with a
//! single `/// @solidity memory-safe-assembly` line.

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;

/// Collects whether each assembly statement is memory safe.
#[derive(Default)]
struct Collector(Vec<bool>);

impl ir::visitor::Visitor for Collector {
    fn enter_assembly_statement(&mut self, node: &ir::AssemblyStatement) -> bool {
        self.0.push(node.is_memory_safe);
        false
    }
}

/// Whether each assembly statement of `source` is memory safe, in source order.
fn memory_safety_at(source: &str, language_version: LanguageVersion) -> Vec<bool> {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&"test.sol".into(), source, language_version);
    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let ir::BuildOutput { ir_root, .. } = ir::build(
        &"test.sol".into(),
        &source_unit,
        &source,
        language_version,
        &mut super::single_file_id_generator(),
    );

    let mut collector = Collector::default();
    ir::visitor::accept_source_unit(&ir_root, &mut collector);
    collector.0
}

fn memory_safety(source: &str) -> Vec<bool> {
    memory_safety_at(source, LanguageVersion::LATEST)
}

/// Wraps `body` in a function.
fn in_function(body: &str) -> String {
    format!(
        "
function f() pure {{
{body}
}}
"
    )
}

#[test]
fn flag() {
    let source = in_function(
        r#"
    assembly ("memory-safe") {}
    assembly {}"#,
    );
    assert_eq!(memory_safety(&source), [true, false]);
}

#[test]
fn natspec() {
    let source = in_function(
        "
    /// @solidity memory-safe-assembly
    assembly {}
    assembly {}",
    );
    assert_eq!(memory_safety(&source), [true, false]);
}

#[test]
fn unrecognized_natspec() {
    let source = in_function(
        "
    /** @solidity memory-safe-assembly */
    assembly {}",
    );
    assert_eq!(memory_safety(&source), [false]);
}

#[test]
fn natspec_and_flag() {
    let source = in_function(
        r#"
    /// @solidity memory-safe-assembly
    assembly "evmasm" ("memory-safe") {}"#,
    );
    assert_eq!(memory_safety(&source), [true]);
}

#[test]
fn natspec_before_0_8_13() {
    // Memory safety was introduced in 0.8.13, together with the flag: before that, the comment has
    // no meaning
    let source = in_function(
        "
    /// @solidity memory-safe-assembly
    assembly {}",
    );
    assert_eq!(memory_safety_at(&source, LanguageVersion::V0_8_12), [false]);
    assert_eq!(memory_safety_at(&source, LanguageVersion::V0_8_13), [true]);
}

#[test]
fn natspec_in_constructors_and_free_functions() {
    // solc's 'memoryGuardTests/comment/safe_and_unmarked_unsafe' and 'free_function'
    let source = "
contract C {
    constructor() {
        /// @solidity memory-safe-assembly
        assembly { mstore(0, 0) }
        assembly { mstore(0, 0) }
    }
}
function safe() pure returns (uint256 x) {
    assembly { x := 42 }
    /// @solidity memory-safe-assembly
    assembly { mstore(0, 0) }
}
";
    assert_eq!(memory_safety(source), [true, false, false, true]);
}
