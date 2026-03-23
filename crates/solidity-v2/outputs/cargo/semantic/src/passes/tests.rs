use anyhow::{anyhow, Result};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser;

use crate::binder::Binder;
use crate::file::File;
use crate::ir;
use crate::passes::{p2_collect_definitions, p3_linearise_contracts};

#[test]
fn test_collect_definitions_and_linearise_contracts() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let version = LanguageVersion::V0_8_30;
    let source_unit_cst =
        Parser::parse(CONTENTS, version).map_err(|message| anyhow!(format!("{message:?}")))?;
    let source_unit = ir::build(&source_unit_cst, &CONTENTS);

    let file = File::new("test.sol".to_string(), source_unit);

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
