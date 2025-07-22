use anyhow::{anyhow, Ok, Result};
use slang_solidity::backend::passes;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::backend::passes::compilation_builder::{CompilationBuilder, CompilationBuilderConfig};

const D_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {A} from "a.sol";
import {B} from "b.sol";

contract D is A, B {
}
"#;

const C_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

interface C {
}
"#;

const B_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {C} from "c.sol";

abstract contract B is C {
}
"#;

const A_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

import {C} from "c.sol";

interface A is C {
}
"#;

#[derive(Default)]
struct Config {}

impl CompilationBuilderConfig for Config {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> std::result::Result<Option<String>, Self::Error> {
        match file_id {
            "a.sol" => Ok(Some(A_CONTENTS.to_string())),
            "b.sol" => Ok(Some(B_CONTENTS.to_string())),
            "c.sol" => Ok(Some(C_CONTENTS.to_string())),
            "d.sol" => Ok(Some(D_CONTENTS.to_string())),
            _ => Err(anyhow!("Incorrect file_id: {file_id}")),
        }
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> std::result::Result<Option<String>, Self::Error> {
        let import_string = import_path_cursor.node().unparse();
        let import_string = &import_string[1..import_string.len() - 1];
        Ok(Some(import_string.to_string()))
    }
}

fn build_compilation_unit() -> Result<CompilationUnit> {
    let mut builder = CompilationBuilder::new(LanguageFacts::LATEST_VERSION, Config::default())?;
    builder.add_file("d.sol")?;
    Ok(builder.build())
}

#[test]
fn test_backend_pipeline() -> Result<()> {
    let unit = build_compilation_unit()?;
    let data = passes::p0_build_ast::run(unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_type_definitions::run(data);
    let data = passes::p3_x_flatten_hierarchy::run(data);

    // Create a map relating each contract with its linearised bases (Vec<String>)
    let mut contract_to_bases = std::collections::HashMap::new();
    for (key, values) in &data.linearisations {
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
