use std::collections::HashMap;

use anyhow::{Ok, Result};
use slang_solidity::backend::binder::Binder;
use slang_solidity::backend::passes;
use slang_solidity::backend::passes::p3_linearise_contracts::Output;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

fn build_compilation_unit(contents: &str) -> Result<CompilationUnit> {
    struct Config<'a> {
        contents: &'a str,
    }
    impl CompilationBuilderConfig for Config<'_> {
        type Error = anyhow::Error;

        fn read_file(
            &mut self,
            _file_id: &str,
        ) -> std::result::Result<Option<String>, Self::Error> {
            Ok(Some(self.contents.to_owned()))
        }

        fn resolve_import(
            &mut self,
            _source_file_id: &str,
            _import_path_cursor: &slang_solidity::cst::Cursor,
        ) -> std::result::Result<Option<String>, Self::Error> {
            panic!("No requires to solve");
        }
    }

    let mut builder =
        CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Config { contents })?;
    assert!(builder.add_file("main.sol").is_ok());
    let compilation_unit = builder.build();
    Ok(compilation_unit)
}

fn build_linearisation_output(contents: &str) -> Result<Output> {
    let compilation_unit = build_compilation_unit(contents)?;

    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);

    Ok(data)
}

fn get_contract_to_bases_map(binder: &Binder) -> HashMap<String, Vec<String>> {
    // Create a map relating each contract with its linearised bases (Vec<String>)
    let mut contract_to_bases = HashMap::new();
    for (key, values) in binder.linearisations() {
        let contract_name = binder
            .find_definition_by_id(*key)
            .unwrap()
            .identifier()
            .unparse();

        let base_names: Vec<String> = values
            .iter()
            .map(|value| {
                binder
                    .find_definition_by_id(*value)
                    .unwrap()
                    .identifier()
                    .unparse()
            })
            .collect();

        contract_to_bases.insert(contract_name, base_names);
    }

    contract_to_bases
}

const VALID_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract D is A, B {}
interface C {}
abstract contract B is C {}
interface A is C {}
"#;

#[test]
fn test_valid_linearisations() -> Result<()> {
    let data = build_linearisation_output(VALID_CONTENTS)?;

    let contract_to_bases = get_contract_to_bases_map(&data.binder);

    let mut expected = HashMap::new();
    expected.insert(
        "D".to_string(),
        vec![
            "D".to_string(),
            "B".to_string(),
            "A".to_string(),
            "C".to_string(),
        ],
    );
    expected.insert("A".to_string(), vec!["A".to_string(), "C".to_string()]);
    expected.insert("B".to_string(), vec!["B".to_string(), "C".to_string()]);
    expected.insert("C".to_string(), vec!["C".to_string()]);

    assert_eq!(contract_to_bases, expected);

    Ok(())
}

const INVALID_CONTENTS: &str = r#"
contract Base {}

library Foo {}

// Foo is an invalid base, but it shouldn't crash the linearisation pass
contract Test is Base, Foo { // Base should resolve to the contract, not the var
    string Base;
}
"#;

#[test]
fn test_linearise_with_invalid_input() -> Result<()> {
    let data = build_linearisation_output(INVALID_CONTENTS)?;

    let contract_to_bases = get_contract_to_bases_map(&data.binder);

    let mut expected = HashMap::new();
    expected.insert("Base".to_string(), vec!["Base".to_string()]);
    expected.insert(
        "Test".to_string(),
        vec!["Test".to_string(), "Base".to_string()],
    );

    assert_eq!(contract_to_bases, expected);

    Ok(())
}
