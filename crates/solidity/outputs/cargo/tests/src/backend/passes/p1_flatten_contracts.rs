use std::collections::HashMap;
use std::rc::Rc;

use anyhow::Result;
use slang_solidity::backend::ir::ir2_flat_contracts;
use slang_solidity::backend::passes::{p0_build_ast, p1_flatten_contracts};
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig};
use slang_solidity::utils::LanguageFacts;

#[test]
fn test_flatten_contracts() -> Result<()> {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    struct Config {}
    impl CompilationBuilderConfig for Config {
        type Error = ();

        fn read_file(
            &mut self,
            _file_id: &str,
        ) -> std::result::Result<Option<String>, Self::Error> {
            Ok(Some(CONTENTS.to_owned()))
        }

        fn resolve_import(
            &mut self,
            _source_file_id: &str,
            _import_path_cursor: &slang_solidity::cst::Cursor,
        ) -> std::result::Result<Option<String>, Self::Error> {
            panic!("No requires to solve");
        }
    }

    let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Config {})?;
    assert!(builder.add_file("main.sol").is_ok());
    let compilation_unit = Rc::new(builder.build());

    let language_version = compilation_unit.language_version();
    let files = compilation_unit
        .files()
        .iter()
        .filter_map(|file| {
            let file_id = file.id().to_string();
            p0_build_ast::run_file(file)
                .map(|file| p1_flatten_contracts::run_file(language_version, &file))
                .map(|file| (file_id, file))
        })
        .collect::<HashMap<_, _>>();

    let l2 = &files["main.sol"];

    assert_eq!(2, l2.members.len());

    let ir2_flat_contracts::SourceUnitMember::ContractDefinition(base_contract) = &l2.members[0]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Base", base_contract.name.unparse());
    assert!(base_contract.inheritance_types.is_empty());
    assert!(base_contract.storage_layout.is_none());

    let ir2_flat_contracts::SourceUnitMember::ContractDefinition(test_contract) = &l2.members[1]
    else {
        panic!("Expected ContractDefinition");
    };
    assert_eq!("Test", test_contract.name.unparse());
    assert_eq!(1, test_contract.inheritance_types.len());
    assert_eq!(
        "Base",
        test_contract.inheritance_types[0]
            .type_name
            .iter()
            .map(|node| node.unparse())
            .collect::<Vec<_>>()
            .join(".")
    );
    assert!(test_contract.storage_layout.is_some());

    Ok(())
}
