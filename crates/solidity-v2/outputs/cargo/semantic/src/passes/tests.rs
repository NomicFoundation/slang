use anyhow::{anyhow, Result};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::binder::Binder;
use crate::file::File;
use crate::ir;
use crate::passes::{p2_collect_definitions, p3_linearise_contracts, p4_type_definitions};
use crate::types::TypeRegistry;

fn build_file(name: &str, contents: &str) -> Result<File> {
    let version = LanguageVersion::V0_8_30;
    let source_unit_cst =
        Parser::parse(contents, version).map_err(|message| anyhow!(format!("{message:?}")))?;
    let source_unit = ir::build(&source_unit_cst, &contents);
    Ok(File::new(name.to_string(), source_unit))
}

#[test]
fn test_collect_definitions_and_linearise_contracts() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let file = build_file("test.sol", CONTENTS)?;

    let files = [file];
    let mut binder = Binder::new();
    p2_collect_definitions::run(&files, &mut binder);
    p3_linearise_contracts::run(&files, &mut binder);

    // Verify definitions were collected
    assert_eq!(2, binder.definitions().len());
    // Verify linearisations were computed
    assert_eq!(2, binder.linearisations().len());

    Ok(())
}

#[test]
fn test_type_definitions() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {
    uint256 public x;
    function foo(uint256 a) public pure returns (uint256) {
        return a;
    }
}

contract Test is Base {
    mapping(address => uint256) balances;
    bool flag;

    struct Point {
        uint256 x;
        uint256 y;
    }

    enum Color { Red, Green, Blue }

    function bar(uint256 b) external view returns (bool) {
        return flag;
    }
}
    "###;

    let file = build_file("test.sol", CONTENTS)?;

    let files = [file];
    let mut binder = Binder::new();
    let mut types = TypeRegistry::default();

    p2_collect_definitions::run(&files, &mut binder);
    p3_linearise_contracts::run(&files, &mut binder);

    let types_before = types.iter_types().count();
    p4_type_definitions::run(&files, &mut binder, &mut types);
    let types_after = types.iter_types().count();

    // The pass registers new types for: contracts, mappings, structs, enums,
    // function types, getter types, etc.
    let registered_types = types_after - types_before;
    assert_eq!(registered_types, 7);

    Ok(())
}
