use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use anyhow::{bail, Context, Result};
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;
use walkdir::WalkDir;

pub fn generate_cst_output_tests(
    grammar: &Grammar,
    codegen: &mut CodegenContext,
    data_dir: &PathBuf,
    output_dir: &PathBuf,
) -> Result<()> {
    let mod_file_path = output_dir.join("mod.rs");
    let parser_tests = &collect_parser_tests(codegen, data_dir)?;
    generate_mod_file(grammar, codegen, &mod_file_path, parser_tests)?;

    for (parser_name, test_names) in parser_tests {
        generate_unit_test_file(
            codegen,
            parser_name,
            test_names,
            &output_dir.join(format!("{parser_name}.rs")),
        )?;
    }

    return Ok(());
}

fn collect_parser_tests(
    codegen: &mut CodegenContext,
    data_dir: &PathBuf,
) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut parser_tests = BTreeMap::<String, BTreeSet<String>>::new();

    // Rebuild if input files are added/removed
    codegen.track_input_dir(data_dir);

    let walker = WalkDir::new(data_dir).into_iter().filter_entry(|entry| {
        // skip generated files
        codegen.get_generated_dir(entry.path()).is_err()
    });

    for entry in walker {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }

        let file_path = entry.into_path();
        let parts = file_path
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| Ok(p.to_str().context(format!("Failed to parse part: {p:?}"))?))
            .collect::<Result<Vec<&str>>>()?;

        match &parts[..] {
            [parser_name, test_name, "input.sol"] => {
                let parser_tests = parser_tests
                    .entry(parser_name.to_string())
                    .or_insert_with(|| BTreeSet::new());

                parser_tests.insert(test_name.to_string());
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/PARSER_NAME/TEST_NAME/input.sol', but found: {file_path:?}");
            }
        };
    }

    return Ok(parser_tests);
}

fn generate_mod_file(
    grammar: &Grammar,
    codegen: &mut CodegenContext,
    mod_file_path: &PathBuf,
    parser_tests: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let module_declarations = parser_tests
        .keys()
        .map(|parser_name| format!("#[allow(non_snake_case)] mod {parser_name};"))
        .collect::<String>();

    let version_breaks = grammar.collect_version_breaks();
    let version_breaks_len = version_breaks.len();
    let version_breaks_str = version_breaks
        .iter()
        .map(|version| format!("\"{version}\","))
        .collect::<String>();

    let contents = format!(
        "
            {module_declarations}

            pub const VERSION_BREAKS: [&str; {version_breaks_len}] = [
                {version_breaks_str}
            ];
        ",
    );

    return codegen.write_file(mod_file_path, &contents);
}

fn generate_unit_test_file(
    codegen: &mut CodegenContext,
    parser_name: &str,
    test_names: &BTreeSet<String>,
    unit_test_file_path: &PathBuf,
) -> Result<()> {
    let unit_tests = test_names
        .iter()
        .map(|test_name| {
            format!(
                "
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        return run(\"{parser_name}\", \"{test_name}\");
                    }}
                "
            )
        })
        .collect::<String>();

    let contents = format!(
        "
            use crate::cst_output::runner::run;
            use anyhow::Result;

            {unit_tests}
        "
    );

    return codegen.write_file(unit_test_file_path, &contents);
}
