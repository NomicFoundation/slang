use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use anyhow::{bail, Result};
use codegen_language_definition::model::Language;
use infra_utils::{
    codegen::{Codegen, CodegenReadWrite},
    paths::{FileWalker, PathExtensions},
};

pub fn generate_cst_output_tests(
    language: &Language,
    data_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let parser_tests = collect_parser_tests(data_dir)?;

    let mut codegen = Codegen::read_write(data_dir)?;

    generate_mod_file(
        language,
        &mut codegen,
        &output_dir.join("mod.rs"),
        &parser_tests,
    )?;

    for (parser_name, test_names) in &parser_tests {
        generate_unit_test_file(
            &mut codegen,
            parser_name,
            test_names,
            &output_dir.join(format!("{parser_name}.rs")),
        )?;
    }

    Ok(())
}

fn collect_parser_tests(data_dir: &Path) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut parser_tests = BTreeMap::<String, BTreeSet<String>>::new();

    for file in FileWalker::from_directory(data_dir).find_all()? {
        if let Ok(generated_dir) = file.generated_dir() {
            assert!(
                generated_dir.unwrap_parent().join("input.sol").exists(),
                "Each snapshot should have a matching input.sol test file: {file:?}",
            );

            // skip generated files
            continue;
        }

        let parts: Vec<_> = file
            .strip_prefix(data_dir)?
            .iter()
            .map(|p| p.to_str().unwrap())
            .collect();

        match parts[..] {
            [parser_name, test_name, "input.sol"] => {
                parser_tests
                    .entry(parser_name.to_owned())
                    .or_insert_with(BTreeSet::new)
                    .insert(test_name.to_owned());
            }
            _ => {
                bail!("Invalid test input. Should be in the form of '<tests-dir>/PARSER_NAME/TEST_NAME/input.sol', but found: {file:?}");
            }
        };
    }

    Ok(parser_tests)
}

fn generate_mod_file(
    language: &Language,
    codegen: &mut CodegenReadWrite,
    mod_file_path: &Path,
    parser_tests: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let module_declarations = parser_tests
        .keys()
        .map(|parser_name| format!("#[allow(non_snake_case)] mod {parser_name};"))
        .collect::<String>();

    let version_breaks = language.collect_breaking_versions();
    let version_breaks_len = version_breaks.len();
    let version_breaks_str = version_breaks
        .iter()
        .map(|version| {
            format!(
                "Version::new({}, {}, {}),",
                version.major, version.minor, version.patch
            )
        })
        .collect::<String>();

    let contents = format!(
        "
            use semver::Version;
            {module_declarations}

            pub const VERSION_BREAKS: [Version; {version_breaks_len}] = [
                {version_breaks_str}
            ];
        ",
    );

    codegen.write_file(mod_file_path, contents)
}

fn generate_unit_test_file(
    codegen: &mut CodegenReadWrite,
    parser_name: &str,
    test_names: &BTreeSet<String>,
    unit_test_file_path: &Path,
) -> Result<()> {
    let unit_tests = test_names
        .iter()
        .map(|test_name| {
            format!(
                "
                    #[test]
                    fn {test_name}() -> Result<()> {{
                        run(\"{parser_name}\", \"{test_name}\")
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

    codegen.write_file(unit_test_file_path, contents)
}
