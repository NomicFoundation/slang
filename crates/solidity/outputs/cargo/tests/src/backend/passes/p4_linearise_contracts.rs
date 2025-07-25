use anyhow::{Ok, Result};
use slang_solidity::backend::passes;
use slang_solidity::compilation::{CompilationUnit, InternalCompilationBuilder};
use slang_solidity::utils::LanguageFacts;

const CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract D is A, B {
}

interface C {
}

abstract contract B is C {
}

interface A is C {
}
"#;

fn build_compilation_unit() -> Result<CompilationUnit> {
    let mut internal_builder = InternalCompilationBuilder::create(LanguageFacts::LATEST_VERSION)?;
    _ = internal_builder.add_file("main.sol".into(), CONTENTS);
    let compilation_unit = internal_builder.build();
    Ok(compilation_unit)
}

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let data = passes::p0_build_ast::run(unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_type_definitions::run(data);
    let data = passes::p4_linearise_contracts::run(data);

    // Create a map relating each contract with its linearised bases (Vec<String>)
    let mut contract_to_bases = std::collections::HashMap::new();
    for (key, values) in data.binder.linearisations() {
        let contract_name = data
            .binder
            .find_definition_by_id(*key)
            .unwrap()
            .identifier()
            .unparse();

        let base_names: Vec<String> = values
            .iter()
            .map(|value| {
                data.binder
                    .find_definition_by_id(*value)
                    .unwrap()
                    .identifier()
                    .unparse()
            })
            .collect();

        contract_to_bases.insert(contract_name, base_names);
    }

    let mut expected = std::collections::HashMap::new();
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
